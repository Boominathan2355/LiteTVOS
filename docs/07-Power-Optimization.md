# 07 · Power Optimization

LiteTV OS switches power modes automatically based on workload, coordinated with
the [kernel power governor](05-Kernel.md).

## Power Modes

| Mode | Intent |
|------|--------|
| Performance | Maximum responsiveness and frame rate |
| Balanced | Default — responsiveness with efficiency |
| Battery Saver | Reduced clocks and background activity |
| Ultra Low Power | Minimal footprint for idle / standby |
| Gaming | Sustained high performance for games |

Modes switch automatically based on workload; the user can also select a mode
from [Quick Settings](03-UI-Design.md).

## Per-Mode Policy

Each mode is a concrete set of governor, GPU, display, and network settings — not
just a label:

| Knob | Performance | Balanced | Battery Saver | Ultra Low Power | Gaming |
|------|-------------|----------|---------------|-----------------|--------|
| CPU governor | `performance` | `schedutil` (EAS) | `powersave`-biased | deepest idle | `performance` |
| CPU cores | all online | EAS placement | LITTLE-biased | most cores parked | all online + RT pinning |
| GPU clock | high | on-demand | capped | floor | sustained high |
| Display refresh | 60 Hz | 60 Hz (PSR when idle) | PSR / 30 Hz | panel off / dim | 60 Hz, VRR if available |
| Background work | unrestricted | deferred | heavily throttled | suspended | suspended |
| Wi-Fi | full | power-save when idle | aggressive power-save | beacon-only | full, low-latency |

## Memory-Management Subsystems → Power

The power and [memory](06-Memory-Optimization.md) managers share the same
workload signals: parking cores and lowering clocks must never touch the
UI-reserved zone or stall the compositor. Power scaling is applied to background
and elastic work first.

## Subsystem Power Management

### CPU — DVFS & scheduling

- **Energy-Aware Scheduling (EAS).** On big.LITTLE, light/background tasks land on
  efficiency cores; the compositor and decoder land where they meet deadlines.
- **schedutil governor** scales frequency to actual utilization in Balanced;
  `performance` pins max clocks for Gaming and active transitions.
- **Deep idle (C-states).** At idle the SoC enters its deepest C-state to hold
  CPU idle < 2%.

### GPU

- On-demand frequency scaling tied to the compositor's frame budget; clocks rise
  for animation/gaming and fall immediately after.
- A **guaranteed floor clock** during UI animation so motion never stutters when
  the GPU was idling.

### Display

- **Panel Self-Refresh (PSR).** On a static screen the panel refreshes itself and
  the SoC display pipeline powers down.
- **Adaptive backlight / local dimming** scales with content and power mode.
- **Refresh rate** drops (e.g. 30 Hz / PSR) when nothing is animating, snaps back
  to 60 Hz on the first input or animation.

### Video playback (the efficient path)

During 4K playback the heavy lifting is on the **hardware decoder + overlay
plane**, so CPU and GPU stay near idle. This is what lets long playback sessions
run cool and lets Balanced mode (not Performance) handle 4K comfortably.

### Thermal

- Thermal trip points feed back into DVFS: clocks throttle gracefully before
  hotspots, prioritizing the decoder and compositor so playback and UI survive
  throttling while background work yields first.

### Standby & resume

- **Suspend-to-idle / standby** with a small set of wake sources (remote, HDMI-CEC,
  network wake).
- **Fast resume** path restores the shell from the warm UI-reserved zone, keeping
  perceived wake near-instant.

## Automatic Mode Switching

Modes switch on detected workload, with hysteresis to avoid flapping:

| Detected workload | Mode |
|-------------------|------|
| Foreground game | Gaming |
| 4K/HD playback | Balanced (decoder offloads the work) |
| Active browsing/animation | Balanced → Performance on sustained load |
| Idle UI, screensaver | Ultra Low Power (PSR, parked cores) |
| User override | Pinned via [Quick Settings](03-UI-Design.md) |

## UI Stays Smooth Across Modes

Power saving never costs UI fluidity:

- The compositor's **guaranteed GPU floor clock** and EAS placement mean focus
  movement and transitions hit 60 FPS even from a low-power state — the first
  input ramps clocks within a frame.
- Effect *richness* steps down with the mode (per the adaptive
  [animation tiers](04-Design-System.md)), but **responsiveness never does** —
  input and focus stay instant in every mode.
- 4K playback runs under Balanced, leaving thermal and power headroom while the UI
  animates on top of it without dropped frames (see
  [UI Is Never Starved at 4K](06-Memory-Optimization.md)).

## Principles

- **Idle is cheap.** Target CPU idle < 2% so standby power stays low.
- **Offload, don't burn.** Push media to fixed-function hardware (decoder, overlay
  planes) instead of CPU/GPU.
- **Adaptive effects.** Expensive visual effects step down with the power mode.
- **Responsiveness is sacred.** Saving power may simplify effects but never delays
  input or drops UI frames.
- **Workload-aware.** Foreground media playback, gaming, and idle each map to an
  appropriate governor profile.

## Related

- [Kernel & Low-Level Platform](05-Kernel.md)
- [Memory Optimization](06-Memory-Optimization.md)
