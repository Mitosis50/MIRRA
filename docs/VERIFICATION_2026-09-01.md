# MIRRA RC5 verification — 2026-09-01

Status: **NOT RELEASE-READY**. This report tracks the recovered RC4
certified-snapshot increment and the separate RC5 verification changes.

## Current artifact

The sole current Wasm is `artifacts/mirra_canister.wasm` (878,189 bytes):

`acf963dadeb9eeb4c7ee5e6065045a1dcdc93af0700ce47868b5b2aa7485dfa2`

The `9171f7e7...dd53` binary was historical RC3 output carried in the RC4
archive. It is not the current certified-snapshot build or an attestation
subject for it. Any eventual attestation must bind the current source commit,
current build, and current SBOMs together.

## The three formatting safeguards

1. Commit `a2cd307702d951bad0e5ecd005514cc1373d1b2e`, titled
   `style: apply rustfmt to RC4 increment`, changes only whitespace in two
   files. Removing whitespace from each file before/after produces identical
   bytes. Inspection also confirms no string literal was changed.
2. Formatter policy (`reorder_imports = false` and the nested import skip),
   Candid ordering, SBOM normalization, and integration changes are later,
   separate commits. They are not hidden inside the formatting diff.
3. The vector corpus is byte-identical to commit
   `09284efc0975cc167a43fee80adfefe616061830`, with SHA-256
   `21c64f9ea4aef2440f3a3929497551b4d289b49fee28753d4a1f4412bef01b1b`.
   All 99 post-format Rust outputs match the frozen expected values exactly.
   This demonstrates preservation for that corpus, not a new proof over all
   possible inputs.

Changing source bytes does not universally guarantee a different compiled
binary. The digest above is measured from the actual post-format build, not
inferred from that assumption.

## Seven gates

| Gate | Status | Evidence / remaining requirement |
|---|---|---|
| 1. Genuine P1 certificate | Open | Actual bytes, original schema/checker, and provenance are absent. Hash-intake gate fails as expected. |
| 2. Rust 1.88 rebuild | Passed | Locked native tests: 6 conformance tests (including 99 vectors), 6 protocol tests; main Clippy; Wasm build; Python P1 smoke/grid. |
| 3. Candid byte equality | Passed | `candid-extractor 0.1.6` output from the current Wasm exactly equals `mirra.did`. |
| 4. PocketIC `1 → 1 → 2` | Blocked | The host previously denied PocketIC's Unix socket. The recovered harness compiles, but its runtime assertions have not executed here. |
| 5. Certificate versus trusted root | Blocked | Delegated-root verification and negative tests compile; runtime evidence is still required. No live-mainnet verification is claimed. |
| 6. Upstream review and signature | Open | Local reconstructed commits are unsigned; authoritative owner review and attestation are missing. |
| 7. Transitive crate hardening | Contained, not eliminated | Fresh scans find zero known vulnerabilities and exactly the reviewed warning set. Both normalized SBOMs reproduce; unmaintained crates still require review/replacement. |

## Additional checks

- Rust: `1.88.0 (6b00bc388 2025-06-23)`, LLVM `20.1.5`.
- Production lockfile unchanged. Recovered integration lockfile exactly matches
  the previously recorded `643c2773...15f78` digest.
- Python smoke oracle and the full 2^16-interval grid pass.
- `dfx 0.32.0 --identity anonymous build --check` passed.
- Three clean Wasm builds at different source directory depths are byte-identical
  at the current digest. The packaged Wasm also matches that build.
- Both main and integration Clippy pass with warnings denied; both formatting
  checks pass.
- RustSec commit `72f8b23d78ea6c4c9ded301a4c6ec4260e8b4c27`, containing
  1,235 advisories, reports zero known vulnerabilities in both lockfiles.
  There is one production warning and four integration warnings, exactly
  matching the policy through 2026-11-30. These are five scope entries but
  four unique crates. No exception was silently removed or extended.
- Both CycloneDX 1.5 SBOMs regenerate byte-for-byte and match after generating
  at another source path and normalizing local source URIs. Production has
  49 components; integration has 290. The test-only certificate verifier adds
  61 locked packages; no production dependencies changed.
- The first integration compile encountered an empty-object archive error in
  `ic-agent`. A single-job, non-incremental retry succeeded without changing
  dependency or application source to accommodate the error.

## Certificate harness scope

The test places MIRRA on an application subnet and requires a certificate
delegation to the local NNS root. It uses the trusted PocketIC instance's root
key only for that emulator. It recomputes every snapshot's canonical commitment,
verifies the signature/delegation/canister range/freshness through the pinned
`ic-agent`, then compares the certificate's `certified_data` leaf with that
commitment. It checks before/after upgrade and after correction 2.

Negative checks reject an altered signature, altered root key, and a canister
outside the delegated range. Certified emulator time is synchronized with the
verifier's wall clock; freshness checks are not disabled. These are compiled
tests, not passed runtime evidence on this host.

Production verification must use the trusted ICP root and intended canister ID,
not a root fetched from an untrusted proof response. A local PocketIC pass alone
would not establish that the public deployment is live or certified correctly.

`serde_cbor 0.11.2` is now also a direct dependency of the test-only certificate
decoder. Its existing unmaintained advisory remains visible in the integration
policy. It is not a production-canister dependency.

## Handoff

Run the complete verification workflow on an authorized, compatible host; then
have the authoritative maintainer review and attest the exact current artifact.
Supplying certificate bytes that merely match a hash is not a substitute for
replaying the genuine mathematical proof checker and checking provenance.
