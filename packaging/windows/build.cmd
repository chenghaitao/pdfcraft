@echo off
rem Double-click entry point for build.ps1 (run `build.cmd -Check` to see the toolchain it finds).
rem Produces the MSI installer and the portable zip in dist\release.
setlocal
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0build.ps1" %*
set "code=%ERRORLEVEL%"
echo.
if "%code%"=="0" (echo Build finished.) else (echo Build failed with exit code %code%.)
echo Artifacts are in dist\release.
pause
exit /b %code%
