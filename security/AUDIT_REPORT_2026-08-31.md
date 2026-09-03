# Dependency and SBOM audit

Historical evidence only. See `AUDIT_REPORT_2026-09-01.md` for the current
lockfiles, audit database revision, SBOMs, and dependency scope.

Date: 2026-08-31

Tools:

- `cargo-audit 0.22.2`
- RustSec database commit `b331df68b3ed0e99594d259040bdcb9de3c7c8a4`
- RustSec database timestamp `2026-08-29T08:11:09+02:00`
- `cargo-cyclonedx 0.5.9`, CycloneDX 1.5 JSON

## Results

| Dependency surface | Vulnerabilities | Informational warnings | SBOM components |
|---|---:|---:|---:|
| Production canister lockfile | 0 | 1 unmaintained crate | 49 |
| PocketIC integration lockfile | 0 | 4 unmaintained crates | 229 |

Production warning:

- `RUSTSEC-2024-0436`: `paste 1.0.15` is unmaintained. It is transitive through
  `candid 0.10.35`, not used directly by MIRRA.

Integration-only warnings:

- `RUSTSEC-2025-0012`: `backoff 0.4.0` is unmaintained.
- `RUSTSEC-2024-0384`: `instant 0.1.13` is unmaintained through `backoff`.
- `RUSTSEC-2024-0436`: `paste 1.0.15` is unmaintained through Candid.
- `RUSTSEC-2021-0127`: `serde_cbor 0.11.2` is unmaintained.

The integration-only warnings are transitive through `pocket-ic 15.0.0`.
Neither lockfile contains a RustSec vulnerability as of the database revision
above. “No known vulnerability” is time-bounded evidence, not a permanent
security guarantee.

The exact advisory IDs, crate names, and versions are allowlisted in
`verification/dependency-exceptions.json` through 2026-11-30. CI fails for any
new advisory, changed version, vulnerability, or expired review window.

SBOM generation emitted a metadata warning because `ic-cdk-executor` publishes
a non-RFC-3986 `links` string. Dependency resolution completed and the SBOMs are
valid CycloneDX JSON; this is upstream package metadata, not a MIRRA runtime
finding.

## Artifact hashes

| Artifact | SHA-256 |
|---|---|
| Production audit JSON | `317d6412e93eeb8c92301e085ecdbe240fa6fba5725a6fb5302b806ea440dfd4` |
| Integration audit JSON | `2e3713246db341b2efbfd1e96c8edaa6030b072c3ddff2850679e008cd2103a1` |
| Production CycloneDX SBOM | `e5f9317ddb450381e19438258cad8146a2592ef3c102d2494a99deb00846a567` |
| Integration CycloneDX SBOM | `7439000ab523dd1a1f5256137246bdf9ba9452f798c9c2809808bf1b56d03f72` |
