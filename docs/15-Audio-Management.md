# 15 · Audio Management

Audio is managed as a first-class subsystem alongside
[memory](06-Memory-Optimization.md) and [power](07-Power-Optimization.md). It must
deliver low-latency UI sound and high-quality media audio (including object-based
formats) **within the fixed 512 MB budget**, while never stealing time or memory
from the UI or video pipeline.

## Goals

- Low-latency UI feedback sounds and lip-synced media audio.
- Bit-perfect passthrough for premium formats (Dolby/DTS, Atmos via eARC).
- Efficient: audio offloaded to the DSP so the CPU stays near idle during playback.
- Small footprint: the whole audio stack fits inside the elastic budget headroom.

## Pipeline

```
Source (app / media hub / UI) → Decoder/Mixer → Routing → Sink
        │                          │              │         │
   PCM / encoded            DSP offload     stream policy   HDMI/eARC · S/PDIF
                            + lip-sync      + ducking       · analog · Bluetooth
```

- **DSP/HW offload.** Decode and mixing run on the audio DSP where available, so
  long playback sessions add almost no CPU load (ties into
  [Power Optimization](07-Power-Optimization.md)).
- **Low-latency path.** UI sounds and game audio use a short-buffer fast-mixer
  path; media uses a deep-buffer path for efficiency.

## Streams & Mixing

| Stream | Use | Policy |
|--------|-----|--------|
| Media | Movies, music, games | Primary; ducked briefly for voice/alerts |
| UI / SFX | Navigation, selection, error, notification, power | Low-latency, mixed over media |
| Voice / [LiteAI](16-AI-Management.md) | Assistant prompts & replies | Ducks media while active |
| Accessibility | Screen reader, audio description | Highest priority, never ducked away |

UI sounds (Navigation · Selection · Error · Notification · Power) come from the
[Sound Design](03-UI-Design.md) set and can be fully disabled by the user.

## Formats & Passthrough

- **Decode:** PCM, AAC, MP3, Opus, FLAC, AC-3, E-AC-3.
- **Passthrough (bitstream):** Dolby Digital / DD+, DTS, and **Dolby Atmos / DTS:X
  over HDMI eARC** — sent untouched to AVRs/soundbars.
- **Auto-detect** sink capabilities (EDID/eARC) and pick decode vs passthrough.

## Routing

HDMI / **eARC** · Optical S/PDIF · Analog / headphone · **Bluetooth A2DP**
(SBC, AAC, aptX, LDAC). Hot-swap between sinks without dropping the stream.

## A/V Sync

- Lip-sync compensation aligns audio to the video overlay plane, accounting for
  sink latency (eARC reports it; manual offset otherwise).
- Sync is maintained across routing changes and during 4K playback — audio timing
  is locked to the decoder's presentation timestamps.

## Volume & Loudness

- Master + per-stream volume.
- **Loudness normalization** for consistent level across apps/content.
- **Night mode** dynamic range compression.
- Per-output calibration (balance, mono downmix for accessibility).

## Accessibility

- Screen-reader and audio-description streams are prioritized and never ducked
  away by media or UI sound.
- Mono output and left/right balance for hearing accessibility.

## Power & Memory Discipline

- Audio buffers come from a small fixed pool — no per-frame allocation; fits the
  elastic budget alongside the [4K memory plan](06-Memory-Optimization.md).
- DSP offload keeps the CPU in deep idle during playback (see
  [Power Optimization](07-Power-Optimization.md)).
- The mixer and UI-sound path share the UI's protected scheduling so audio
  feedback stays instant even under load.

## Related

- [UI Design — Sound Design](03-UI-Design.md)
- [AI Management](16-AI-Management.md) (voice ducking)
- [Power Optimization](07-Power-Optimization.md)
- [Memory Optimization](06-Memory-Optimization.md)
