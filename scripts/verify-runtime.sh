#!/usr/bin/env bash
# Development evidence for an explicitly identified Wasm; no release approval.
set -euo pipefail
project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "${project_root}"
python3 verification/runtime_preflight.py
: "${POCKET_IC_BIN:?Set POCKET_IC_BIN to the hash-pinned PocketIC 15.0.0 Linux x86_64 binary}"
test -x "${POCKET_IC_BIN}"
expected_server="29472ea4433b30a280676c4e22e369d79d5ba6ee1b4d48bab32ebe7d0ad2b4bb"
actual_server="$(sha256sum "${POCKET_IC_BIN}" | cut -d ' ' -f 1)"
if [[ "${actual_server}" != "${expected_server}" ]]; then
  echo "FAIL: PocketIC server hash mismatch" >&2
  exit 1
fi
if (( $# == 0 )); then
  runtime_wasm="${project_root}/artifacts/mirra_canister.wasm"
  expected_wasm="$(cut -d ' ' -f 1 artifacts/mirra_canister.wasm.sha256)"
elif (( $# == 2 )); then
  runtime_wasm="$1"
  expected_wasm="$2"
else
  echo "Usage: verify-runtime.sh [absolute-wasm-path expected-sha256]" >&2
  exit 1
fi
if [[ "${runtime_wasm}" != /* || ! "${expected_wasm}" =~ ^[0-9a-f]{64}$ ]]; then
  echo "FAIL: supply an absolute Wasm path and lowercase SHA-256." >&2
  exit 1
fi
actual_wasm="$(sha256sum "${runtime_wasm}" | cut -d ' ' -f 1)"
if [[ "${actual_wasm}" != "${expected_wasm}" ]]; then
  echo "FAIL: runtime Wasm hash mismatch" >&2
  exit 1
fi
echo "RUNTIME_WASM_SHA256=${actual_wasm}"
git rev-parse HEAD
if [[ -n "$(git status --porcelain --untracked-files=normal)" ]]; then
  echo "FAIL: runtime evidence requires a clean checkout of the named commit." >&2
  git status --short
  exit 1
fi
sha256sum --check SOURCE_MANIFEST.sha256
sha256sum "${runtime_wasm}" integration-tests/tests/upgrade.rs \
  Cargo.lock integration-tests/Cargo.lock tests/vectors.json
cargo --version
rustc --version
# Compile separately so the runtime deadline does not count dependency builds.
cargo test --manifest-path integration-tests/Cargo.toml --locked --test upgrade --no-run
export POCKET_IC_BIN
# Pin both sides explicitly; an inherited upgrade override must not change this scenario.
MIRRA_WASM="${runtime_wasm}" \
MIRRA_UPGRADE_WASM="${runtime_wasm}" \
  timeout --kill-after=5s 240s cargo test \
    --manifest-path integration-tests/Cargo.toml --locked --test upgrade -- --nocapture
echo "RUNTIME CHECKS PASSED for the displayed Wasm and test-source hashes."
echo "Emulator evidence only. P1 replacement review, real-network verification and upstream attestation remain required."
