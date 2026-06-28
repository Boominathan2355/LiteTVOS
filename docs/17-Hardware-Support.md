# 17 · Hardware Support & Future-Proofing

LiteTV OS targets **all current and next-generation TVs** — from HD panels to
8K/120 Hz HDMI 2.1 sets — without abandoning the lean architecture. The native
shell (see [Architecture](02-Architecture.md)) scales by buffer size and clock,
not by swapping runtimes, so the same OS spans every tier.

## Display

| Capability | Support |
|------------|---------|
| Resolution | HD · FHD · **4K (baseline)** · **8K** (scaled profile) |
| Refresh rate | 60 Hz · **120 Hz** · 144 Hz (gaming panels) |
| HDMI | up to **HDMI 2.1** (FRL, 48 Gbps) |
| Gaming | **VRR**, **ALLM**, QMS, QFT |
| Panel tech | LED · QLED · **OLED · QD-OLED · Mini-LED · MicroLED** |

## HDR

- **HDR10**, **HDR10+**, **Dolby Vision**, **HLG**.
- Dynamic-metadata aware; tone-maps to panel capability and respects an 8-bit
  default with 10-/12-bit enabled for HDR content (see
  [Memory Optimization](06-Memory-Optimization.md)).

## Video Codecs

- **H.264/AVC, HEVC/H.265, VP9, AV1** — hardware-decoded up to the panel's max
  resolution/refresh.
- Forward-looking: **AV2 / VVC (H.266)** added as silicon and decoders land,
  delivered via the modular codec layer below.
- Decode always offloaded to fixed-function hardware (CPU/GPU stay free — see
  [Power Optimization](07-Power-Optimization.md)).

## Audio

Dolby/DTS passthrough and **Dolby Atmos / DTS:X over eARC** — see
[Audio Management](15-Audio-Management.md).

## Tuners & Inputs (Live TV)

LiteTV OS supports broadcast and cable TV through the tuner HAL, plus external
inputs:

| Source | Support |
|--------|---------|
| Antenna (over-the-air) | **ATSC 1.0 / ATSC 3.0 (NextGen TV)**, DVB-T/T2, ISDB-T |
| Cable | **QAM**, DVB-C; CableCARD where required |
| Satellite | DVB-S/S2 (with compatible front-end) |
| External | **HDMI 1–3** (one eARC), AV / composite |

- **Channel guide / EPG** — now/next per channel, driven by
  [`aurora-catalog`](../frameworks/aurora-catalog) (`GET /api/channels`).
- **Input switching** — `GET /api/inputs`; HDMI-CEC for one-touch play and
  power. The active source is shown in the [Live TV](03-UI-Design.md) UI.
- **DVR** — record or schedule live programs (`GET /api/recordings`); recordings
  track a lifecycle (Recording / Recorded / Scheduled). Captured to local
  storage via the same hardware decode path, so recording costs no extra CPU.
- Tuning and demux run on the SoC's hardware front-end/decoder, so live TV
  follows the same offload model as streamed media (see
  [Power](07-Power-Optimization.md)).

## Connectivity

- **HDMI 2.1** (eARC, CEC, VRR/ALLM).
- **Wi-Fi 6 / 6E / Wi-Fi 7**, **Bluetooth 5.x / LE Audio**.
- Gigabit / **2.5 GbE** Ethernet, USB 3.x.

## SoC & Architecture Support

- **ARM** (Amlogic, Rockchip, MediaTek Pentonic-class, Allwinner) and **x86**.
- Requirement, not a specific part: a **hardware video decoder + display overlay
  planes + (optional) NPU**. These are what make the 512 MB / 4K and
  [LiteAI](16-AI-Management.md) budgets achievable on commodity silicon.

## Scalable RAM Profiles

The 512 MB ceiling is the **4K baseline and remains fixed**. Higher tiers scale
the pixel buffers (which are raw data and cannot be compressed) while the software
stack stays the same size:

| Profile | Resolution / Refresh | Target RAM | Notes |
|---------|----------------------|-----------|-------|
| **Baseline** | 4K @ 60 Hz | **≤ 512 MB** (fixed) | The core spec |
| Plus | 4K @ 120 Hz | ~640 MB | Larger/extra display buffers |
| Ultra | 8K @ 60 Hz | ~1–1.5 GB | 8K framebuffer ≈ 4× 4K; larger decode DPB |

> 8K and 120 Hz raise only the **pixel-buffer** lines from the
> [4K budget table](06-Memory-Optimization.md); the kernel, shell, services, and
> AI footprints are unchanged. The OS does not get heavier — the pixels do.

## Future-Proofing Strategy

- **HAL abstraction.** Display, decode, audio, and NPU sit behind stable HALs so
  new SoCs and panels are a vendor port ([vendor/](../vendor)), not an OS rewrite.
- **Capability flags.** The platform exposes what the hardware supports (max
  resolution/refresh, HDR formats, codecs, NPU) via the
  [Platform API](12-API.md); apps and the UI adapt instead of assuming.
- **Modular codecs.** New codecs (AV2/VVC) and HDR formats ship as drop-in
  modules, deliverable over [OTA](09-OTA.md) without a full image update.
- **Native shell scales.** Because the UI engine is native (not a heavy runtime),
  moving from 4K to 8K/120 Hz changes buffer sizes and clocks — never the
  architecture or the budget for the software stack.
- **One image, many tiers.** A single build adapts to the detected hardware
  profile at boot; manufacturers select the tier their panel needs.

## Related

- [Architecture](02-Architecture.md) · [Memory](06-Memory-Optimization.md) ·
  [Power](07-Power-Optimization.md)
- [Audio Management](15-Audio-Management.md) · [AI Management](16-AI-Management.md)
- [Roadmap](13-Roadmap.md)
