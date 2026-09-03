# MIRRA v1 release candidate

MIRRA is an ICP canister that converts a stream of externally scored expert
corrections into deterministic, exact-sum trust weights. It is intended as a
small infrastructure primitive for AI ensembles, oracle aggregation, DAO
review panels, and reproducible research workflows.

This tree closes the code-level gaps found in the audited handoff:

- the Rust kernel conforms bit-for-bit to all 99 frozen vectors;
- the learning rate is frozen at `1/8` and losses are unsigned Q32.32 in
  inclusive `[0,1]`;
- experts have domain-separated canonical SHA-256 identities;
- correction updates use global event keys and durable idempotency receipts;
- state lives in versioned `ic-stable-structures` maps and cells;
- governance controls role-scoped writers through an audited two-step transfer;
- native reopen, clean-deployment, Wasm, Candid, and PocketIC upgrade harnesses
  are included.
- both lockfiles have RustSec audit evidence and reproducible CycloneDX 1.5
  SBOMs; the Apache-2.0 license and security policy are included.

## Current release status

This is a production-oriented candidate, not a release declaration. The original
P1 proof bytes have been recovered and hash-matched, but the proof's argument
enclosure is invalid. Its digest is now blocked by the release intake gate.
A corrected derivation is a review candidate, not an issued certificate.
See [the withdrawal record](docs/P1_PROOF_WITHDRAWAL_2026-09-03.md).
A real PocketIC upgrade harness is included, but it must pass on a host that
permits PocketIC's local Unix-socket endpoint.

The current Wasm digest and its verification evidence are recorded in
[docs/PROVENANCE.md](docs/PROVENANCE.md). Older dated reports describe historical
artifacts, not a second current build.

Do not label a build release-ready until `scripts/verify-release.sh` exits zero
and replacement P1 mathematics, certificate provenance, authoritative review,
and signed upstream attestation are verified. A passing hash-intake check alone
does not verify a mathematical proof.

## Quick verification

Requirements: Rust 1.88.0, Python 3, `candid-extractor` 0.1.6, PocketIC 15.0.0
(downloaded automatically by its Rust client), and dfx 0.32.0.

```bash
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
python3 verification/p1/exp_q32_production.py
python3 verification/generate_vectors.py --check
bash scripts/build-wasm.sh
CANDID_EXTRACTOR=candid-extractor bash scripts/verify-candid.sh
MIRRA_WASM="$PWD/target/wasm32-unknown-unknown/release/mirra_canister.wasm" \
  cargo test --manifest-path integration-tests/Cargo.toml --locked --test upgrade
```

The exhaustive, accumulating release runner is:

```bash
bash scripts/verify-release.sh
```

The canister also exposes a [certified public snapshot](docs/CERTIFIED_PUBLIC_SNAPSHOT.md)
that binds live state to the frozen protocol and verification artifacts using
ICP certified data.

See [docs/PROTOCOL.md](docs/PROTOCOL.md) for consensus semantics and
[docs/RELEASE_CHECKLIST.md](docs/RELEASE_CHECKLIST.md) for the remaining gates.
The authoritative maintainer procedure and signed GitHub provenance workflow
are documented in [docs/UPSTREAM_ATTESTATION.md](docs/UPSTREAM_ATTESTATION.md).

The [runtime handoff](docs/RUNTIME_HANDOFF_2026-09-03.md) runs the packaged RC5
Wasm separately from release approval while replacement P1 review is pending.
