# 12 · Platform API

> **Status:** Draft surface definition. APIs are illustrative and subject to
> change before 1.0.

The platform API exposes Aurora UI and Lite services to apps and plugins. The
surface is grouped by capability; each maps to a framework under
[frameworks/](../frameworks).

## API Groups

| Group | Provides |
|-------|----------|
| UI / Design System | Components, focus engine, animation primitives |
| Media | Unified player, playback state, subtitle/audio tracks |
| Audio | Stream/volume policy, routing (eARC/BT), passthrough, A/V sync — see [Audio Management](15-Audio-Management.md) |
| Search | Register search providers for universal search |
| Launcher | Rows, pinned apps, recommendations, widgets |
| Notifications | Post/group/dismiss notifications, silent mode |
| Profiles | Current profile, per-profile storage |
| Connect | Pairing, clipboard, file sharing, handoff |
| Cast | Mirroring and streaming sessions |
| AI | Voice intents, settings control, diagnostics — see [AI Management](16-AI-Management.md) |
| Power | Query/observe power mode |
| Capabilities | Query hardware support: max resolution/refresh, HDR formats, codecs, NPU, frame-rate tier — see [Hardware Support](17-Hardware-Support.md) |
| System | Device info, storage, network, OTA status |

## Conventions

- **Focus-first.** Every interactive API integrates with the shared focus
  engine; components expose focus, glow, and quick-action states.
- **Animation budget.** UI transitions must respect the
  [≤ 400 ms rule](04-Design-System.md).
- **Adaptive.** The Capabilities group exposes hardware flags (max
  resolution/refresh, HDR formats, codecs, NPU, glass-effects, frame-rate tier)
  so apps and the UI adapt instead of assuming — see
  [Hardware Support](17-Hardware-Support.md) and the
  [adaptive frame rate tiers](04-Design-System.md).
- **Permissioned.** Sensitive groups (Connect, Cast, AI, System) are gated by
  profile and permission (see [Security](08-Security.md)).

## Example (illustrative)

```kotlin
// Register a universal-search provider
LiteSearch.registerProvider(object : SearchProvider {
    override fun query(term: String): List<SearchResult> =
        catalog.find(term).map { it.toSearchResult() }
})

// Observe power mode
LitePower.observe { mode ->
    renderer.setEffectsEnabled(mode != PowerMode.UltraLowPower)
}
```

See [Developer SDK](11-Developer-SDK.md) for tooling and the plugin model.
