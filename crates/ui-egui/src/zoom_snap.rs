//! View ▸ Zoom ▸ Marquee Zoom (drag a rectangle to fill the window with it; click to zoom in)
//! and Edit ▸ Take a Snapshot (drag a rectangle to copy that area as an image).

use egui::{Color32, CornerRadius, Pos2, Rect, Stroke};
use pdfcraft_render::{MAX_SCALE, MAX_TILE_SIDE, RenderConfig, RenderRequest, RequestKind, Tile};

use crate::canvas::{DocView, PageXform};
use crate::{PdfCraftApp, QuickTool};

/// A finished gesture: (page, the rectangle on screen, the same in page view points
/// [x0, y0, x1, y1]); a plain click has an empty rectangle.
pub type Marquee = (usize, Rect, [f32; 4]);

/// Drag a rectangle on page `page` (Marquee Zoom, Snapshot).
pub(crate) fn page_input(ui: &egui::Ui, resp: &egui::Response, xf: &PageXform, page: usize, view: &mut DocView) {
    let pointer = ui.input(|i| i.pointer.hover_pos());
    let origin = ui.input(|i| i.pointer.press_origin());
    if pointer.is_some_and(|p| xf.rect.contains(p)) {
        ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
    }
    if resp.drag_started()
        && let Some(o) = origin.filter(|o| xf.rect.contains(*o))
    {
        view.marquee = Some((page, o));
    }
    if let Some((p, start)) = view.marquee
        && p == page
        && let Some(end) = pointer
    {
        let end = Pos2::new(end.x.clamp(xf.rect.left(), xf.rect.right()), end.y.clamp(xf.rect.top(), xf.rect.bottom()));
        let r = Rect::from_two_pos(start, end);
        ui.painter().rect_stroke(r, CornerRadius::ZERO, Stroke::new(1.0, Color32::from_rgb(0x14, 0x73, 0xE6)), egui::StrokeKind::Middle);
        if resp.drag_stopped() {
            view.marquee = None;
            let (a, b) = (xf.screen_to_view(r.left_top()), xf.screen_to_view(r.right_bottom()));
            view.marquee_done = Some((page, r, [a.0.min(b.0), a.1.min(b.1), a.0.max(b.0), a.1.max(b.1)]));
        }
    }
    if resp.clicked()
        && let Some(p) = pointer.filter(|p| xf.rect.contains(*p))
    {
        let v = xf.screen_to_view(p);
        view.marquee_done = Some((page, Rect::from_min_size(p, egui::Vec2::ZERO), [v.0, v.1, v.0, v.1]));
    }
}

impl PdfCraftApp {
    /// Finish a marquee gesture for the current tool.
    pub(crate) fn finish_marquee(&mut self, index: usize, done: Marquee) {
        let (page, rect, view_rect) = done;
        match self.quick_tool {
            QuickTool::MarqueeZoom => {
                let view = &mut self.views[index];
                if rect.width() < 6.0 || rect.height() < 6.0 {
                    // A click zooms in one step around the point.
                    let z = (view.zoom * 1.5).min(64.0);
                    view.zoom_at(z, rect.center());
                } else {
                    view.zoom_to_rect(rect);
                }
            }
            QuickTool::Snapshot => {
                if view_rect[2] - view_rect[0] < 2.0 || view_rect[3] - view_rect[1] < 2.0 {
                    return;
                }
                match self.snapshot(index, page, view_rect) {
                    Ok((w, h)) => {
                        self.notify_fmt("The selected area has been copied ({w} × {h} pixels)", &[("w", &w.to_string()), ("h", &h.to_string())])
                    }
                    Err(e) => self.notify_fmt("Couldn't take the snapshot: {e}", &[("e", &e.to_string())]),
                }
            }
            _ => {}
        }
    }

