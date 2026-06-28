SUMMARY = "LiteTV OS base image — Aurora UI shell on a lean Wayland stack"
LICENSE = "MIT"

inherit core-image

# Minimal Wayland runtime + the Aurora UI launcher. No X11, no desktop bulk.
IMAGE_INSTALL += " \
    wayland \
    weston \
    litetv-launcher \
    kernel-modules \
    "

# Keep the rootfs lean (docs/13: system image under 2.5 GB).
IMAGE_FEATURES += "ssh-server-dropbear debug-tweaks"
IMAGE_LINGUAS = "en-us"
IMAGE_ROOTFS_EXTRA_SPACE = "131072"

# Hard ceiling so a regression fails the build instead of shipping bloat.
IMAGE_ROOTFS_MAXSIZE = "2560000"
