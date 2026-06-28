# 06 · Memory Optimization

A small, predictable memory footprint is a first-class feature of LiteTV OS.

## Budget

> **Fixed specification: total system RAM stays at or under 512 MB at all times,
> including heavy 4K usage** (OS + UI shell + a foreground 4K stream/player,
> including all decode and display buffers). This is the single RAM target for
> LiteTV OS — there is no separate idle or Google-services budget.

System image stays **under 2.5 GB**.

## Full-Usage Budget (Heavy 4K)

The 512 MB ceiling is sized for the worst case — heavy 4K playback. Idle usage
sits comfortably below it.

| Component (full 4K playback, all-in) | RAM |
|--------------------------------------|-----|
| Kernel + drivers | ~60 MB |
| 4K decode reference frames (HEVC 8-bit DPB) | ~130 MB |
| Display buffers — UI plane RGBA (2×) + video overlay plane NV12 (2×) | ~90 MB |
| Native Wayland compositor | ~25 MB |
| Aurora UI shell + launcher | ~50 MB |
| Core services (net, BT, settings, LiteAI-lite) | ~50 MB |
| Foreground app / native player | ~60 MB |
| **Peak total** | **~465 MB** (≈45 MB headroom under 512) |

> The pixel buffers (decode DPB + framebuffers) are ~220 MB of this and **cannot
> be compressed** — they are raw pixel data. The 512 MB target is only reachable
> with the techniques below; running heavy third-party app runtimes
> (Chromium/Flutter/Android-class, 150–400 MB each) on top of 4K playback will
> not fit and is out of scope for this budget — see [Architecture](02-Architecture.md).

### Techniques that make 512 MB fit

- **Hardware overlay planes.** Video is scanned out on a dedicated display plane
  so it bypasses the GPU compositor — avoids a full extra 4K RGBA copy.
- **DMABUF zero-copy.** Decoder output is shared straight to the display plane;
  no intermediate buffer between decode and scan-out.
- **8-bit pipeline by default.** 10-bit/HDR buffers are ~25% larger; reserve them
  for HDR content only.
- **Bounded decode DPB.** Cap reference-frame count to the stream's actual need.
- **zram** for cold pages plus the low-memory killer tuning below.

## Memory Management Subsystems

Memory is actively managed, not just budgeted. The manager runs as a core service
coordinated with the [kernel](05-Kernel.md) and is organized into the layers
below.

### 1. Memory zones (reservations)

RAM is partitioned into protected zones so the UI can never be starved by media
or background work:

| Zone | Owner | Policy |
|------|-------|--------|
| **UI-reserved** | Compositor + Aurora UI shell | Pinned; never reclaimed or swapped |
| **Media CMA carveout** | Decoder + display planes | Pre-reserved contiguous region; sized for one 4K stream |
| **App working set** | Foreground app/player | Bounded by cgroup; reclaimed first under pressure |
| **Cache/elastic** | Page cache, prefetch, thumbnails | Shrinks on demand |

The UI-reserved and CMA zones are allocated at boot so a 4K stream starting up can
never compete with the shell for memory.

### 2. Reclaim and compression

- **zram swap.** Cold anonymous pages are compressed in RAM (lz4) instead of
  paged to slow storage — typically 2–3× effective capacity on cold data.
- **PSI-driven reclaim.** Pressure Stall Information drives proactive reclaim
  *before* allocation stalls reach the UI thread, rather than reacting at OOM.
- **KSM (same-page merging).** Identical pages across the shared design-system
  code path are deduplicated.
- **Tiered low-memory killer.** Under pressure, the manager sheds in order:
  elastic cache → background services → backgrounded apps — and **never** the
  UI-reserved zone.

### 3. Per-process limits (cgroups)

Every app and service runs under a memory cgroup with a hard cap, so no single
process can blow the 512 MB budget. The foreground app/player gets the largest
cap; background services are squeezed and idle-killed.

### 4. Buffer pools (zero-copy)

- **DMABUF pools.** Decode and display buffers are recycled from a fixed pool —
  no per-frame allocation churn, no fragmentation during playback.
- **Slab/jemalloc tuning.** Arenas sized for the steady-state shell to avoid heap
  growth over long uptimes.

### 5. Asset & service discipline

- **Minimal idle service set.** Only essential services run at idle; everything
  else is demand-started and idle-killed.
- **Streamed launcher art.** Poster/thumbnail art is decoded on demand,
  downscaled to display size, and dropped when off-screen — never full-res in RAM.
- **mmap'd, demand-paged assets.** Fonts, icons, and UI assets are memory-mapped
  and paged in lazily; unused pages are reclaimable.
- **Shared component library.** A single design-system code path avoids duplicate
  UI allocations across surfaces.
- **Adaptive effects.** Glass effects and live wallpapers are disabled
  automatically on low-end hardware to free memory and GPU bandwidth.
- **Cached app launches.** Hot caches keep cached app launch under 1 second
  without holding excessive resident memory.

## UI Is Never Starved at 4K

A hard guarantee: **4K playback must never cost the UI a frame.** This is enforced
structurally, not by best effort:

- **Separate display planes.** Video scans out on a hardware overlay plane while
  the UI composites on its own plane. The two never share a buffer or a
  composition pass, so 4K decode cannot stall UI compositing.
- **Reserved UI memory.** The UI-reserved zone (above) is pinned — UI plane
  buffers, glyph atlases, and the focus/animation state always have RAM, even at
  the 4K peak.
- **OOM protection.** The compositor and shell carry the most-protected
  `oom_score_adj`; the killer sheds everything else first.
- **GPU time budget.** The compositor holds a guaranteed GPU time slice each
  frame so overlay/scaling work for video cannot crowd out UI rendering.
- **Animation budget intact.** Because video bypasses the GPU compositor, the
  full ≤ 400 ms motion system (see [Design System](04-Design-System.md)) runs at
  60 FPS *on top of* live 4K — overlays, focus glow, and transitions are
  unaffected.

> Result: opening Quick Settings, moving focus, or animating a transition stays
> smooth while a 4K stream plays underneath — no dropped frames, no stutter.

## Measurement

Use the Developer Mode memory tools (Memory Usage, Frame Profiler) described in
[Kernel](05-Kernel.md) and the profilers under [tools/](../tools) to validate the
budget on target hardware.