    /// View ▸ Zoom ▸ Fit Visible on the current page: find where the page has ink (a quick
    /// low-resolution render) and zoom so that spans the window's width. A blank page fits the
    /// width.
    pub fn fit_visible(&mut self, index: usize) -> Result<(), String> {
        let view = &self.views[index];
        let page = view.current;
        let doc = self.session.get(view.id).ok_or("no document")?;
        let config = RenderConfig { password: doc.password.as_deref().map(std::sync::Arc::from), ..RenderConfig::default() };
        let mut r = pdfcraft_render::PageRenderer::new(doc.bytes.clone(), config);
        // About 800 px along the longer side is plenty to find the margins.
        let size = doc.info.pages.get(page).map_or(792.0, |p| p.width.max(p.height));
        let out = r.render(RenderRequest { page, kind: RequestKind::Pixels, tile: None, scale: (800.0 / size).clamp(0.1, 4.0), tag: 0 });
        if let Some(e) = out.error {
            return Err(e);
        }
        let rot = view.rotation;
        let view = &mut self.views[index];
        match ink_bounds(out.width as usize, out.height as usize, &out.rgba) {
            Some([u0, v0, u1, v1]) => {
                // View space → the rotated view on screen.
                let rotate = |u: f32, v: f32| match rot {
                    90 => (1.0 - v, u),
                    180 => (1.0 - u, 1.0 - v),
                    270 => (v, 1.0 - u),
                    _ => (u, v),
                };
                let (a, b) = (rotate(u0, v0), rotate(u1, v1));
                if !view.fit_content(page, [a.0.min(b.0), a.1.min(b.1), a.0.max(b.0), a.1.max(b.1)]) {
                    view.fit = crate::canvas::Fit::Width;
                }
            }
            None => view.fit = crate::canvas::Fit::Width,
        }
        Ok(())
    }

    /// Render `view_rect` (page view points) of `page` and copy it to the clipboard (and keep it in
    /// `last_snapshot`).
    ///
    /// The copy is made at the resolution the view shows on screen (`DocView::render_scale`), so a
    /// snapshot taken at a deep zoom is as sharp as the screen instead of being pulled back to a
    /// fixed scale, and a deep-zoom copy is not resampled when it is pasted. Only the size one
    /// raster allows lowers that, and then only as far as it must.
    pub fn snapshot(&mut self, index: usize, page: usize, view_rect: [f32; 4]) -> Result<(u32, u32), String> {
        let view = &self.views[index];
        let doc = self.session.get(view.id).ok_or("no document")?;
        let ppp = self.ctx.as_ref().map_or(2.0, |c| c.pixels_per_point());
        let (vw, vh) = ((view_rect[2] - view_rect[0]).max(0.0), (view_rect[3] - view_rect[1]).max(0.0));
        let scale = snapshot_scale(view.render_scale(ppp), vw, vh);
        let tile = Tile {
            x: (view_rect[0] * scale).floor().max(0.0) as u32,
            y: (view_rect[1] * scale).floor().max(0.0) as u32,
            w: (vw * scale).ceil().max(1.0) as u32,
            h: (vh * scale).ceil().max(1.0) as u32,
        };
        // `snapshot_scale` keeps the region inside one tile, so this cannot fire for a selection
        // made on screen; it guards the public entry point against a rectangle no raster could hold.
        if tile.w > MAX_TILE_SIDE || tile.h > MAX_TILE_SIDE {
            return Err("the area is too large at this zoom".into());
        }
        let config = RenderConfig { password: doc.password.as_deref().map(std::sync::Arc::from), ..RenderConfig::default() };
        let mut r = pdfcraft_render::PageRenderer::new(doc.bytes.clone(), config);
        let out = r.render(RenderRequest { page, kind: RequestKind::Pixels, tile: Some(tile), scale, tag: 0 });
        if let Some(e) = out.error {
            return Err(e);
        }
        let (w, h) = (out.width, out.height);
        #[cfg(not(target_arch = "wasm32"))]
        if self.system_clipboard {
            let img = arboard::ImageData { width: w as usize, height: h as usize, bytes: std::borrow::Cow::Borrowed(&out.rgba) };
            arboard::Clipboard::new().and_then(|mut c| c.set_image(img)).map_err(|e| e.to_string())?;
        }
        self.last_snapshot = Some((w, h, out.rgba));
        Ok((w, h))
    }
}

