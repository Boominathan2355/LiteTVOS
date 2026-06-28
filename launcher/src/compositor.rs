//! Compositor / renderer seam.
//!
//! The headless build models the shell state (rows, focus, tokens). The real
//! display path — a **Smithay** Wayland compositor driving a **Skia** renderer
//! on KMS/DRM overlay planes — plugs in here behind the `render` feature, per
//! `docs/02-Architecture.md` and `docs/06-Memory-Optimization.md`
//! (UI plane separate from the video overlay plane).

use aurora_focus::Focus;

/// What the renderer needs each frame. The render backend turns this into draw
/// calls; the headless backend prints it.
pub struct Frame<'a> {
    pub row_titles: &'a [String],
    pub focus: Focus,
    /// Achievable frame rate → drives the motion tier (`aurora_tokens`).
    pub fps: u32,
}

/// A display backend. One impl is headless (today); `render` adds the Skia/
/// Smithay impl driving real planes.
pub trait Backend {
    fn present(&mut self, frame: &Frame);
}

/// Headless backend: renders the focused cell as text. Used for tests, CI, and
/// bring-up before the GPU path exists.
pub struct HeadlessBackend;

impl Backend for HeadlessBackend {
    fn present(&mut self, frame: &Frame) {
        let tier = aurora_tokens::MotionTier::for_fps(frame.fps);
        let title = frame
            .row_titles
            .get(frame.focus.row)
            .map(String::as_str)
            .unwrap_or("<none>");
        println!(
            "  [{:?}] focus → row {} (\"{}\"), col {} | scale {:.2}",
            tier,
            frame.focus.row,
            title,
            frame.focus.col,
            aurora_tokens::motion::FOCUS_SCALE,
        );
    }
}

#[cfg(feature = "render")]
mod skia_smithay {
    //! TODO(render): Smithay compositor + Skia renderer.
    //! - Smithay: seat/input (remote → Direction), output on KMS/DRM.
    //! - UI composited on its own plane; video on a hardware overlay plane.
    //! - Skia surface sized to the UI plane; tokens drive paint/radius/motion.
}
