# 14 · Contributing

LiteTV OS has an open-source core and a developer community, with a commercial
edition for TV manufacturers.

## Repository Layout

| Directory | Contents |
|-----------|----------|
| `docs/` | Design and engineering documentation (this set) |
| `frameworks/` | Core frameworks and the shared design-system library |
| `launcher/` | Aurora UI home launcher |
| `systemui/` | Top bar, quick settings, notification center |
| `settings/` | Settings application |
| `kernel/` | Kernel config and patches |
| `vendor/` | Board support, HALs, vendor blobs |
| `build/` | Build system, manifests, image packaging |
| `tools/` | Developer tooling and profilers |
| `themes/` | Wallpapers, accents, theme-store assets |

## Principles for Contributors

- **Respect the budget.** Changes must not regress the
  [engineering goals](13-Roadmap.md) (RAM, CPU idle, boot, FPS, image size).
- **Use the design system.** Build UI from existing
  [components](04-Design-System.md); don't fork styling or timing.
- **Honor the animation rule.** No transition exceeds 400 ms.
- **Adaptive by default.** Gate expensive effects behind hardware capability
  flags so low-end devices degrade gracefully.
- **Focus-first.** All interactive UI must work with the remote/focus engine.

## Workflow

1. Open an issue describing the change and its budget impact.
2. Branch, implement, and validate against target hardware using
   [Developer Mode](11-Developer-SDK.md) profilers.
3. Document user-facing changes in `docs/`.
4. Submit for review.

## Editions

- **Open-source core** — community-developed platform.
- **Commercial edition** — packaged and supported for TV manufacturers.
