# MIRRA release verification — 2026-08-31

Historical RC2/RC3 evidence only. Its Wasm digest is superseded; see
`PROVENANCE.md` for the sole current artifact digest.

## Decision

**NOT RELEASE-READY.** Two mandatory gates remain unverified and the candidate
has not been merged into an authoritative upstream repository.

## Passing gates

- Python P1 self-test and the full 2^16 interval grid.
- Deterministic regeneration of all 99 vectors; SHA-256
  `21c64f9ea4aef2440f3a3929497551b4d289b49fee28753d4a1f4412bef01b1b`.
- Five Rust conformance tests, including all 99 vectors.
- Six Rust protocol tests covering canonical identities, frozen parameters,
  authorization, idempotency, governance transfer, stable reopen, unique
  sequences, restored roots/weights, and clean-deployment reproducibility.
- Rust formatting and Clippy with warnings denied.
- Generated Candid exactly equals `mirra.did` for the canonical Wasm.
- `dfx 0.32.0 build --check`.
- Three Wasm builds from different source-directory depths are byte-identical.
  The canonical Wasm SHA-256 is
  `9171f7e725488bf64211eb22832d35954c05d7773f36c11d7044a626fa88dd53`.
- Both Cargo lockfiles have zero known RustSec vulnerabilities at database
  commit `b331df68b3ed0e99594d259040bdcb9de3c7c8a4`; one production and four
  integration-only transitive crates are marked unmaintained.
- Both CycloneDX 1.5 SBOMs regenerate byte-for-byte.

## Fixed during this verification

The earlier reproducibility check only rebuilt from one absolute project path.
Rust panic metadata embedded Cargo registry paths, so changing `CARGO_HOME`
changed the Wasm. The canonical build now applies `--remap-path-prefix` to the
source, Cargo, and Rustup roots, and the release test builds copies at two
different directory depths before comparing all three artifacts.

## Failing or blocked gates

1. **P1 certificate — FAIL.** `verification/p1/CERTIFICATE.sha256` names
   `MIRRA_P1_CERTIFICATE.bin`, but that payload is absent. Its claimed
   `b9d1c0d5…57b` digest and proof cannot be verified.
2. **PocketIC authorization/upgrade — BLOCKED by execution host.** PocketIC
   15.0.0 starts, then its canister HTTP adapter fails to bind a Unix socket
   with `Operation not permitted`; the instance aborts after 60 seconds.
   Consequently, the canister-level non-writer rejection, count `1 -> 1 -> 2`,
   restored root/weight equality, and unique sequence IDs remain unverified in
   a real PocketIC upgrade. Native stable-memory protocol tests pass but are not
   a substitute for this gate.
3. **Authoritative provenance — OPEN.** This is a traceable local release
   candidate reconstructed from the audited handoff, not the project owner's
   original Git history. The owner must merge, review, and attest the final
   commit in the authoritative repository.

## RC3 remediation added after this run

- Fail-closed P1 certificate intake and digest verification.
- Hash-pinned dfx and PocketIC binaries in a protected-tag Ubuntu release job.
- Signed GitHub/Sigstore build-provenance and CycloneDX SBOM attestations,
  emitted only after the complete release script exits successfully.
- Exact, expiring dependency-advisory exceptions plus weekly Dependabot checks.
- A maintainer procedure for authoritative merge, branch protection, tag
  verification, and publication of upstream evidence.

These controls make the remaining work executable and auditable, but they do
not convert missing certificate bytes or an unexecuted PocketIC job into a pass.

## Release rule

Do not publish this build as release-ready until the missing certificate is
supplied and verified, the included PocketIC test passes on a compatible host,
and the exact source and artifact hashes are attested in the upstream release.
