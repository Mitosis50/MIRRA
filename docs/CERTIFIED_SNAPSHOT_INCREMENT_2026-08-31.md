# MIRRA certified-snapshot increment — 2026-08-31

## Outcome

This increment adds an independently reproducible public-state commitment and
exposes it through ICP certified data. It is a narrow, useful protocol moat:
integrators can pin MIRRA's protocol identity and detect substitutions in the
live sequence, expert count, state root, or frozen verification artifacts.

## Added

- `PublicSnapshot` and `CertifiedSnapshot` Candid records.
- Domain-separated protocol fingerprint and snapshot commitment.
- `public_snapshot` and `certified_snapshot` query methods.
- Certified-data refresh during initialization, upgrade, expert registration,
  and accepted correction updates.
- An independent Python commitment verifier with a successful known input and
  tamper-rejection check.
- Canonical encoding documentation and Rust sensitivity tests.

## Verification status

Passed in this workspace:

- Python syntax check for the independent verifier.
- Independent commitment reproduction for the fixed sample:
  `b606243974690bdadc90aa8778bf8d41b0a4b3dbe5119a0bfe463d83372166ae`.
- Tampered-sequence rejection.
- Git whitespace validation.

Not rerun after the workspace reset:

- Rust format, unit, conformance, Clippy, and Wasm compilation. The transient
  Rust 1.88 toolchain was removed by the reset and is not available on the
  current host. RC3 passed these gates before this source increment, but that
  evidence does not validate the new code.
- Generated-versus-checked-in Candid comparison. The checked-in interface was
  updated, but must be regenerated from a newly compiled Wasm.
- PocketIC upgrade and ICP certificate verification.

## Remaining release blockers

1. Obtain and verify the actual P1 certificate binary.
2. Compile and test this commit with frozen Rust 1.88.0.
3. Regenerate Candid from that Wasm and require byte equality.
4. Run the PocketIC `1 -> 1 -> 2` upgrade sequence on a host that permits its
   Unix socket, including restored state, authorization, receipt, and sequence
   assertions.
5. Verify the returned ICP data certificate against the canister ID and subnet
   root of trust in the integration test.
6. Review and sign the commit in the authoritative upstream repository.
7. Continue replacing formally unmaintained transitive crates.

This candidate is not release-ready until every item above passes.
