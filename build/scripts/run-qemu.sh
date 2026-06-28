#!/usr/bin/env bash
# Boot the built LiteTV OS image under QEMU (aarch64).
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
YOCTO_DIR="${YOCTO_DIR:-$REPO_ROOT/../yocto}"

if ! command -v runqemu >/dev/null && ! [ -d "$YOCTO_DIR/poky" ]; then
  echo "Yocto build env not found. Run setup.sh and build.sh first." >&2
  exit 1
fi

# shellcheck disable=SC1091
source "$YOCTO_DIR/poky/oe-init-build-env" "$YOCTO_DIR/build" >/dev/null
runqemu litetv-qemuarm64 litetv-image nographic
