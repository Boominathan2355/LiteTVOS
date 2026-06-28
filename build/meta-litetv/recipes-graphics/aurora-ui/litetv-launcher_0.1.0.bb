SUMMARY = "LiteTV OS launcher / Aurora UI shell (Rust)"
HOMEPAGE = "LiteTV OS"
LICENSE = "Apache-2.0"
LIC_FILES_CHKSUM = "file://${WORKDIR}/sources/LICENSE;md5=0000000000000000000000000000000000000000"

inherit cargo

# In-tree source. For a real fetch, replace with a git SRC_URI + SRCREV.
# Build with the headless backend by default; the `render` feature pulls the
# Smithay + Skia stack once those recipes/deps are added.
SRC_URI = "file://${TOPDIR}/../"
S = "${WORKDIR}/sources"

CARGO_BUILD_FLAGS:append = " --bin litetv-launcher"

# NOTE: Yocto's cargo class needs the crate dependency list materialized.
# Generate it once deps are added:  cargo bitbake  (from meta-rust-bin tools).
# Today the workspace is std-only, so there are no external crates to fetch.

FILES:${PN} += "${bindir}/litetv-launcher"
