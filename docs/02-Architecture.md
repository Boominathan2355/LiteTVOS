# 02 · Architecture

LiteTV OS is layered to keep the system image small, the idle footprint low, and
the UI engine (Aurora UI) decoupled from the platform.

## Layered Overview

```
┌─────────────────────────────────────────────────────────┐
│  Apps          Streaming apps · Games · 3rd-party (SDK)   │
├─────────────────────────────────────────────────────────┤
│  Aurora UI     Launcher · SystemUI · Settings · Media Hub │
│                Design System · Animation engine           │
├─────────────────────────────────────────────────────────┤
│  Lite Services LiteAI · LiteConnect · LiteCast · LiteCloud │
│                Notifications · Search · Profiles           │
├─────────────────────────────────────────────────────────┤
│  Frameworks    Media · Graphics · Input/Focus · Power     │
│                Package mgmt · OTA · Theme/Plugin engine    │
├─────────────────────────────────────────────────────────┤
│  HAL / Vendor  Display · Audio · Wi-Fi/BT · Tuner · GPU    │
├─────────────────────────────────────────────────────────┤
│  Kernel        Scheduler · Memory · Power governor · I/O   │
└─────────────────────────────────────────────────────────┘
```

## Module Mapping (repository)

| Layer | Directory |
|-------|-----------|
| Launcher | `launcher/` |
| System UI (top bar, quick settings, notifications) | `systemui/` |
| Settings app | `settings/` |
| Shared frameworks & design system | `frameworks/` |
| Kernel config & patches | `kernel/` |
| Board support, HALs, vendor blobs | `vendor/` |
| Build system & image packaging | `build/` |
| Dev tooling & profilers | `tools/` |
| Wallpapers, accents, theme assets | `themes/` |

## Design Goals

- **Decoupled UI engine.** Aurora UI talks to the platform through stable
  framework APIs so the renderer and theming evolve independently.
- **Minimal service set.** Only essential services run at idle; everything else
  is demand-started and aggressively idle-killed.
- **Hardware adaptivity.** Expensive effects (glass, live wallpapers) are
  disabled automatically on low-end hardware.
- **ARM and x86.** A common build target supports both architectures.
- **Optional Google services.** The platform runs with or without Google
  services, staying within the same ≤ 512 MB RAM budget either way.

## Memory Constraint → Native Shell

The platform targets **≤ 512 MB total RAM under heavy 4K usage** (see
[Memory Optimization](06-Memory-Optimization.md)). About 220 MB of that peak is
irreducible pixel data (4K decode reference frames + display buffers), which
leaves under ~290 MB for the entire software stack. This rules out heavyweight UI
runtimes and forces a **native shell**:

- **Aurora UI** is a native compositor + renderer (C/C++/Rust on Wayland with
  Skia/GLES) — **not** Chromium, Flutter, or an Android runtime, each of which
  alone would consume 150–400 MB.
- **Media** runs through a first-party native player decoding HLS/DASH directly,
  using hardware overlay planes and DMABUF zero-copy.
- Heavy third-party app runtimes are **out of scope** at this budget; apps are
  native, lightweight, or remote/cast.

This constraint is the reason the base platform should be a lean Linux stack
(Buildroot/Yocto) rather than an AOSP fork.

## Cross-Cutting Systems

- **Focus engine** — single source of truth for remote-driven navigation.
- **Animation system** — shared spring/transition primitives (see
  [Design System](04-Design-System.md)).
- **Power governor** — drives [power modes](07-Power-Optimization.md) based on
  workload.
- **Theme & plugin engine** — backs the [Developer SDK](11-Developer-SDK.md) and
  Theme Store.
