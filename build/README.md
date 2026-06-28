# build — LiteTV OS image build (Yocto)

Builds the LiteTV OS image for the **QEMU aarch64** reference target
(virtual-first bring-up; real SoCs are added as machine confs later).

- Build system: **Yocto / OpenEmbedded**
- Distro: `litetv` — lean Wayland appliance, no X11, busybox init
- Image: `litetv-image` — Wayland + Weston + the Aurora UI launcher
- Target: `litetv-qemuarm64`

## Layout

```
build/
├── meta-litetv/                       # the Yocto layer
│   ├── conf/layer.conf
│   ├── conf/distro/litetv.conf
│   ├── conf/machine/litetv-qemuarm64.conf
│   ├── recipes-core/images/litetv-image.bb
│   └── recipes-graphics/aurora-ui/litetv-launcher_0.1.0.bb
└── scripts/
    ├── setup.sh        # clone poky + layers
    ├── build.sh        # bitbake litetv-image
    └── run-qemu.sh     # boot it under QEMU
```

## Quick start

```bash
build/scripts/setup.sh     # fetch Yocto (poky) — one time
build/scripts/build.sh     # build litetv-image
build/scripts/run-qemu.sh  # boot in QEMU
```

> Requires standard Yocto host dependencies and `qemu-system-aarch64`. The first
> build is large; subsequent builds are incremental.

## Notes

- The `litetv-launcher` recipe builds the Rust shell (see the workspace root
  `Cargo.toml`). It builds the headless backend today; the Smithay + Skia render
  path enables via the crate's `render` feature once those deps are vendored.
- Image size is capped (`IMAGE_ROOTFS_MAXSIZE`) so bloat fails the build rather
  than shipping — see [docs/13-Roadmap.md](../docs/13-Roadmap.md).

See [docs/](../docs/README.md) for design and architecture.
