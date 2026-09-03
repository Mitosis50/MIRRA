# Dependency and SBOM audit — 2026-09-01

The recovered candidate has zero known RustSec vulnerabilities in both
lockfiles. This is dated scan evidence, not a security guarantee or a claim
that unmaintained dependencies have been removed.

Tools: `cargo-audit 0.22.2`, `cargo-cyclonedx 0.5.9`, CycloneDX 1.5 JSON.
RustSec database: `72f8b23d78ea6c4c9ded301a4c6ec4260e8b4c27`, timestamp
`2026-09-01T09:55:13+02:00`, 1,235 advisories. The two machine-readable audit
reports contain that database revision.

| Scope | Locked packages | Known vulnerabilities | Unmaintained warnings | SBOM components |
|---|---:|---:|---:|---:|
| Main lockfile | 65 | 0 | 1 | 49 |
| Integration lockfile | 324 | 0 | 4 | 290 |

The production warning is `paste 1.0.15`, `RUSTSEC-2024-0436`.
Integration warnings are `backoff 0.4.0` (`RUSTSEC-2025-0012`),
`instant 0.1.13` (`RUSTSEC-2024-0384`), `paste 1.0.15`
(`RUSTSEC-2024-0436`), and `serde_cbor 0.11.2` (`RUSTSEC-2021-0127`).

The exact advisory/crate/version policy passes and remains due for review by
2026-11-30. The warning entries were neither suppressed nor extended. There
are five scope entries, covering four unique crates.

The integration certificate verifier adds 61 locked packages without changing
the production lockfile. `serde_cbor` was already transitive through PocketIC
and is now also used directly to decode certificates in the test harness.
That direct test-only dependency is included in the existing exception, not
treated as maintained or silently omitted from the scan.

## SBOM reproducibility

Local Cargo source URIs are normalized to `file:///mirra/source`. Both SBOMs
then regenerate byte-identically, including in a separate source directory at
a different depth. All dependency package/version references remain intact.

The generator still reports upstream `ic-cdk-executor` metadata containing an
invalid links URI. This warning is recorded; generation and comparison pass.

| Artifact | SHA-256 |
|---|---|
| Main audit JSON | `3b3224edb13e2be1ccad0307d6bc88bb2ab725abf494c9e1721730afb58f973e` |
| Integration audit JSON | `f5ca74f546693a9cf728e1ef08c7204c0415e8917841991f0d3b92418eb90a6e` |
| Main SBOM | `6dadf58bfda8891fa167bd5ca1af83c22d8bdb235e68b802718eac49a4ff027b` |
| Integration SBOM | `4616160906d6f9772b2ab65ee4eb1708b8cfabffb28ac86546695f3127d97357` |

Gate 7 is contained, not eliminated: keep the fail-closed exception policy and
review replacements before expiry. Do not describe this scan as proof that
the broader release, PocketIC runtime, or live certificate verification passed.
