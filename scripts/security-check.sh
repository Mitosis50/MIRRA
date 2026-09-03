#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cargo_bin="${CARGO:-cargo}"
main_check="${project_root}/.mirra-canister-sbom.check.json"
integration_check="${project_root}/integration-tests/.mirra-integration-sbom.check.json"

cleanup() {
  rm -f -- "${main_check}" "${integration_check}"
}
trap cleanup EXIT

"${cargo_bin}" audit --file "${project_root}/Cargo.lock"
"${cargo_bin}" audit --file "${project_root}/integration-tests/Cargo.lock"
"${cargo_bin}" audit --json --file "${project_root}/Cargo.lock" > "${project_root}/security/cargo-audit-main.json"
"${cargo_bin}" audit --json --file "${project_root}/integration-tests/Cargo.lock" > "${project_root}/security/cargo-audit-integration.json"
python3 "${project_root}/verification/check_audit.py"

SOURCE_DATE_EPOCH=0 "${cargo_bin}" cyclonedx \
  --manifest-path "${project_root}/Cargo.toml" \
  --format json --spec-version 1.5 --target wasm32-unknown-unknown \
  --override-filename .mirra-canister-sbom.check
SOURCE_DATE_EPOCH=0 "${cargo_bin}" cyclonedx \
  --manifest-path "${project_root}/integration-tests/Cargo.toml" \
  --format json --spec-version 1.5 \
  --override-filename .mirra-integration-sbom.check

python3 "${project_root}/verification/normalize_sbom.py" \
  "${main_check}" "${project_root}"
python3 "${project_root}/verification/normalize_sbom.py" \
  "${integration_check}" "${project_root}"

cmp "${project_root}/security/mirra-canister.cdx.json" "${main_check}"
cmp "${project_root}/security/mirra-integration.cdx.json" "${integration_check}"
echo "PASS: both lockfiles have no known vulnerabilities and both SBOMs reproduce"
