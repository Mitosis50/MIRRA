# MIRRA security policy

## Supported release line

MIRRA is currently a release candidate. No version should be represented as a
production release until every gate in `docs/RELEASE_CHECKLIST.md` passes for a
named Git commit and exact Wasm digest.

## Reporting a vulnerability

Do not open a public issue containing exploit details, private principals,
deployment configuration, or correction data. Contact the repository owner
through its private security-reporting channel and include:

- affected commit, version, Candid method, and Wasm digest;
- reproduction steps and the caller roles required;
- whether stable state, idempotency, authorization, or deterministic arithmetic
  is affected;
- the smallest non-sensitive test case that demonstrates the problem.

The authoritative repository must replace this section with an operated contact
method before public deployment.

## Security invariants

- Unauthorized callers cannot record corrections or retrieve retry receipts
  through the update path.
- One canonical event key can commit at most one canonical payload.
- Active expert membership cannot change after sequence 1.
- Every accepted correction advances the sequence exactly once.
- Governance transfer is two-step and revokes the former governance principal.
- Consensus arithmetic uses checked integers only; no floating-point values
  enter the Wasm compute path.
- Stable schema and state-root length are validated after upgrade.

## Release supply chain

The repository pins Rust, direct Rust dependencies, PocketIC, dfx, and both
Cargo lockfiles. Release CI must verify formatting, Clippy, unit and conformance
tests, vulnerability audits, CycloneDX SBOMs, Candid parity, deterministic Wasm,
the P1 certificate, and the PocketIC upgrade path.

Informational RustSec warnings are documented rather than hidden. They do not
currently represent known vulnerabilities, but upstream replacements should be
tracked.
