#!/usr/bin/env bash
# Fetch Yocto (poky) and required layers next to this repo, ready to build the
# LiteTV OS QEMU image. Idempotent.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
YOCTO_DIR="${YOCTO_DIR:-$REPO_ROOT/../yocto}"
BRANCH="${YOCTO_BRANCH:-scarthgap}"

mkdir -p "$YOCTO_DIR"
cd "$YOCTO_DIR"

clone() { # url dir
  [ -d "$2/.git" ] || git clone -b "$BRANCH" "$1" "$2"
}

clone https://git.yoctoproject.org/git/poky poky
clone https://git.openembedded.org/meta-openembedded poky/meta-openembedded || true

echo
echo "Yocto ready at: $YOCTO_DIR"
echo "meta-litetv layer: $REPO_ROOT/build/meta-litetv"
echo "Next: build/scripts/build.sh"