/// The device pixels per point a snapshot of a `w` × `h` point area may use: `wanted` (what the
/// view shows on screen), pulled back only as far as one raster requires — a side within
/// [`MAX_TILE_SIDE`] — and kept inside the renderer's own `0.01`…`MAX_SCALE` range. A selection
/// therefore always copies the sharpest image that fits rather than one at a fixed scale.
fn snapshot_scale(wanted: f32, w: f32, h: f32) -> f32 {
    let mut scale = if wanted.is_finite() { wanted } else { 1.0 };
    let longest = w.max(h);
    if longest.is_finite() && longest > 0.0 {
        scale = scale.min(MAX_TILE_SIDE as f32 / longest);
    }
    scale.clamp(0.01, MAX_SCALE)
}

/// The normalised bounds [u0, v0, u1, v1] of the pixels that aren't (near) white, if any.
pub(crate) fn ink_bounds(w: usize, h: usize, rgba: &[u8]) -> Option<[f32; 4]> {
    if w == 0 || h == 0 || rgba.len() < w * h * 4 {
        return None;
    }
    let ink = |p: &[u8]| p[3] > 16 && (p[0] < 240 || p[1] < 240 || p[2] < 240);
    let (mut x0, mut y0, mut x1, mut y1) = (w, h, 0, 0);
    for (y, row) in rgba.chunks_exact(w * 4).take(h).enumerate() {
        for (x, p) in row.as_chunks::<4>().0.iter().enumerate() {
            if ink(p) {
                x0 = x0.min(x);
                x1 = x1.max(x + 1);
                y0 = y0.min(y);
                y1 = y1.max(y + 1);
            }
        }
    }
    (x1 > x0 && y1 > y0).then(|| [x0 as f32 / w as f32, y0 as f32 / h as f32, x1 as f32 / w as f32, y1 as f32 / h as f32])
}

#[cfg(test)]
mod tests {
    #[test]
    fn ink_bounds_finds_the_marked_area() {
        let (w, h) = (10, 20);
        let mut px = vec![255u8; w * h * 4];
        for (x, y) in [(2, 5), (7, 15)] {
            px[(y * w + x) * 4] = 0;
        }
        assert_eq!(super::ink_bounds(w, h, &px), Some([0.2, 0.25, 0.8, 0.8]));
        assert_eq!(super::ink_bounds(w, h, &vec![255u8; w * h * 4]), None);
    }

    /// A snapshot copies at the scale the view shows: 16 px/pt came back as 8 (400 % at 2× display
    /// scaling halved the copy), and the copy was resampled on paste.
    #[test]
    fn a_snapshot_keeps_the_on_screen_scale() {
        assert_eq!(super::snapshot_scale(16.0, 50.0, 20.0), 16.0);
        assert_eq!(super::snapshot_scale(170.0, 5.0, 4.0), 170.0);
    }

    /// The only thing that lowers the scale is one raster's size, and then just enough to fit.
    #[test]
    fn a_snapshot_lowers_the_scale_only_to_fit_one_tile() {
        let s = super::snapshot_scale(16.0, 4000.0, 10.0);
        assert!((s * 4000.0 - super::MAX_TILE_SIDE as f32).abs() < 0.5, "{s}");
        // A selection that fits one tile is copied at the screen scale however deep the zoom.
        assert_eq!(super::snapshot_scale(170.0, 20.0, 16.0), 170.0);
    }

    #[test]
    fn a_snapshot_scale_stays_inside_the_renderers_range() {
        assert_eq!(super::snapshot_scale(10_000.0, 4.0, 4.0), pdfcraft_render::MAX_SCALE);
        assert_eq!(super::snapshot_scale(0.0, 4.0, 4.0), 0.01);
        assert_eq!(super::snapshot_scale(f32::NAN, 4.0, 4.0), 1.0);
        assert_eq!(super::snapshot_scale(8.0, f32::NAN, 4.0), 8.0);
    }
}
