# 11 · Developer SDK

LiteTV OS is developer-friendly by design: a documented SDK, a plugin
architecture, and a Theme Store.

## SDK Goals

- Build third-party TV apps against stable platform APIs (see [API](12-API.md)).
- Reuse the Aurora UI [design system](04-Design-System.md) (100+ components
  targeted — see [Roadmap](13-Roadmap.md)).
- First-class focus navigation, animation, and media primitives.
- Targets both ARM and x86.

## Plugin System

A plugin architecture lets developers extend system surfaces (launcher rows,
widgets, quick settings, search providers) without forking the OS.

## Theme Store

Distributes accent packs, wallpapers, and themes built on the design-system
tokens. Theme assets live under [themes/](../themes).

## Developer Mode

Enabled via the Developer profile ([Security](08-Security.md)) and Settings →
Developer:

- ADB / Wireless ADB
- FPS Counter
- CPU / GPU / Memory usage
- Network Monitor
- Frame Profiler
- System Logs
- Crash Reports

## Community

The core is open-source with a developer community; a commercial edition targets
TV manufacturers. See [Contributing](14-Contributing.md).
