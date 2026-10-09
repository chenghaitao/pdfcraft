<#
.SYNOPSIS
  Build PdfCraft for Windows locally with one command: the MSI installer and the portable zip.

.DESCRIPTION
  packaging/windows/package.ps1 is the script CI runs; it assumes the machine is already set up.
  This is the local front door for it: it checks the toolchain, installs WiX v5 into the user
  profile when it is missing, wires up a craft-fonts checkout when there is one, then hands over
  to package.ps1 and lists what came out.

  Produces, in $env:DIST (default: dist\release):
    pdfcraft-<version>-windows-<arch>.msi            per-machine installer       (needs WiX)
    pdfcraft-<version>-windows-<arch>-portable.zip   unzip and run, no install   (no WiX)

  Succeeds without code-signing secrets: sign.ps1 warns and leaves the artifacts unsigned, which
  is what a local build wants. Set WINDOWS_CERTIFICATE (+ password) or the AZURE_* variables to
  sign as a release does.

  Works on Windows PowerShell 5.1 (the one every Windows has); it hands package.ps1 to pwsh when
  pwsh is present, so a local run packages exactly like CI.

  Needs: Rust with the MSVC toolchain and the target, the Windows SDK (rc.exe), and .NET for WiX.
        rustup target add x86_64-pc-windows-msvc
        dotnet tool install --global wix --version 5.0.2   (this script does it for you)

.EXAMPLE
  powershell -ExecutionPolicy Bypass -File packaging\windows\build.ps1
  powershell -ExecutionPolicy Bypass -File packaging\windows\build.ps1 -Check
  powershell -ExecutionPolicy Bypass -File packaging\windows\build.ps1 -PortableOnly
  powershell -ExecutionPolicy Bypass -File packaging\windows\build.ps1 -SkipBuild -PortableDir
#>
[CmdletBinding()]
param(
  # Target architecture. Only x64 is installed on a stock MSVC toolchain; see the header.
  [ValidateSet('x64', 'x86', 'arm64')] [string] $Arch = 'x64',
  # Skip the MSI and build just the portable zip: no WiX, no .NET needed.
  [switch] $PortableOnly,
  # Reuse the binaries already in target\<target>\release instead of rebuilding them.
  [switch] $SkipBuild,
  # Also leave the portable build unpacked in $env:DIST, ready to copy and run.
  [switch] $PortableDir,
  # Report the toolchain and what would be built, then exit without building anything.
  [switch] $Check,
  # Never install anything: fail when WiX v5 is missing instead of installing it.
  [switch] $NoInstall
)
$ErrorActionPreference = 'Stop'
$Root = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
# Keep artefacts of a local run together; package.ps1 honours the same variable.
$Dist = if ($env:DIST) { $env:DIST } else { Join-Path $Root 'dist\release' }
$WixVersion = '5.0.2'
$ToolsDir = Join-Path $env:USERPROFILE '.dotnet\tools'

function Write-Step([string] $Message) {
  Write-Output "==> $Message"
}

# Windows PowerShell 5.1 turns anything a native command writes to stderr into a terminating
# error while $ErrorActionPreference is 'Stop', and cargo, wix, dotnet and signtool all write
# progress there ("Blocking waiting for file lock", "Compiling ..."). Run them with the
# preference relaxed and trust the exit code instead.
function Invoke-Native([string] $What, [scriptblock] $Block) {
  Write-Step $What
  $previous = $ErrorActionPreference
  $ErrorActionPreference = 'Continue'
  try { & $Block } finally { $ErrorActionPreference = $previous }
  if ($LASTEXITCODE -ne 0) { throw "$What failed with exit code $LASTEXITCODE" }
}

function Get-Tool([string] $Name) {
  $command = Get-Command $Name -ErrorAction SilentlyContinue
  if ($command) { return $command.Source }
  return $null
}

