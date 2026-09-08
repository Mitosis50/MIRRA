# MIRRA three-path reproducibility — 2026-09-07

**Decision:** NEEDS-WORK (pending genuine third host; not a release attestation)  
**Tip of prior evidence record:** `159d41bf75e3543893396f67135e53e31c9d3f08`  
**Tip tree of prior evidence record:** `ee002b307e0f5b9f24cbe36fcc78e318d430ca9f`  
**Reviewed executable-source baseline:** `b8c56a1744b53aa9dae656bee17bc09fc717733e` (tree `1d7bf27932859b42e2fad1736ea1bf47490d7e80`)  
**Expected Wasm SHA-256:** `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba`

## Correction

The earlier **passed** decision is withdrawn.

Reason: prior Path C labeled `fully_separate_clean_host_env` was the same Debian host/kernel as Path B (`Linux cursor 6.12.94+`, Debian GNU/Linux 13). Separate `CARGO_HOME` / `CARGO_TARGET_DIR` do not count as an independent third environment. Path C is pending an M1 Mac (or pinned container) rebuild.

This correction does not claim mainnet trusted-root verification or release attestation, and does not clear PR #8 to merge.

## Continuity

Changes from `b8c56a1…` to evidence tip `159d41bf…` are evidence / checksum / review / provenance / manifest-pin only.  
No source, proof, checker, Cargo, build-script, Candid, or Wasm product changes. Paths A and B matched the reviewed Wasm bytes. That does not establish three-path reproducibility.

## Paths

| Path | Environment | Independent third host? | Wasm SHA-256 |
|------|-------------|-------------------------|--------------|
| A | GitHub Actions verification run [34170383195](https://github.com/Mitosis50/MIRRA/actions/runs/34170383195) artifact `10035602693` | n/a (path A) | `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba` |
| B | Clean Linux host (Debian 13 / rustc 1.88.0); fresh clone; fresh `CARGO_TARGET_DIR`; `scripts/build-wasm.sh` | n/a (path B) | `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba` |
| C | Withdrawn. Same Debian host/kernel as Path B. Fresh `CARGO_HOME` + `CARGO_TARGET_DIR` only. Docker `rust:1.88.0-bookworm` was not used. | **No** | historical process emitted `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba`; does not count as a third path |

`cmp` of those three binaries showed identical bytes. That result is historical only. It is not a three-path pass.

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

### Path C (withdrawn; not an independent host)
```
# Same Debian host/kernel as Path B. Not a genuine third environment.
git checkout 159d41bf75e3543893396f67135e53e31c9d3f08
export CARGO_HOME=…/path-c/env/cargo-home
export CARGO_TARGET_DIR=…/path-c/env/target
export CARGO_INCREMENTAL=0
bash scripts/build-wasm.sh --locked
```

Pending replacement: native M1 Mac rebuild, or a pinned container on that Mac if rustup cannot be installed there. Do not reuse the Grok Debian box for Path C.

## Path C extra checks

The withdrawn same-host process also ran certificate checker, Candid equality, 99 vectors, and a runtime witness. Those checks do not repair the missing independent host. They are not a three-path pass.

## Fidelity

The prior reviewer fidelity **PASS** is withdrawn. Independent re-hash showed matching bytes, but Path C was not a separate host. Three-path reproducibility remains **needs-work**.

## Policy

- `REVIEW.json` `scope.provenance` = **needs-work** (unchanged)
- `REVIEW.json` `overall` = **needs-work** (unchanged)
- Three-path reproducibility = **needs-work** (pending genuine third host)
- Mainnet trusted-root verification = **open**
- Release attestation = **open**
- PR #8 not merge-cleared

Machine-readable twin: `docs/THREE_PATH_REPRODUCIBILITY_2026-09-07.json`
