#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
candid_extractor="${CANDID_EXTRACTOR:-candid-extractor}"
wasm="${MIRRA_WASM:-${project_root}/target/wasm32-unknown-unknown/release/mirra_canister.wasm}"

diff --unified "${project_root}/mirra.did" <("${candid_extractor}" "${wasm}")
echo "PASS: checked-in Candid exactly matches generated Wasm interface"
