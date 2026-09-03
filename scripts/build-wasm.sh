#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cargo_bin="${CARGO:-cargo}"

if [[ -n "${CARGO_HOME:-}" ]]; then
  cargo_home="$(cd "${CARGO_HOME}" && pwd -P)"
else
  cargo_home="$(cd "$(dirname "$(command -v "${cargo_bin}")")/.." && pwd -P)"
fi

rustflags="--remap-path-prefix=${project_root}=/mirra/source --remap-path-prefix=${cargo_home}=/mirra/cargo"
if [[ -n "${RUSTUP_HOME:-}" ]]; then
  rustup_home="$(cd "${RUSTUP_HOME}" && pwd -P)"
  rustflags+=" --remap-path-prefix=${rustup_home}=/mirra/rustup"
fi

SOURCE_DATE_EPOCH=0 RUSTFLAGS="${rustflags}" CARGO_BUILD_JOBS=1 \
  "${cargo_bin}" build --manifest-path "${project_root}/Cargo.toml" \
  --locked --release --target wasm32-unknown-unknown
