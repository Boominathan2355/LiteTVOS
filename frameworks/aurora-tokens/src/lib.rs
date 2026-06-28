//! Aurora UI design tokens — the single source of truth in code.
//!
//! Mirrors `docs/04-Design-System.md` and `frameworks/aurora-ui/tokens.css`.
//! The renderer (Skia) and compositor consume these; nothing hard-codes values.

/// 8-bit sRGB color with alpha.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color {
    /// Parse `#RRGGBB` (alpha = 255). Panics on malformed input — tokens are
    /// compile-time constants, so a bad literal is a build-time bug.
    pub const fn hex(rgb: u32) -> Self {
        Color(
            ((rgb >> 16) & 0xFF) as u8,
            ((rgb >> 8) & 0xFF) as u8,
            (rgb & 0xFF) as u8,
            0xFF,
        )
    }
    /// Premultiplied-friendly RGBA tuple for the renderer.
    pub const fn rgba(self) -> (u8, u8, u8, u8) {
        (self.0, self.1, self.2, self.3)
    }
}

/// Color palette — see `docs/04-Design-System.md` § Color Palette.
pub mod palette {
    use super::Color;
    pub const BG: Color = Color::hex(0x101114);
    pub const SURFACE: Color = Color::hex(0x1B1D22);
    pub const SURFACE_2: Color = Color::hex(0x23262D); // raised / focused
    pub const PRIMARY: Color = Color::hex(0x3A7AFE);
    pub const SECONDARY: Color = Color::hex(0x5C6BC0);
    pub const SUCCESS: Color = Color::hex(0x34C759);
    pub const WARNING: Color = Color::hex(0xF4B400);
    pub const DANGER: Color = Color::hex(0xEA4335);
    pub const TEXT: Color = Color::hex(0xFFFFFF);
    pub const TEXT_2: Color = Color::hex(0x9AA0A6);
}

/// Accent colors a theme may select (default = PRIMARY).
pub mod accent {
    use super::Color;
    pub const BLUE: Color = super::palette::PRIMARY;
    pub const PURPLE: Color = Color::hex(0x8E7CFF);
    pub const ORANGE: Color = Color::hex(0xFF8A3D);
    pub const GREEN: Color = super::palette::SUCCESS;
    pub const RED: Color = super::palette::DANGER;
    pub const WHITE: Color = super::palette::TEXT;
}

/// Corner radii (px). `docs/04` § Rounded corners.
pub mod radius {
    pub const SM: f32 = 12.0;
    pub const MD: f32 = 16.0;
    pub const LG: f32 = 20.0;
    pub const XL: f32 = 24.0;
}

/// 8pt spacing grid (px).
pub mod space {
    pub const X1: f32 = 4.0;
    pub const X2: f32 = 8.0;
    pub const X3: f32 = 12.0;
    pub const X4: f32 = 16.0;
    pub const X5: f32 = 24.0;
    pub const X6: f32 = 32.0;
}

/// Motion durations (ms). Hard rule: never exceed [`motion::MAX_MS`].
pub mod motion {
    pub const D1: u16 = 100;
    pub const D2: u16 = 150;
    pub const D3: u16 = 250;
    pub const D4: u16 = 350;
    /// `docs/04`: "never exceed 400ms".
    pub const MAX_MS: u16 = 400;

    /// Focus scale-up factor (`--au-focus-scale`).
    pub const FOCUS_SCALE: f32 = 1.06;
}

/// Adaptive frame-rate tier — `docs/04` § Adaptive frame rate.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MotionTier {
    /// Full motion: springs, shared-element, depth, glass.
    Full60,
    /// Cheap transforms only; blur/glass/depth stepped down.
    Reduced30,
    /// Minimal: fades + instant focus snaps; expensive effects off.
    Minimal,
}

impl MotionTier {
    /// Pick a tier from the frame rate the hardware can actually sustain.
    pub fn for_fps(fps: u32) -> Self {
        match fps {
            f if f >= 55 => MotionTier::Full60,
            f if f >= 28 => MotionTier::Reduced30,
            _ => MotionTier::Minimal,
        }
    }
    /// Whether expensive effects (glass, depth, blur) are allowed.
    pub fn rich_effects(self) -> bool {
        matches!(self, MotionTier::Full60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_parses_channels() {
        assert_eq!(palette::PRIMARY.rgba(), (0x3A, 0x7A, 0xFE, 0xFF));
        assert_eq!(palette::BG.rgba(), (0x10, 0x11, 0x14, 0xFF));
    }

    #[test]
    fn durations_never_exceed_max() {
        for d in [motion::D1, motion::D2, motion::D3, motion::D4] {
            assert!(d <= motion::MAX_MS, "{d}ms exceeds the 400ms motion budget");
        }
    }

    #[test]
    fn motion_tier_follows_fps() {
        assert_eq!(MotionTier::for_fps(60), MotionTier::Full60);
        assert_eq!(MotionTier::for_fps(30), MotionTier::Reduced30);
        assert_eq!(MotionTier::for_fps(10), MotionTier::Minimal);
        assert!(MotionTier::for_fps(60).rich_effects());
        assert!(!MotionTier::for_fps(30).rich_effects());
    }
}
