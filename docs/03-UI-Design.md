# 03 · UI Design

All UI surfaces follow the [core principles](01-Vision.md) and are built from the
[Design System](04-Design-System.md) component library.

## Launcher

Sections:

- Home
- Continue Watching
- Favorites
- Recent Apps
- Installed Apps
- Settings
- Inputs
- Search
- Power

## Home Screen Layout

**Top Bar**

- Time
- Wi-Fi
- Bluetooth
- Storage
- Notifications
- Profile

**Center**

- Continue Watching
- Pinned Apps
- Media Recommendations

**Bottom**

- Recently Opened
- Quick Settings

## App Library

- Grid layout
- Alphabetical ordering
- Categories
- Search
- Favorites

**Sort options:** Recently Used · Installed Date · Most Used

## Search

Universal search across:

- Apps
- Movies
- TV Shows
- Settings
- Files
- Voice

## Quick Settings

Wi-Fi · Bluetooth · Brightness · Sound · Display · Power Mode · Network ·
Storage · Developer Mode

## Settings

Appearance · Display · Network · Storage · Applications · Developer · Security ·
Accessibility · Accounts · Updates · About

**Appearance & Themes** — **Dark** (default) and **Light** themes, plus a system
**accent** color (Blue / Purple / Green / Orange). Themes are pure design-token
swaps (color only — radius, spacing, motion are shared), so the whole UI re-skins
instantly and consistently. See [Design System](04-Design-System.md).

## Media Hub

- Unified player
- Continue watching
- Watch history
- Favorites
- Streaming shortcuts
- Subtitle settings
- Audio settings

## Live TV

Cable and antenna tuner support plus external inputs:

- **Channel guide** — now/next EPG per channel, grouped by source (Cable / Antenna)
- **Inputs** — Antenna (ATSC), Cable (QAM), HDMI 1–3 (eARC), Component (YPbPr),
  VGA (PC), AV 1 / AV 2 (composite)
- **Categories** — News, Sports, Movies, Kids, Music, Documentary, Lifestyle
- **Quick tune** — select a channel to watch live; channels are searchable
  through [universal search](#search)
- **DVR / Recordings** — record the live program (or schedule one), with a
  recordings strip showing each item's state: **Recording** (live), **Recorded**
  (ready to play), or **Scheduled**. From a recording you can play, stop, or
  cancel as its state allows.

Backed by the tuner/input HAL — see [Hardware Support](17-Hardware-Support.md).

## Notification Center

- Minimal cards
- Grouped notifications
- Dismiss all
- Silent mode
- Quick reply *(future)*

## Widgets *(optional)*

Weather · Clock · Calendar · Storage · Network · Media Controls · Downloads

## Dynamic Wallpapers

Gradient · Nature · Abstract · Minimal · Live *(optional)* · AI Generated *(future)*

## Sound Design

Minimal UI sounds for: Navigation · Selection · Error · Notification · Power.
The user can disable all sounds. Playback, routing, and passthrough are handled by
the [Audio Management](15-Audio-Management.md) subsystem.

## Accessibility

- Large Text
- Color Blind Mode
- High Contrast
- Screen Reader
- Voice Navigation
- Closed Captions
- Reduced Motion — forces the minimal animation tier (see [Design System](04-Design-System.md))
