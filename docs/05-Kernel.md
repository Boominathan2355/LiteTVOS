# 05 · Kernel & Low-Level Platform

The kernel layer keeps idle CPU under 2% and boot under 20 seconds while
sustaining 60 FPS UI on supported hardware.

## Targets

| Metric | Target |
|--------|--------|
| CPU idle | < 2% |
| Boot time | < 20 s |
| Architectures | ARM and x86 |

## Responsibilities

- **Scheduler** — prioritizes the UI/render thread to protect 60 FPS; throttles
  background work under load.
- **Memory management** — backs the [memory budget](06-Memory-Optimization.md)
  with aggressive reclaim and low-memory killer tuning.
- **Power governor** — cooperates with [power modes](07-Power-Optimization.md) to
  scale CPU/GPU frequency by workload.
- **I/O** — fast storage path for sub-second cached app launches.

## Platform Configuration

The kernel defconfig enables exactly the features the higher-level subsystems
depend on — nothing more, to keep the image under 2.5 GB:

| Feature | Enables |
|---------|---------|
| **CMA** (contiguous memory allocator) | Media carveout for 4K decode/display buffers ([Memory](06-Memory-Optimization.md)) |
| **zram** + lz4 | In-RAM compressed swap for cold pages |
| **PSI** (pressure stall info) | Proactive reclaim before UI stalls |
| **cgroup v2** (memory + cpu) | Per-process caps and the UI-reserved zone |
| **KSM** | Same-page merging across the shared shell |
| **EAS** + schedutil, big.LITTLE | Energy-aware placement and DVFS ([Power](07-Power-Optimization.md)) |
| **PSR**, panel/overlay planes (DRM/KMS) | Self-refresh + the UI-never-starved-at-4K plane split |
| **V4L2 stateful decode**, DMABUF | Hardware video decode with zero-copy |
| **NPU/DSP drivers** | LiteAI offload ([AI Management](16-AI-Management.md)) |

## Boot Pipeline

1. Bootloader (verified boot — see [Security](08-Security.md))
2. Kernel init
3. Essential services only
4. Launcher start (target < 500 ms after services)

## Developer Mode (low-level)

Exposed through the Settings → Developer surface:

- ADB / Wireless ADB
- FPS Counter
- CPU Usage · GPU Usage · Memory Usage
- Network Monitor
- Frame Profiler
- System Logs
- Crash Reports

See [tools/](../tools) for the profiling tooling that consumes these signals.
