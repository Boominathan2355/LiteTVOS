# 13 · Roadmap

## Engineering Goals

| Metric | Target |
|--------|--------|
| Total RAM (all-in, incl. heavy 4K usage) | ≤ 512 MB |
| CPU idle | < 2% |
| Boot time | < 20 s |
| Launcher start | < 500 ms |
| App launch (cached) | < 1 s |
| Animation | 60 FPS on supported hardware |
| System image | under 2.5 GB |

## Future Vision (5 Years)

- Cross-platform ecosystem
- Custom design language
- 100+ reusable UI components
- Plugin architecture
- AI-powered personalization
- Cloud synchronization
- SDK for third-party TV apps
- Support for ARM and x86
- Support for all latest TVs — 8K, 120 Hz, HDMI 2.1, VRR/ALLM, and Dolby
  Vision / HDR10+ / next-gen panels (OLED, QD-OLED, Mini-LED, MicroLED)
- Future-proof codec/feature delivery (AV1 → AV2/VVC) over OTA
- Developer community
- Open-source core
- Commercial edition for TV manufacturers

## Themes Carried Forward

- **Performance budget is the product.** Every release is measured against the
  engineering goals above ([Memory](06-Memory-Optimization.md),
  [Power](07-Power-Optimization.md), [Kernel](05-Kernel.md)).
- **Ecosystem expansion.** LitePhone, LitePad, and LiteWatch extend the
  [ecosystem](10-Ecosystem.md).
- **Latest-TV first.** New display, HDR, and codec standards are absorbed via HALs
  and OTA modules without growing the software budget — see
  [Hardware Support](17-Hardware-Support.md).
- **Openness.** Open-source core plus a commercial manufacturer edition.

## Success Statement

LiteTV OS should become a lightweight, elegant, and developer-friendly smart TV
platform that delivers premium usability without requiring premium hardware,
emphasizing efficiency, openness, and a distinctive design language rather than
imitating existing ecosystems.
