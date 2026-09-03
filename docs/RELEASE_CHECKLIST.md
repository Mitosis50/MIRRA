# Release checklist

No single passing unit test is a release declaration. Every required row must
be green for the exact source commit and artifact digest being published.

| Gate | Required evidence |
|---|---|
| Python P1 | smoke oracle and 2^16 grid pass |
| P1 evidence | independently reviewed replacement proof and provenance; replacement digest bound to rebuilt candidate; withdrawn `b9d1c0d5…57b` remains rejected |
| Vectors | regenerated file is byte-identical; SHA-256 is `21c64f9e…01b1b` |
| Rust | all unit, protocol, and 99-vector tests pass |
| Lint | Clippy passes with `-D warnings`; formatting is clean |
| Dependency security | both lockfiles have zero known RustSec vulnerabilities; informational warnings are reviewed |
| Dependency exceptions | exact advisory/crate/version allowlist is unexpired and no unexpected warning exists |
| SBOM | pinned CycloneDX 1.5 files regenerate byte-for-byte |
| Wasm | pinned Rust 1.88.0 builds `wasm32-unknown-unknown` with `Cargo.lock` |
| Candid | extraction from the exact Wasm equals `mirra.did` |
| Artifact identity | packaged Wasm equals the current rebuilt Wasm; historical digests are not current attestation subjects |
| Certificate root | snapshot commitment is bound to a valid, fresh certificate for the intended canister under the trusted root and delegation; tampering fails |
| Authorization | a non-writer correction is rejected without state change |
| Idempotency | identical retries retain sequence 1; conflict rejects |
| Upgrade | PocketIC reports counts `1 -> 1 -> 2` |
| Restore | state root and weights match immediately across upgrade |
| Sequence | correction records contain unique sequence IDs 1 and 2 |
| Governance | two-step transfer revokes old governance and audit events persist |
| Reproducibility | three clean Wasm builds from different source-path depths match byte-for-byte |
| State/transactions | two clean deployments return identical receipts, records, roots, and weights |
| dfx | dfx 0.32.0 `build --check` passes |
| Provenance | source commit, source manifest, both lockfiles, SBOMs, audit database revision, tool versions, vectors, certificate, and Wasm hashes are recorded |
| Upstream review | authoritative owner review and signed attestation cover the exact source and current artifact, not only a local reconstructed bundle |

## Known external blockers in the supplied handoff

1. Original P1 proof bytes were recovered, but the argument enclosure is invalid.
   The original digest is withdrawn. Corrected mathematics and replacement
   evidence need independent review and binding to a newly measured candidate.
2. The handoff was a Markdown report rather than the real Git repository. This
   candidate must be merged into the authoritative repository and verified at a
   named commit.
3. PocketIC requires a host that permits its local Unix-socket endpoint. The
   included integration test is the release gate; a native stable-memory reopen
   test is useful evidence but is not a substitute for that gate.

The protected-tag workflow in `.github/workflows/release-attestation.yml`
provides the required compatible-host test and signed provenance path after the
candidate is merged upstream and independently reviewed replacement P1 evidence
is bound to that candidate. A runtime development pass does not close P1.
