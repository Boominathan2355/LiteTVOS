# LiteTV OS

**Codename:** Aurora UI · **Version:** 1.0

> Beautiful. Fast. Intelligent. Minimal.

LiteTV OS is a lightweight, elegant, and developer-friendly smart TV platform that
delivers premium usability without requiring premium hardware. It emphasizes
efficiency, openness, and a distinctive design language rather than imitating
existing ecosystems.

---

## Repository Layout

```
LiteTVOS/
├── docs/        # Design, architecture, and engineering documentation
├── frameworks/  # Core system frameworks and shared libraries
├── launcher/    # Home screen launcher (Aurora UI)
├── systemui/    # System UI: top bar, quick settings, notifications
├── settings/    # Settings application
├── kernel/      # Kernel configuration and patches
├── vendor/      # Vendor blobs, board support, HALs
├── build/       # Build system, manifests, image packaging
├── tools/       # Developer tooling, scripts, profilers
└── themes/      # Wallpapers, accent packs, theme store assets
```

## Building

**Native Aurora UI shell** (Rust workspace — runs on your desktop now):

```bash
cargo test --workspace          # focus engine + design-token tests
cargo run --bin litetv-launcher # headless shell: scripted remote navigation
```

Crates: `frameworks/aurora-tokens` (design tokens), `frameworks/aurora-focus`
(TV focus engine), `launcher` (shell binary). The Smithay compositor + Skia
renderer attach behind the launcher's `render` feature.

**Live demo + backend:** <https://litetvos.onrender.com> (the launcher doubles as
a std-only HTTP backend when `PORT` is set — `PORT=8080 cargo run`). Deploy config
in [render.yaml](render.yaml). This is a demo host, not the OS itself.

The page is a full launcher shell — **left sidebar** (Home · Search · Apps ·
Live TV · Media · Settings), a **top search bar** with status + clock, landscape
**Continue Watching** heroes, an app row, Recently-Used chips, a DVR recordings
strip, light/dark theming, and SVG icons — all focus-navigable and data-served by
the Rust backend (`aurora-catalog` + `aurora-focus`):

| Endpoint | Returns |
|----------|---------|
| `GET /` | The Aurora UI launcher (SPA) |
| `GET /api/home` | Home rows with media items |
| `GET /api/apps` | App library |
| `GET /api/channels` | Live TV channels (cable + antenna) with now/next |
| `GET /api/inputs` | Tuner & HDMI inputs |
| `GET /api/recordings` | DVR recordings (recorded / recording / scheduled) |
| `GET /api/search?q=` | Universal search over media, apps + channels |
| `GET /api/item?id=` | Media/app detail |
| `GET /api/navigate?row=&col=&dir=` | Stateless focus move (engine demo) |
| `GET /api/state` | Scripted navigation trace |
| `GET /api/specs` | Engineering targets (512 MB, 60 FPS, …) |
| `GET /healthz` | Health check |

Crates behind it: `frameworks/aurora-catalog` (content model + search),
`frameworks/aurora-focus` (navigation), `frameworks/aurora-tokens` (design tokens).

**OS image** (Yocto → QEMU aarch64): see [build/README.md](build/README.md).

- Stack: Yocto · lean Wayland (no X11) · Rust/Skia (Smithay) shell · QEMU-first

## Documentation

Start with the [docs index](docs/README.md). Highlights:

| Doc | Topic |
|-----|-------|
| [01-Vision](docs/01-Vision.md) | Vision and design philosophy |
| [02-Architecture](docs/02-Architecture.md) | System architecture |
| [03-UI-Design](docs/03-UI-Design.md) | UI surfaces and layouts |
| [04-Design-System](docs/04-Design-System.md) | Visual identity and components |
| [05-Kernel](docs/05-Kernel.md) | Kernel and low-level platform |
| [06-Memory-Optimization](docs/06-Memory-Optimization.md) | Memory budget and tuning |
| [07-Power-Optimization](docs/07-Power-Optimization.md) | Power modes |
| [08-Security](docs/08-Security.md) | Security model |
| [09-OTA](docs/09-OTA.md) | Over-the-air updates |
| [10-Ecosystem](docs/10-Ecosystem.md) | The Lite ecosystem |
| [11-Developer-SDK](docs/11-Developer-SDK.md) | Developer SDK |
| [12-API](docs/12-API.md) | Platform APIs |
| [13-Roadmap](docs/13-Roadmap.md) | Roadmap and future vision |
| [14-Contributing](docs/14-Contributing.md) | Contributing guide |
| [15-Audio-Management](docs/15-Audio-Management.md) | Audio pipeline, routing, passthrough, sync |
| [16-AI-Management](docs/16-AI-Management.md) | LiteAI on-device intelligence |
| [17-Hardware-Support](docs/17-Hardware-Support.md) | Latest-TV support (8K, HDMI 2.1, HDR) and future-proofing |

## Engineering Targets

- **Total RAM:** ≤ 512 MB, all-in (including heavy 4K usage)
- **CPU Idle:** < 2%
- **Boot Time:** < 20 s
- **Launcher Start:** < 500 ms
- **App Launch:** < 1 s for cached apps
- **Animation:** 60 FPS on supported hardware
- **System Image:** under 2.5 GB

## License

Open-source core with a commercial edition for TV manufacturers. See
[docs/14-Contributing.md](docs/14-Contributing.md).
