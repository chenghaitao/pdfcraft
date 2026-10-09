//! The last-resort interface fonts: the faces already installed on this machine.
//!
//! The embedded faces (Inter, egui's defaults, craft-fonts) come first in every family; these
//! only draw characters none of them has — every Chinese character in a build without
//! craft-fonts, an Arabic file name, a Telugu one. They are read at runtime and never embedded
//! or shipped (AGENTS.md §1.4), and `PDFCRAFT_SYSTEM_FONTS=0` turns them off (published
//! screenshots do).

use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use egui::FontData;

/// Larger files are not read: a font path is still untrusted input.
const MAX_BYTES: u64 = 32 << 20;
/// Faces tried in a collection (`.ttc`).
const MAX_FACES: u32 = 16;
/// Script groups a face is read for; `Script` is usable as an index into a three-slot cache.
const SCRIPTS: usize = 3;

/// A script group the interface falls back to an installed face for.
///
/// One face per group, not one in total: the Han glyphs of a Chinese menu and the Telugu of a
/// Telugu file name never come from the same installed file, and a single face would leave one
/// of the two as boxes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Script {
    /// Han and kana: the craft-fonts `Hans`/`Jpan` faces. The heaviest group (17–22 MB here).
    Cjk,
    /// Arabic script: the craft-fonts `Arab` faces — file names, document titles.
    Arabic,
    /// Telugu: the craft-fonts `Telu` faces — the Telugu catalog, file names.
    Telugu,
}

impl Script {
    /// A character the face must map to be worth loading for this group.
    fn probe(self) -> char {
        match self {
            Script::Cjk => '\u{4E2D}',    // 中
            Script::Arabic => '\u{0627}', // ا
            Script::Telugu => '\u{0C24}', // త
        }
    }

    /// Well-known installed files with broad coverage of this group, best first.
    fn candidates(self) -> Vec<PathBuf> {
        if cfg!(windows) {
            let dir =
                std::env::var_os("WINDIR").or_else(|| std::env::var_os("SystemRoot")).map_or_else(|| PathBuf::from(r"C:\Windows"), PathBuf::from);
            // Microsoft YaHei is the Windows UI face and covers Simplified, Traditional and
            // kana; SimSun and SimHei cover the same range where YaHei is absent.
            let files: &[&str] = match self {
                Script::Cjk => &["msyh.ttc", "msyh.ttf", "simsun.ttc", "simhei.ttf", "mingliu.ttc"],
                Script::Arabic => &["segoeui.ttf", "tahoma.ttf", "arial.ttf"],
                // Nirmala UI is Windows' Indic face; it ships with the language pack.
                Script::Telugu => &["Nirmala.ttf", "nirmala.ttf"],
            };
            files.iter().map(|f| dir.join("Fonts").join(f)).collect()
        } else if cfg!(target_os = "macos") {
            let files: &[&str] = match self {
                Script::Cjk => &["/System/Library/Fonts/PingFang.ttc", "/Library/Fonts/Arial Unicode.ttf"],
                Script::Arabic => {
                    &["/System/Library/Fonts/SFArabic.ttf", "/System/Library/Fonts/GeezaPro.ttc", "/System/Library/Fonts/Supplemental/Arial.ttf"]
                }
                Script::Telugu => &["/System/Library/Fonts/KohinoorTelugu.ttc", "/System/Library/Fonts/Supplemental/Telugu MN.ttc"],
            };
            files.iter().map(PathBuf::from).collect()
        } else {
            let files: &[&str] = match self {
                Script::Cjk => &[
                    "opentype/noto/NotoSansCJK-Regular.ttc",
                    "noto-cjk/NotoSansCJK-Regular.ttc",
                    "truetype/noto/NotoSansCJK-Regular.ttc",
                    "truetype/wqy/wqy-zenhei.ttc",
                    "wqy-zenhei/wqy-zenhei.ttc",
                    "truetype/wqy/wqy-microhei.ttc",
                    "wqy-microhei/wqy-microhei.ttc",
                ],
                Script::Arabic => &[
                    "truetype/noto/NotoSansArabic-Regular.ttf",
                    "noto/NotoSansArabic-Regular.ttf",
                    "google-noto/NotoSansArabic-Regular.ttf",
                    "truetype/dejavu/DejaVuSans.ttf",
                    "TTF/DejaVuSans.ttf",
                    "dejavu/DejaVuSans.ttf",
                    "dejavu-sans-fonts/DejaVuSans.ttf",
                ],
                Script::Telugu => &[
                    "truetype/noto/NotoSansTelugu-Regular.ttf",
                    "noto/NotoSansTelugu-Regular.ttf",
                    "lohit-telugu/Lohit-Telugu.ttf",
                    "truetype/lohit-telugu/Lohit-Telugu.ttf",
                ],
            };
            ["/usr/share/fonts", "/usr/local/share/fonts"].iter().flat_map(|dir| files.iter().map(move |f| Path::new(dir).join(f))).collect()
        }
    }
}

