# MIRRA three-path reproducibility — 2026-09-07

**Decision:** PASSED (evidence recorded; not a release attestation)  
**Tip:** `159d41bf75e3543893396f67135e53e31c9d3f08`  
**Tip tree:** `ee002b307e0f5b9f24cbe36fcc78e318d430ca9f`  
**Reviewed executable-source baseline:** `b8c56a1744b53aa9dae656bee17bc09fc717733e` (tree `1d7bf27932859b42e2fad1736ea1bf47490d7e80`)  
**Expected / measured Wasm SHA-256 (all paths):** `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba`

## Continuity

Changes from `b8c56a1…` to tip `159d41bf…` are evidence / checksum / review / provenance / manifest-pin only.  
No source, proof, checker, Cargo, build-script, Candid, or Wasm product changes. Tip rebuilds match the reviewed Wasm bytes.

## Paths (byte-identical)

| Path | Environment | Wasm SHA-256 |
|------|-------------|--------------|
| A | GitHub Actions verification run [34170383195](https://github.com/Mitosis50/MIRRA/actions/runs/34170383195) artifact `10035602693` | `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba` |
| B | Clean Linux host (Debian 13 / rustc 1.88.0); fresh clone; fresh `CARGO_TARGET_DIR`; `scripts/build-wasm.sh` | `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba` |
| C | Fresh Rust 1.88 separate clean host env (Docker `rust:1.88.0-bookworm` unavailable; allowed fallback); fresh `CARGO_HOME` + `CARGO_TARGET_DIR`; `scripts/build-wasm.sh --locked` | `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba` |

`cmp` A↔B, A↔C, B↔C: identical.

### Path A commands
```
mkdir -p /workspace/mirra-repro/path-a
curl -sL -o /workspace/mirra-repro/path-a/artifact.zip 'https://nightly.link/Mitosis50/MIRRA/actions/runs/34170383195/mirra-p1-candidate-0c69388e024c8d13cd154be1b0fbbf5061ccd258.zip'
unzip -o /workspace/mirra-repro/path-a/artifact.zip -d /workspace/mirra-repro/path-a/extracted
sha256sum /workspace/mirra-repro/path-a/extracted/mirra_canister.wasm
cp -a extracted/mirra_canister.wasm ./mirra_canister.wasm
# Mac (machineId 8f5619b6-6eaf-4da5-b552-2a582cb7c713) for metadata/logs: gh api repos/Mitosis50/MIRRA/actions/runs/34170383195 and gh run view --log
```

### Path B commands
```
mkdir -p /workspace/mirra-repro/path-b
cd /workspace/mirra-repro/path-b && git clone https://github.com/Mitosis50/MIRRA.git src
cd src && git checkout 159d41bf75e3543893396f67135e53e31c9d3f08
git rev-parse HEAD  # -> 159d41bf75e3543893396f67135e53e31c9d3f08
git rev-parse HEAD^{tree}  # -> ee002b307e0f5b9f24cbe36fcc78e318d430ca9f
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain none
source "$HOME/.cargo/env"
rustup toolchain install 1.88.0 --profile minimal --component clippy,rustfmt
rustup target add wasm32-unknown-unknown --toolchain 1.88.0
rustup override set 1.88.0
unset RUSTC_WRAPPER SCCACHE_DIR SCCACHE CARGO_INCREMENTAL
export CARGO_TARGET_DIR=/workspace/mirra-repro/path-b/target-fresh
rm -rf "$CARGO_TARGET_DIR" && mkdir -p "$CARGO_TARGET_DIR"
export RUSTUP_TOOLCHAIN=1.88.0
bash scripts/build-wasm.sh
sha256sum "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/mirra_canister.wasm"
cp -a "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/mirra_canister.wasm" /workspace/mirra-repro/path-b/mirra_canister.wasm
```

### Path C commands (summary)
```
git checkout 159d41bf75e3543893396f67135e53e31c9d3f08
export CARGO_HOME=…/path-c/env/cargo-home
export CARGO_TARGET_DIR=…/path-c/env/target
export CARGO_INCREMENTAL=0
bash scripts/build-wasm.sh --locked
# plus certificate checker, Candid equality, 99 vectors, runtime witness
```

## Path C extra checks

All PASS: certificate (`check_candidate.py --check`), Candid equality, 99 vectors, runtime witness  
`MIRRA_RUNTIME_OK correction=1 retry=1 upgrade=1 retry_after_upgrade=1 next=2`.

## Fidelity

MIRRA Commons Reviewer independent re-hash + cross-path `cmp`: **PASS**  
(`THREE_PATH_FIDELITY` local record; caveats non-blocking: Docker→clean-host fallback; Path A BUILD_RECORD `source_commit` ≠ tip).

## Policy

- `REVIEW.json` `scope.provenance` = **needs-work** (unchanged)
- `REVIEW.json` `overall` = **needs-work** (unchanged)
- Three-path reproducibility = **passed** (recorded separately)
- Mainnet trusted-root verification = **open**
- Release attestation = **open**
- PR #8 not merge-cleared

Machine-readable twin: `docs/THREE_PATH_REPRODUCIBILITY_2026-09-07.json`
