#!/usr/bin/env bash
set -u -o pipefail

project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cargo_bin="${CARGO:-cargo}"
python_bin="${PYTHON:-python3}"
candid_extractor="${CANDID_EXTRACTOR:-candid-extractor}"
dfx_bin="${DFX:-dfx}"
target_root="${MIRRA_VERIFY_TARGET_ROOT:-${project_root}/target-verification}"
wasm="${target_root}/wasm32-unknown-unknown/release/mirra_canister.wasm"
failures=0

run_gate() {
  local label="$1"
  shift
  echo
  echo "== ${label} =="
  if "$@"; then
    echo "PASS: ${label}"
  else
    echo "FAIL: ${label}"
    failures=$((failures + 1))
  fi
}

build_wasm() {
  CARGO_TARGET_DIR="${target_root}" CARGO="${cargo_bin}" \
    bash "${project_root}/scripts/build-wasm.sh"
}

verify_vector_hash() {
  cd "${project_root}/tests" && sha256sum --check vectors.sha256
}

verify_certificate_hash() {
  bash "${project_root}/scripts/verify-p1-certificate.sh"
}

verify_candid() {
  CANDID_EXTRACTOR="${candid_extractor}" MIRRA_WASM="${wasm}" \
    bash "${project_root}/scripts/verify-candid.sh"
}

verify_pocket_ic() {
  MIRRA_WASM="${wasm}" timeout --kill-after=5s 240s \
    "${cargo_bin}" test --manifest-path "${project_root}/integration-tests/Cargo.toml" \
    --locked --test upgrade -- --nocapture
}

verify_reproducible_wasm() {
  local temporary
  temporary="$(mktemp -d "${TMPDIR:-/tmp}/mirra-repro.XXXXXX")" || return 1
  trap 'rm -rf -- "${temporary}"' RETURN
  local first_source="${temporary}/source-a"
  local second_source="${temporary}/nested/source-b"
  local first_target="${temporary}/target-a"
  local second_target="${temporary}/target-b"
  mkdir -p "${first_source}" "${second_source}"
  tar -C "${project_root}" \
    --exclude='.git' --exclude='target' --exclude='target-*' --exclude='.dfx' \
    -cf - . | tar -C "${first_source}" -xf -
  tar -C "${project_root}" \
    --exclude='.git' --exclude='target' --exclude='target-*' --exclude='.dfx' \
    -cf - . | tar -C "${second_source}" -xf -
  CARGO_TARGET_DIR="${first_target}" CARGO="${cargo_bin}" \
    bash "${first_source}/scripts/build-wasm.sh" || return 1
  CARGO_TARGET_DIR="${second_target}" CARGO="${cargo_bin}" \
    bash "${second_source}/scripts/build-wasm.sh" || return 1
  cmp "${wasm}" "${first_target}/wasm32-unknown-unknown/release/mirra_canister.wasm" && \
    cmp "${wasm}" "${second_target}/wasm32-unknown-unknown/release/mirra_canister.wasm"
}

verify_dfx() {
  local cargo_home
  if [[ -n "${CARGO_HOME:-}" ]]; then
    cargo_home="$(cd "${CARGO_HOME}" && pwd -P)" || return 1
  else
    cargo_home="$(cd "$(dirname "$(command -v "${cargo_bin}")")/.." && pwd -P)" || return 1
  fi
  local rustflags="--remap-path-prefix=${project_root}=/mirra/source --remap-path-prefix=${cargo_home}=/mirra/cargo"
  if [[ -n "${RUSTUP_HOME:-}" ]]; then
    local rustup_home
    rustup_home="$(cd "${RUSTUP_HOME}" && pwd -P)" || return 1
    rustflags+=" --remap-path-prefix=${rustup_home}=/mirra/rustup"
  fi
  cd "${project_root}" && RUSTFLAGS="${rustflags}" SOURCE_DATE_EPOCH=0 \
    "${dfx_bin}" --identity anonymous build --check
}

run_gate "rustc 1.88.0 pin" bash -c 'rustc --version | grep -F "rustc 1.88.0"'
run_gate "Cargo.lock present" test -s "${project_root}/Cargo.lock"
run_gate "integration Cargo.lock present" test -s "${project_root}/integration-tests/Cargo.lock"
run_gate "Python P1 smoke and grid" "${python_bin}" "${project_root}/verification/p1/exp_q32_production.py"
run_gate "99-vector deterministic regeneration" bash -c "cd '${project_root}' && '${python_bin}' verification/generate_vectors.py --check"
run_gate "99-vector SHA-256" verify_vector_hash
run_gate "P1 proof intake and withdrawal policy" verify_certificate_hash
run_gate "Rust formatting" "${cargo_bin}" fmt --manifest-path "${project_root}/Cargo.toml" --all -- --check
run_gate "Rust unit, protocol, and conformance tests" "${cargo_bin}" test --manifest-path "${project_root}/Cargo.toml" --locked
run_gate "Clippy warnings denied" "${cargo_bin}" clippy --manifest-path "${project_root}/Cargo.toml" --locked --all-targets -- -D warnings
run_gate "RustSec audits and reproducible SBOMs" bash "${project_root}/scripts/security-check.sh"
run_gate "release Wasm build" build_wasm
run_gate "packaged-versus-built Wasm" cmp "${project_root}/artifacts/mirra_canister.wasm" "${wasm}"
run_gate "generated-versus-checked-in Candid" verify_candid
run_gate "PocketIC authorization and 1-1-2 upgrade" verify_pocket_ic
run_gate "three clean byte-identical Wasm builds" verify_reproducible_wasm
run_gate "dfx build check" verify_dfx

echo
echo "== Artifact provenance =="
"${cargo_bin}" --version || true
rustc --version --verbose || true
"${dfx_bin}" --version || true
sha256sum "${project_root}/Cargo.lock" \
  "${project_root}/integration-tests/Cargo.lock" \
  "${project_root}/tests/vectors.json" \
  "${wasm}" 2>/dev/null || true

if (( failures > 0 )); then
  echo
  echo "NOT RELEASE-READY: ${failures} gate(s) failed."
  exit 1
fi
echo
echo "AUTOMATED GATES PASSED for the displayed artifact hashes."
echo "Release still requires independent review of replacement P1 mathematics/provenance and authoritative upstream review/attestation."