/// The installed face for `script`, read once. `None` when it is turned off or no candidate fits.
pub fn fallback(script: Script) -> Option<Arc<FontData>> {
    static CACHE: [OnceLock<Option<Arc<FontData>>>; SCRIPTS] = [const { OnceLock::new() }; SCRIPTS];
    CACHE[script as usize].get_or_init(|| load(script)).clone()
}

fn load(script: Script) -> Option<Arc<FontData>> {
    if std::env::var_os("PDFCRAFT_SYSTEM_FONTS").is_some_and(|v| v == "0") {
        return None;
    }
    let probe = script.probe();
    script.candidates().iter().find_map(|path| read(path, probe))
}

fn read(path: &Path, probe: char) -> Option<Arc<FontData>> {
    let meta = std::fs::metadata(path).ok()?;
    if !meta.is_file() || meta.len() > MAX_BYTES {
        return None;
    }
    let bytes = std::fs::read(path).ok()?;
    let index = face_with(&bytes, probe)?;
    let mut data = FontData::from_owned(bytes);
    data.index = index;
    log::info!("interface font fallback: {} (face {index})", path.display());
    Some(Arc::new(data))
}

/// The first face of the file that parses and maps `c`. egui parses fonts with the same skrifa,
/// so a face accepted here is one it can load.
fn face_with(bytes: &[u8], c: char) -> Option<u32> {
    use skrifa::MetadataProvider as _;
    (0..MAX_FACES).find(|&index| skrifa::FontRef::from_index(bytes, index).is_ok_and(|font| font.charmap().map(c).is_some()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn broken_and_missing_files_are_skipped() {
        let probe = Script::Arabic.probe();
        assert_eq!(face_with(b"", probe), None);
        assert_eq!(face_with(b"not a font at all", probe), None);
        assert_eq!(face_with(&[0u8; 4096], probe), None);
        assert!(read(Path::new("definitely/not/here.ttf"), probe).is_none());
        // A directory is not a font.
        assert!(read(&std::env::temp_dir(), probe).is_none());
    }

    #[test]
    fn a_face_without_the_probe_is_rejected() {
        // Inter is Latin, Greek and Cyrillic only.
        let inter = include_bytes!("../../../assets/fonts/Inter-Regular.ttf");
        for script in [Script::Cjk, Script::Arabic, Script::Telugu] {
            assert_eq!(face_with(inter, script.probe()), None, "{script:?}");
        }
        assert_eq!(face_with(inter, 'A'), Some(0));
    }

    #[test]
    fn candidates_are_absolute_font_files() {
        for script in [Script::Cjk, Script::Arabic, Script::Telugu] {
            let list = script.candidates();
            assert!(!list.is_empty(), "{script:?}");
            assert!(list.iter().all(|p| p.extension().is_some_and(|e| e == "ttf" || e == "ttc")), "{script:?}");
        }
    }

    /// The Chinese face comes before the others on Windows: a Simplified Chinese interface in a
    /// build without craft-fonts is drawn by Microsoft YaHei, not by a Latin-only face.
    #[test]
    #[cfg(windows)]
    fn the_windows_chinese_face_leads_the_cjk_candidates() {
        let list = Script::Cjk.candidates();
        assert_eq!(list.first().and_then(|p| p.file_name()).and_then(|n| n.to_str()), Some("msyh.ttc"), "{list:?}");
        // SimSun/SimHei are only reached when it is absent, so they must be behind it.
        let simsun = list.iter().position(|p| p.ends_with("simsun.ttc")).expect("simsun is a fallback");
        assert!(simsun > 0, "{list:?}");
    }

    /// The face is not just named: the file is read and a Han glyph comes out of it. This is the
    /// whole point of the module — without craft-fonts this face is what draws a Chinese menu
    /// instead of egui's replacement box.
    #[test]
    #[cfg(windows)]
    fn the_windows_chinese_face_loads_and_has_han_glyphs() {
        if std::env::var_os("PDFCRAFT_SYSTEM_FONTS").is_some_and(|v| v == "0") {
            eprintln!("skipping the_windows_chinese_face_loads_and_has_han_glyphs: PDFCRAFT_SYSTEM_FONTS=0");
            return;
        }
        let face = fallback(Script::Cjk).expect("a Chinese face is installed on Windows");
        assert!(face.index < MAX_FACES, "{}", face.index);
        // The probe is a Han character, so a loaded face maps it by construction: 中 (U+4E2D).
        assert_eq!(Script::Cjk.probe(), '\u{4E2D}');
    }
}