# pwsh 7 packages exactly like CI; Windows PowerShell is the fallback every machine has. The
# Microsoft Store alias exists as a file even before the app is installed, so try it once.
function Get-PowerShellHost {
  $pwsh = Get-Tool 'pwsh'
  if ($pwsh) {
    $previous = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
      & $pwsh -NoProfile -NonInteractive -Command 'exit 0' 2>$null | Out-Null
      if ($LASTEXITCODE -eq 0) { return $pwsh }
    }
    catch { }
    finally { $ErrorActionPreference = $previous }
  }
  return (Get-Tool 'powershell')
}

# The version lives in one place: [workspace.package] version in the root Cargo.toml.
function Get-WorkspaceVersion {
  $inPackage = $false
  foreach ($line in Get-Content (Join-Path $Root 'Cargo.toml')) {
    if ($line -match '^\s*\[') { $inPackage = ($line.Trim() -eq '[workspace.package]'); continue }
    if ($inPackage -and $line -match '^\s*version\s*=\s*"([^"]+)"') { return $Matches[1] }
  }
  throw 'could not read [workspace.package] version from Cargo.toml'
}

$Target = @{ x64 = 'x86_64-pc-windows-msvc'; x86 = 'i686-pc-windows-msvc'; arm64 = 'aarch64-pc-windows-msvc' }[$Arch]
$Version = Get-WorkspaceVersion
$TargetDir = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $Root 'target' }
$Bin = Join-Path $TargetDir "$Target\release"

# A craft-fonts checkout is an optional build input; with one, the interface has real Japanese,
# Chinese and Arabic glyphs instead of the font system's replacement ones. A checkout at
# <root>\craft-fonts (git-ignored) is picked up automatically.
$FontsDir = Join-Path $Root 'craft-fonts'
$HaveFonts = (Test-Path $FontsDir) -and (Test-Path (Join-Path $FontsDir 'fonts'))
if ($HaveFonts -and -not $env:CRAFT_FONTS_DIR) { $env:CRAFT_FONTS_DIR = $FontsDir }

# WiX v5 is a dotnet tool and installs into the user profile, which is on PATH only in shells
# started after the install; put it there ourselves so a first run works in the same session.
if (Test-Path $ToolsDir) { $env:PATH = "$ToolsDir;$env:PATH" }
$Wix = Get-Tool 'wix'
$Pwsh = Get-PowerShellHost
$Cargo = Get-Tool 'cargo'
$Rustup = Get-Tool 'rustup'
$Git = Get-Tool 'git'
$Targets = @()
if ($Rustup) { $Targets = @(& rustup target list --installed 2>$null) }
# package.ps1 is written for pwsh; under Windows PowerShell 5.1 it can stop on cargo's own stderr.
$PwshNote = ''
if ($Pwsh -and ([IO.Path]::GetFileName($Pwsh) -ieq 'powershell.exe')) {
  $PwshNote = '  (PowerShell 5.1; install PowerShell 7 for the packaging CI uses)'
}

Write-Output "PdfCraft $Version for Windows $Arch ($Target)"
Write-Output ''
foreach ($row in @(
    @('PowerShell', $(if ($Pwsh) { "$Pwsh$PwshNote" } else { 'not found' })),
    @('cargo', $(if ($Cargo) { (& cargo --version 2>&1 | Out-String).Trim() } else { 'not found' })),
    @('rust target', $(if ($Targets -contains $Target) { "$Target installed" } else { "MISSING - rustup target add $Target" })),
    @('git', $(if ($Git) { 'found' } else { 'not found (build SHA is left empty)' })),
    @('dotnet', $(if (Get-Tool 'dotnet') { (& dotnet --version 2>&1 | Out-String).Trim() } else { 'not found' })),
    @('WiX v5', $(if ($Wix) { $Wix } elseif ($PortableOnly) { 'not needed (-PortableOnly)' } else { "not installed (will install $WixVersion)" })),
    @('signtool', $(if (Get-Tool 'signtool.exe') { 'on PATH' } else { 'the Windows SDK one' })),
    @('signing', $(if ($env:WINDOWS_CERTIFICATE -or $env:AZURE_SIGNING_ACCOUNT) { 'configured' } else { 'not configured - artifacts stay unsigned' })),
    @('craft-fonts', $(if ($HaveFonts) { $FontsDir } else { 'no checkout - CJK interface text shows replacement glyphs' })),
    @('output', $Dist)
  )) {
  Write-Output ("  {0,-14} {1}" -f $row[0], $row[1])
}
Write-Output ''

