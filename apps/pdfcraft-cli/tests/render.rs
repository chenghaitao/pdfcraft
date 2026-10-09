//! `render --out` writes the file format its name says (#248): a `.png` is a PNG, not a netpbm
//! PAM stream with a `.png` extension.

use std::path::PathBuf;
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_pdfcraft-cli");

fn tmp(test: &str, name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("pdfcraft-cli-render-{test}-{}", std::process::id()));
    std::fs::create_dir_all(&d).unwrap();
    d.join(name)
}

/// A one-page PDF with some text, as `--out` would be given it.
fn fixture(test: &str) -> PathBuf {
    let pdf = tmp(test, "in.pdf");
    let bytes = pdfcraft_engine::Session::new().create_from_text("Render test", "Hello from PdfCraft").unwrap();
    std::fs::write(&pdf, bytes.as_slice()).unwrap();
    pdf
}

fn render(pdf: &std::path::Path, out: &std::path::Path) -> Output {
    Command::new(BIN).args(["render", pdf.to_str().unwrap(), "--page", "1", "--dpi", "36", "--out", out.to_str().unwrap()]).output().unwrap()
}

fn be32(b: &[u8]) -> u32 {
    u32::from_be_bytes([b[0], b[1], b[2], b[3]])
}

#[test]
fn a_png_name_gets_a_png() {
    let pdf = fixture("png");
    let out = tmp("png", "p1.png");
    let r = render(&pdf, &out);
    assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
    let bytes = std::fs::read(&out).unwrap();
    assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n", "not a PNG: {:?}", &bytes[..bytes.len().min(16)]);
    // IHDR: the page is 612×792 pt, rendered at 36 dpi
    assert_eq!(&bytes[12..16], b"IHDR");
    assert_eq!((be32(&bytes[16..20]), be32(&bytes[20..24])), (306, 396));
    // the reported size matches the file
    let stderr = String::from_utf8_lossy(&r.stderr);
    assert!(stderr.contains("306×396"), "{stderr}");
}

#[test]
fn jpeg_and_tiff_names_get_those_formats() {
    let pdf = fixture("jpg");
    let jpg = tmp("jpg", "p1.JPG");
    assert!(render(&pdf, &jpg).status.success());
    assert_eq!(&std::fs::read(&jpg).unwrap()[..2], b"\xff\xd8");
    let tif = tmp("jpg", "p1.tif");
    assert!(render(&pdf, &tif).status.success());
    let bytes = std::fs::read(&tif).unwrap();
    assert!(bytes.starts_with(b"II*\0") || bytes.starts_with(b"MM\0*"), "not a TIFF: {:?}", &bytes[..4]);
}

#[test]
fn a_pam_name_still_gets_the_netpbm_stream() {
    let pdf = fixture("pam");
    let out = tmp("pam", "p1.pam");
    assert!(render(&pdf, &out).status.success());
    let bytes = std::fs::read(&out).unwrap();
    let header = b"P7\nWIDTH 306\nHEIGHT 396\nDEPTH 4\nMAXVAL 255\nTUPLTYPE RGB_ALPHA\nENDHDR\n";
    assert!(bytes.starts_with(header), "{:?}", String::from_utf8_lossy(&bytes[..bytes.len().min(80)]));
    assert_eq!(bytes.len(), header.len() + 306 * 396 * 4);
}

#[test]
fn an_unknown_extension_fails_and_writes_nothing() {
    let pdf = fixture("bmp");
    let out = tmp("bmp", "p1.bmp");
    let r = render(&pdf, &out);
    assert!(!r.status.success());
    let stderr = String::from_utf8_lossy(&r.stderr);
    assert!(stderr.contains(".png, .jpg, .tif or .pam"), "{stderr}");
    assert!(!out.exists());
}

/// A 1000×20 pt page with a blue bar. At 720 dpi it is 10,000 px wide — past the 8192 px a single
/// raster may be — with few enough pixels to keep the test cheap.
fn wide_fixture(test: &str) -> PathBuf {
    let path = tmp(test, "wide.pdf");
    let content = "0 0 1 rg 900 2 80 16 re f";
    let pdf = format!(
        "%PDF-1.4
1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj
2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj
3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 1000 20] /Contents 4 0 R >> endobj
4 0 obj << /Length {} >> stream
{content}
endstream endobj
trailer << /Root 1 0 R >>
%%EOF",
        content.len()
    );
    std::fs::write(&path, pdf).unwrap();
    path
}

/// A dpi that one raster cannot hold is honoured by tiling instead of being quietly pulled back
/// to 8192 px a side (`view.deep-zoom-tiles` at the CLI).
#[test]
fn a_high_dpi_is_not_pulled_back_to_the_single_raster_cap() {
    let pdf = wide_fixture("hidpi");
    let out = tmp("hidpi", "p1.png");
    let r =
        Command::new(BIN).args(["render", pdf.to_str().unwrap(), "--page", "1", "--dpi", "720", "--out", out.to_str().unwrap()]).output().unwrap();
    assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
    let bytes = std::fs::read(&out).unwrap();
    assert_eq!(&bytes[12..16], b"IHDR");
    assert_eq!((be32(&bytes[16..20]), be32(&bytes[20..24])), (10_000, 200), "the full 720 dpi, not a capped 8192");
    let stderr = String::from_utf8_lossy(&r.stderr);
    assert!(stderr.contains("10000×200"), "{stderr}");
}
