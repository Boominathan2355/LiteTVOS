# 04 · Design System

The Aurora UI design system. Everything is built from reusable components with
shared spacing, color, typography, and animation timing.

## Visual Identity

- **Theme:** Dark by default
- **Accent colors:** Blue · Purple · Orange · Green · Red · White · Custom
- **Rounded corners:** 12px · 16px · 20px · 24px
- **Glass effects:** Optional — disabled automatically on low-end hardware

## Color Palette

| Token | Value |
|-------|-------|
| Background | `#101114` |
| Surface | `#1B1D22` |
| Primary | `#3A7AFE` |
| Secondary | `#5C6BC0` |
| Success | `#34C759` |
| Warning | `#F4B400` |
| Danger | `#EA4335` |
| Text | White |
| Secondary Text | Gray |

## Typography

| Role | Style |
|------|-------|
| Display | Large titles, bold |
| Heading | Medium, readable |
| Body | Clean, comfortable |
| Caption | Small, high contrast |

## Icon System

- Rounded
- Simple
- Outlined (filled when selected)
- Monochrome option
- Adaptive icons
- SVG based

The reference set lives in [`frameworks/aurora-ui/icons.js`](../frameworks/aurora-ui/icons.js):
line icons on a 24×24 grid drawn with `stroke="currentColor"`, so they inherit
text color and re-theme automatically (no emoji, no icon font, no raster assets).
Apps reference an icon by name via [`aurora-catalog`](../frameworks/aurora-catalog)
(`App.icon`), and the launcher renders it with `icon(name)`.

## Animation System

**Types:** Spring · Fade · Slide · Scale · Depth · Shared Element

**Durations:** 100ms · 150ms · 250ms · 350ms — **never exceed 400ms.**

Animations communicate state; they are never decorative.

### Adaptive frame rate

Motion is tuned to the frame rate the hardware can actually sustain — not a
fixed assumption of 60 FPS.

| Tier | Behavior |
|------|----------|
| 60 FPS | Full motion: springs, shared-element, depth, glass |
| 30 FPS | Cheap transforms only (opacity/scale/translate); blur, glass, and depth stepped down |
| < 30 FPS / Reduced Motion | Minimal transitions — fades and instant focus snaps; expensive effects off |

Rules:

- Detect the achievable frame rate and pick the tier automatically; expose it to
  apps via a capability flag (see [API](12-API.md)).
- Degrade the *effect*, not the *responsiveness* — focus and input stay instant
  at every tier.
- When a transition can't hold its frame budget, shorten or simplify it rather
  than letting it stutter.
- A **Reduced Motion** accessibility option (see [UI Design](03-UI-Design.md))
  forces the minimal tier on any hardware.

## Components

Buttons · Cards · Lists · Dialogs · Menus · Sheets · Switches · Tabs ·
Navigation Rail · Progress Bars · Media Controls

> Everything is built from reusable components so spacing, color, timing, and
> typography stay consistent across every surface.

## Focus States

Per the [focus navigation principle](01-Vision.md), a focused component:

- Scales slightly
- Glows softly
- Elevates
- May display quick actions
