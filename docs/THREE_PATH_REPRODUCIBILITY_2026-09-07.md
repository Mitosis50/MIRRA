# MIRRA three-path reproducibility — 2026-09-07

**Decision:** NEEDS-WORK (Path D linux/amd64 match recorded; Path C-native-M1 mismatch permanently retained; not a release attestation)  
**three_path_reproducibility:** needs-work  
**passed:** blocked until new-tip verification and runtime logs are verified  
**Paths A/B evidence tip:** `159d41bf75e3543893396f67135e53e31c9d3f08`  
**Paths A/B evidence tree:** `ee002b307e0f5b9f24cbe36fcc78e318d430ca9f`  
**Reviewed executable-source baseline:** `b8c56a1744b53aa9dae656bee17bc09fc717733e` (tree `1d7bf27932859b42e2fad1736ea1bf47490d7e80`)  
**Expected / Paths A and B Wasm SHA-256:** `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba` (878189 bytes)  
**Recorded:** 2026-09-08T05:21:19Z (2026-09-07T22:21:19-0700 PT)

## Decision

`decision` and `three_path_reproducibility` remain **needs-work**. This record does not set three-path reproducibility to passed. Path D-container-linux-amd64 is a MATCH and does not close the gate. **passed is blocked until new-tip verification and runtime logs are verified.**

## Permanent failed path: Path C-native-M1 (`native_m1_mac`)

This object is permanent negative evidence. Do not replace, rename, delete, or relabel it. A later container measurement does not cancel it. `environment_label` remains `native_m1_mac`. Permanent label: **Path C-native-M1**.

- Status: **MISMATCH**, permanently retained
- Environment label: `native_m1_mac` (retained; not relabeled)
- Host: Darwin arm64 / macOS 26.4.1 (25E253)
- rustc: rustc 1.88.0 (6b00bc388 2025-06-23)
- rustc host: `aarch64-apple-darwin`
- Tip built: `1c73473c23b264e5eda6886d6a398fee95c73c2b`
- Tree: `fb53525d125501e4be009b69cceaf3db262c8d51`
- Measured Wasm SHA-256: `9e32af00502fd7197850407a4b85ea793ccd0530936e6cc7f1495343120feac1` (878197 bytes)
- Expected Wasm SHA-256: `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba` (878189 bytes)
- `cmp`: differs at byte 896 (first differing byte 896)
- Both binaries preserved:
  - Path A: `/Users/aiagents/mirra-path-c-m1/artifacts/path-a-mirra_canister.wasm`
  - Native M1: `/Users/aiagents/mirra-path-c-m1/artifacts/mirra_canister.wasm`

Paths A and B still hash to `2edf7aab…ac5aba`. This native M1 path does not match. Path D does not replace this mismatch.

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

## Path D-container-linux-amd64

This is a separate measurement. It does not overwrite, replace, or relabel Path C-native-M1. Gate not closed.

- Permanent label: **Path D-container-linux-amd64**
- Status: **MATCH**, byte-identical
- Expected/observed SHA-256: `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba`
- Size: 878189
- Source commit: `b8c56a1744b53aa9dae656bee17bc09fc717733e`
- Source tree: `1d7bf27932859b42e2fad1736ea1bf47490d7e80`
- Image: `rust@sha256:4727898c104ecd2e22d780925832502faee9fe4e70581b8572af081370b315a0`
- Platform: `linux/amd64` (`uname -sm`: Linux x86_64). Not linux/arm64. Not the Debian Grok box.
- rustc host: `x86_64-unknown-linux-gnu`
- rustc: rustc 1.88.0 (6b00bc388 2025-06-23)
- cargo: cargo 1.88.0 (873a06493 2025-05-10)
- Reviewer identity: MIRRA Commons Reviewer 2558ecf1-0890-46d4-ab0e-e477f2aa067b (independent re-hash **MATCH**)
- Built: 2026-09-08T05:01:23Z to 2026-09-08T05:04:47Z (`Finished release profile [optimized] target(s) in 3m 15s`)
- Fresh checkout `/Users/aiagents/mirra-path-c-amd64/src`; fresh empty `CARGO_HOME=/tmp/cargo-home` and `CARGO_TARGET_DIR=/tmp/target` created inside the container; no host cargo cache mount; `CARGO_INCREMENTAL=0`; no sccache; source mount read-only

Commands (from the Path D linux/amd64 record):

```
docker buildx imagetools inspect rust:1.88.0  # linux/amd64 digest sha256:4727898c104ecd2e22d780925832502faee9fe4e70581b8572af081370b315a0
docker pull --platform linux/amd64 rust@sha256:4727898c104ecd2e22d780925832502faee9fe4e70581b8572af081370b315a0
git clone --branch p1-surviving-bound-stage-2026-09-03 --single-branch https://github.com/Mitosis50/MIRRA.git /Users/aiagents/mirra-path-c-amd64/src
git checkout --detach b8c56a1744b53aa9dae656bee17bc09fc717733e
git rev-parse HEAD  # b8c56a1744b53aa9dae656bee17bc09fc717733e
docker run --name mirra-path-c-amd64-run --platform linux/amd64 -v /Users/aiagents/mirra-path-c-amd64/src:/src:ro -e CARGO_HOME=/tmp/cargo-home -e CARGO_TARGET_DIR=/tmp/target -e CARGO_INCREMENTAL=0 -w /src rust@sha256:4727898c104ecd2e22d780925832502faee9fe4e70581b8572af081370b315a0 bash scripts/build-wasm.sh
```

`scripts/build-wasm.sh` hardcodes `cargo build --locked` and does not accept arguments. `wasm32-unknown-unknown` was added inside the image with `rustup target add wasm32-unknown-unknown --toolchain 1.88.0`.

Both required checks match: SHA-256 and size 878189. `cmp` identical. This match is of reviewed source baseline `b8c56a1…`, not a rebuild of this evidence-commit tip. **passed is blocked until new-tip verification and runtime logs are verified.**

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

The earlier Path C labeled `fully_separate_clean_host_env` remains withdrawn as not an independent environment. It was the same Debian host/kernel as Path B. Separate `CARGO_HOME` / `CARGO_TARGET_DIR` do not count. That historical process emitted the expected hash; it does not establish three-path reproducibility. It is retained only as that withdrawal record. It is not Path C-native-M1 and must not be used to erase `native_m1_mac`. It is not Path D-container-linux-amd64.

## Policy

- `REVIEW.json` `scope.provenance` = **needs-work** (unchanged)
- `REVIEW.json` `overall` = **needs-work** (unchanged)
- `three_path_reproducibility.decision` = **needs-work** (not passed)
- Path C-native-M1 = **MISMATCH**, permanently retained
- Path D-container-linux-amd64 = **MATCH**, byte-identical; gate not closed
- **passed is blocked until new-tip verification and runtime logs are verified**
- Mainnet trusted-root verification = **open**
- Release attestation = **open**
- PR #8 not merge-cleared

Machine-readable twin: `docs/THREE_PATH_REPRODUCIBILITY_2026-09-07.json` (`permanent_failed_paths.native_m1_mac`, `path_d_container_linux_amd64`)