if (-not $Cargo) { throw 'cargo is not on PATH: install Rust from https://rustup.rs' }
if (-not ($Targets -contains $Target)) { throw "the $Target target is missing: rustup target add $Target" }

if ($Check) {
  Write-Output 'Check only: nothing was built. Drop -Check to build.'
  exit 0
}

# ---- WiX (the MSI needs it; the portable zip does not) ------------------------------------------
if (-not $PortableOnly -and -not $Wix) {
  if ($NoInstall) { throw "WiX v5 is not installed: dotnet tool install --global wix --version $WixVersion" }
  if (-not (Get-Tool 'dotnet')) { throw "WiX v5 is not installed and dotnet is not either: install the .NET SDK, or build just the portable zip with -PortableOnly" }
  # The first install often dies on "unable to load the service index for ... api.nuget.org" and
  # succeeds on a retry, so don't give up on one attempt.
  $installed = $false
  $previous = $ErrorActionPreference
  $ErrorActionPreference = 'Continue'
  try {
    foreach ($attempt in 1..3) {
      Write-Step "dotnet tool install --global wix --version $WixVersion (attempt $attempt of 3)"
      & dotnet tool install --global wix --version $WixVersion
      if ($LASTEXITCODE -eq 0) { $installed = $true; break }
      if ($attempt -lt 3) {
        Write-Output '    retrying in 5 s'
        Start-Sleep -Seconds 5
      }
    }
  }
  finally { $ErrorActionPreference = $previous }
  if (-not $installed) {
    throw "could not install WiX v5. Check that https://api.nuget.org/v3/index.json is reachable (a mirror works: dotnet tool install --global wix --version $WixVersion --add-source https://repo.huaweicloud.com/repository/nuget/v3/index.json), or build just the portable zip with -PortableOnly"
  }
  $env:PATH = "$ToolsDir;$env:PATH"
  $Wix = Get-Tool 'wix'
  if (-not $Wix) { throw "wix is still not on PATH after installing it; open a new terminal, or add $ToolsDir to PATH" }
}

New-Item -ItemType Directory -Force -Path $Dist | Out-Null

# The MSI wants a numeric version (1.2.3), a portable zip is happy with anything; keep the build
# SHA and date in the binaries' VERSIONINFO the way package.ps1 does.
if (-not $env:PDFCRAFT_BUILD_SHA -and $Git) { $env:PDFCRAFT_BUILD_SHA = (& git -C $Root rev-parse HEAD 2>$null) }
if (-not $env:PDFCRAFT_BUILD_DATE) { $env:PDFCRAFT_BUILD_DATE = (Get-Date).ToUniversalTime().ToString('yyyy-MM-dd') }

# Static CRT (the artefacts need no Visual C++ redistributable) and a hard failure if the icon or
# VERSIONINFO cannot be embedded. package.ps1 sets both itself; set them here so -PortableOnly
# builds the same way.
$flagVar = 'CARGO_TARGET_' + ($Target.ToUpper() -replace '-', '_') + '_RUSTFLAGS'
[Environment]::SetEnvironmentVariable($flagVar, '-C target-feature=+crt-static')
$env:PDFCRAFT_REQUIRE_WINRES = '1'

