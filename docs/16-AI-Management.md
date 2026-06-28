# 16 · AI Management (LiteAI)

LiteAI is the **offline-first**, on-device intelligence layer. It is managed like
any other subsystem: it lives **within the fixed 512 MB budget**, runs on
fixed-function hardware where possible, and **always yields to the UI and media** —
voice features must never cost a UI frame or break 4K playback.

## Goals

- **Offline by default.** Core voice and search work with no network; cloud is
  opt-in only.
- **Private.** On-device processing; audio and queries are not sent off-device
  unless the user enables a cloud feature.
- **Cheap at rest.** Near-zero cost when idle; cost only while actively serving a
  request.
- **Never intrusive.** AI yields CPU/GPU/RAM to the compositor and decoder.

## Capabilities

Voice Commands · Search · Settings Control · App Launcher · Media Search ·
Device Diagnostics. *(Future: local LLM.)*

## Pipeline

```
Wake word → Capture → ASR → NLU/Intent → Action → Response (TTS / UI)
  (DSP,        (audio   (NPU/   (intent     (system   (audio mgr +
   always-on)   mgr)     CPU)    router)     APIs)      UI overlay)
```

- **Wake-word engine** runs continuously on the low-power **DSP** — a few MB, no
  main-CPU wakeups, no measurable battery/thermal cost.
- **ASR (speech→text)** and **NLU (intent)** load on demand and run on the **NPU**
  (or quantized on CPU) only while a request is active.
- **Intent router** maps to platform actions via the [Platform API](12-API.md)
  (settings, launcher, search, media, diagnostics).
- **Response** is spoken via the [Audio Manager](15-Audio-Management.md) (ducking
  media) and/or shown as a minimal UI overlay.

## Resource Model (fits 512 MB)

AI is **demand-loaded into the elastic memory zone** — it borrows headroom, never
the protected zones:

| State | Footprint | Notes |
|-------|-----------|-------|
| Idle | ~few MB (DSP wake-word only) | No model resident in RAM |
| Active query | small quantized models, mmap'd | Loaded on demand, unloaded after |
| During 4K playback | streaming ASR on NPU, minimal RAM | Heavy inference deferred; never touches the UI-reserved or CMA zones |

Rules:

- **Priority.** AI is below the compositor and decoder in CPU/GPU/memory priority;
  it is reclaimed first under pressure (see
  [Memory Optimization](06-Memory-Optimization.md)).
- **Quantized, mmap'd models.** Models are memory-mapped and demand-paged, then
  released when the request completes — they do not stay resident.
- **NPU/DSP offload.** Inference runs on fixed-function accelerators where present,
  keeping the CPU/GPU free for UI and video (see
  [Power Optimization](07-Power-Optimization.md)).
- **No UI cost.** Because AI work is offloaded and memory-bounded, voice during 4K
  playback does not drop a UI frame — it only ducks audio.

## Device Diagnostics

LiteAI consumes the Developer-Mode signals (memory, CPU/GPU, thermal, network —
see [Kernel](05-Kernel.md)) to answer natural-language device questions
("why is it warm?", "how much storage is left?") and to suggest fixes. It reads
these signals; it does not override the memory or power managers.

## Privacy & Control

- On-device processing by default; an explicit opt-in is required for any cloud
  model or query upload.
- A visible indicator while the microphone is active.
- Per-profile ([Security](08-Security.md)) AI history and preferences; can be
  cleared or disabled entirely.

## Future

- **Local LLM** for richer conversational control and personalization, gated on
  hardware with sufficient NPU/RAM — still under the offline-first, yields-to-UI
  rules above, and still out of scope for the base 512 MB target unless the device
  ships extra RAM.

## Related

- [Ecosystem — LiteAI](10-Ecosystem.md)
- [Audio Management](15-Audio-Management.md)
- [Platform API](12-API.md)
- [Memory Optimization](06-Memory-Optimization.md) ·
  [Power Optimization](07-Power-Optimization.md)
