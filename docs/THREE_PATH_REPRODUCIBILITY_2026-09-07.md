# MIRRA three-path reproducibility — 2026-09-07

**Decision:** NEEDS-WORK (native M1 mismatch recorded; not a release attestation)  
**three_path_reproducibility:** needs-work  
**Paths A/B evidence tip:** `159d41bf75e3543893396f67135e53e31c9d3f08`  
**Paths A/B evidence tree:** `ee002b307e0f5b9f24cbe36fcc78e318d430ca9f`  
**Reviewed executable-source baseline:** `b8c56a1744b53aa9dae656bee17bc09fc717733e` (tree `1d7bf27932859b42e2fad1736ea1bf47490d7e80`)  
**Expected / Paths A and B Wasm SHA-256:** `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba` (878189 bytes)  
**Recorded:** 2026-09-08T04:57:30Z (2026-09-07T21:57:30-0700 PT)

## Decision

`decision` and `three_path_reproducibility` remain **needs-work**. This record does not set three-path reproducibility to passed.

## Permanent failed path: `native_m1_mac`

This object is permanent negative evidence. Do not replace, rename, or delete it. A later container measurement does not cancel it.

- Environment label: `native_m1_mac`
- Host: Darwin arm64 / macOS 26.4.1 (25E253)
- rustc: rustc 1.88.0 (6b00bc388 2025-06-23)
- rustc host: `aarch64-apple-darwin`
- Tip built: `1c73473c23b264e5eda6886d6a398fee95c73c2b`
- Tree: `fb53525d125501e4be009b69cceaf3db262c8d51`
- Measured Wasm SHA-256: `9e32af00502fd7197850407a4b85ea793ccd0530936e6cc7f1495343120feac1` (878197 bytes)
- Expected Wasm SHA-256: `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba` (878189 bytes)
- `cmp`: differs at byte 896
- Both binaries preserved:
  - Path A: `/Users/aiagents/mirra-path-c-m1/artifacts/path-a-mirra_canister.wasm`
  - Native M1: `/Users/aiagents/mirra-path-c-m1/artifacts/mirra_canister.wasm`

Paths A and B still hash to `2edf7aab…ac5aba`. This native M1 path does not match.

Exact commands:

```
mkdir -p /Users/aiagents/mirra-path-c-m1/{cargo-home,target,artifacts}
git clone --branch p1-surviving-bound-stage-2026-09-03 --single-branch https://github.com/Mitosis50/MIRRA.git /Users/aiagents/mirra-path-c-m1/src
cd /Users/aiagents/mirra-path-c-m1/src && git rev-parse HEAD  # 1c73473c23b264e5eda6886d6a398fee95c73c2b
git rev-parse 'HEAD^{tree}'  # fb53525d125501e4be009b69cceaf3db262c8d51
unset RUSTC_WRAPPER SCCACHE_DIR SCCACHE CARGO_INCREMENTAL RUSTFLAGS RUSTUP_HOME
export CARGO_INCREMENTAL=0
export CARGO_HOME=/Users/aiagents/mirra-path-c-m1/cargo-home
export CARGO_TARGET_DIR=/Users/aiagents/mirra-path-c-m1/target
export PATH="$HOME/.cargo/bin:$PATH"
bash scripts/build-wasm.sh
sha256sum "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/mirra_canister.wasm"
cp -a "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/mirra_canister.wasm" /Users/aiagents/mirra-path-c-m1/artifacts/mirra_canister.wasm
cmp /Users/aiagents/mirra-path-c-m1/artifacts/path-a-mirra_canister.wasm /Users/aiagents/mirra-path-c-m1/artifacts/mirra_canister.wasm
```

`scripts/build-wasm.sh` hardcodes `cargo build --locked` and does not accept arguments.

## Continuity

Executable-source continuity from `b8c56a1…` remains evidence / checksum / review / provenance / manifest only for Paths A and B.  
This amendment commit is docs / `REVIEW.json` / `SOURCE_MANIFEST.sha256` only.

## Paths A and B

| Path | Environment | Wasm SHA-256 | Match expected? |
|------|-------------|----------------|-----------------|
| A | GitHub Actions verification run [34170383195](https://github.com/Mitosis50/MIRRA/actions/runs/34170383195) artifact `10035602693` | `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba` | yes |
| B | Clean Linux host (Debian 13 / rustc 1.88.0); fresh clone; fresh `CARGO_TARGET_DIR`; `scripts/build-wasm.sh` | `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba` | yes |

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

## Withdrawn prior same-Debian Path C

The earlier Path C labeled `fully_separate_clean_host_env` remains withdrawn as not an independent environment. It was the same Debian host/kernel as Path B. Separate `CARGO_HOME` / `CARGO_TARGET_DIR` do not count. That historical process emitted the expected hash; it does not establish three-path reproducibility. It is retained only as that withdrawal record. It is not the native M1 mismatch and must not be used to erase `native_m1_mac`.

## Policy

- `REVIEW.json` `scope.provenance` = **needs-work** (unchanged)
- `REVIEW.json` `overall` = **needs-work** (unchanged)
- Three-path reproducibility = **needs-work** (not passed)
- `native_m1_mac` mismatch = permanent failed path
- Mainnet trusted-root verification = **open**
- Release attestation = **open**
- PR #8 not merge-cleared

Machine-readable twin: `docs/THREE_PATH_REPRODUCIBILITY_2026-09-07.json` (`permanent_failed_paths.native_m1_mac`)