if (-not $SkipBuild) {
  Invoke-Native "cargo build --release -p pdfcraft -p pdfcraft-cli ($Target)" {
    cargo build --release --locked -p pdfcraft -p pdfcraft-cli --target $Target
  }
}
else {
  Write-Step "skipping the build (-SkipBuild); using $Bin"
}
foreach ($exe in 'pdfcraft.exe', 'pdfcraft-cli.exe') {
  if (-not (Test-Path (Join-Path $Bin $exe))) { throw "$exe is not in $Bin`: drop -SkipBuild, or build the $Target target first" }
}

if (-not $PortableOnly) {
  # package.ps1 does the rest: MSI, signing, portable zip, and its own checks.
  $packageArgs = @('-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', (Join-Path $PSScriptRoot 'package.ps1'), '-Arch', $Arch)
  if ($SkipBuild) { $packageArgs += '-SkipBuild' }
  Invoke-Native "$Pwsh $($packageArgs -join ' ')" { & $Pwsh @packageArgs }
}
else {
  # ---- portable zip, without WiX --------------------------------------------------------------
  Invoke-Native 'pdfcraft-cli --version' { & (Join-Path $Bin 'pdfcraft-cli.exe') --version }

  $Portable = Join-Path $TargetDir "windows-package\pdfcraft-$Version-windows-$Arch-portable"
  Remove-Item -Recurse -Force $Portable -ErrorAction SilentlyContinue
  New-Item -ItemType Directory -Force -Path $Portable | Out-Null
  Copy-Item (Join-Path $Bin 'pdfcraft.exe'), (Join-Path $Bin 'pdfcraft-cli.exe') $Portable
  foreach ($file in 'README.md', 'LICENSE', 'LICENSE-MIT', 'LICENSE-APACHE') {
    $path = Join-Path $Root $file
    if (Test-Path $path) { Copy-Item $path $Portable }
  }
  # Fonts embedded from craft-fonts carry their own licences: fonts\<family>\OFL.txt -> OFL-<family>.txt.
  if ($env:CRAFT_FONTS_DIR) {
    Get-ChildItem -Path (Join-Path $env:CRAFT_FONTS_DIR 'fonts') -Directory -ErrorAction SilentlyContinue | ForEach-Object {
      $ofl = Join-Path $_.FullName 'OFL.txt'
      if (Test-Path $ofl) { Copy-Item $ofl (Join-Path $Portable "OFL-$($_.Name).txt") }
    }
  }
  # portable.txt beside pdfcraft.exe switches on portable mode: settings, logs, recovery files and
  # new digital IDs go to PdfCraftData\ next to the exe instead of %APPDATA%.
  Copy-Item (Join-Path $PSScriptRoot 'portable.txt') $Portable
  $Zip = Join-Path $Dist "pdfcraft-$Version-windows-$Arch-portable.zip"
  Remove-Item -Force $Zip -ErrorAction SilentlyContinue
  Write-Step "Compress-Archive $Zip"
  Compress-Archive -Path $Portable -DestinationPath $Zip
}

if ($PortableDir) {
  $Zip = Join-Path $Dist "pdfcraft-$Version-windows-$Arch-portable.zip"
  $Unpacked = Join-Path $Dist "pdfcraft-$Version-windows-$Arch-portable"
  Write-Step "unpacking the portable zip into $Unpacked"
  Remove-Item -Recurse -Force $Unpacked -ErrorAction SilentlyContinue
  Expand-Archive -Path $Zip -DestinationPath $Dist -Force
}

# ---- what came out ------------------------------------------------------------------------------
$produced = @(Get-ChildItem -Path $Dist -Filter "pdfcraft-$Version-windows-$Arch*" -ErrorAction SilentlyContinue |
    Where-Object { -not $_.PSIsContainer } | Sort-Object Name)
Write-Output ''
if ($produced.Count -eq 0) { throw "no artefacts in $Dist" }
Write-Output "Built into $Dist`:"
foreach ($file in $produced) {
  Write-Output ("  {0,-52} {1,8:N1} MB" -f $file.Name, ($file.Length / 1MB))
}
