#!/usr/bin/env bash
# Build the LiteTV OS image for the QEMU aarch64 target.
# Requires: build/scripts/setup.sh already run, and host Yocto build deps.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
YOCTO_DIR="${YOCTO_DIR:-$REPO_ROOT/../yocto}"

if ! [ -d "$YOCTO_DIR/poky" ]; then
  echo "Yocto not found at $YOCTO_DIR. Run build/scripts/setup.sh first." >&2
  exit 1
fi

# shellcheck disable=SC1091
source "$YOCTO_DIR/poky/oe-init-build-env" "$YOCTO_DIR/build"

bitbake-layers add-layer "$REPO_ROOT/build/meta-litetv" || true

cat >> conf/local.conf <<EOF
MACHINE = "litetv-qemuarm64"
DISTRO = "litetv"
EOF

bitbake litetv-image
echo "Image built. Run with: build/scripts/run-qemu.sh"
