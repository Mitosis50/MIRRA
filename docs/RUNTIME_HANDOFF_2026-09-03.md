# RC5 runtime verification handoff

This development run measures the packaged RC5 Wasm
`acf963dadeb9eeb4c7ee5e6065045a1dcdc93af0700ce47868b5b2aa7485dfa2`.
Its original source commit is `15426f130f9604a35ec40022ba05e7c8a68f2b83`.
The handoff branch changes scripts, documentation and integration tests only.
The runtime certificate root is supplied by the isolated PocketIC instance;
it is not the ICP mainnet root and a pass is not mainnet deployment evidence.

## Run on Linux x86_64

Use Rust 1.88.0, Python 3, GNU timeout and a host that allows local Unix sockets.
The runner must be able to fetch the locked Cargo dependencies. Install the
PocketIC 15.0.0 Linux x86_64 server using the download and SHA-256 verification
in `.github/workflows/runtime-evidence.yml`. The binary digest is inherited
from RC5's recorded release workflow; the script checks the downloaded bytes
before execution. The [PocketIC 15 documentation](https://docs.rs/pocket-ic/15.0.0/pocket_ic/)
documents the `POCKET_IC_BIN` configuration.

From a checkout of this handoff branch:

```bash
python3 verification/runtime_preflight.py
POCKET_IC_BIN=/absolute/path/to/pocket-ic bash scripts/verify-runtime.sh
```

The preflight does not install tools or start an emulator. Exit code 1 means
blocked. Its success only establishes prerequisites, not test success.
The runtime script validates the packaged Wasm and server hashes, compiles the
test harness, then gives the runtime test 240 seconds. It pins the install and
upgrade Wasm to the same packaged artifact. It prints the source commit, worktree
status, test-source hash, lockfile hashes and tool versions.

The `MIRRA runtime evidence` workflow provides the same run and retains the log
on success or failure. It has read-only repository permissions and does not
publish, deploy, sign or attest. This workflow is independent of the release
workflow, which must continue to reject the withdrawn P1 proof.

## Assertions in the updated harness

- Unauthorized correction returns an error and leaves health/readout unchanged.
- First accepted correction yields sequence 1.
- Identical retry before upgrade returns the original receipt fields with
  `replayed=true`, leaves one receipt/correction, and preserves readout/snapshot.
- A real canister upgrade preserves weights, sequence and state root.
- Identical retry after upgrade again returns the original receipt fields.
- The next correction yields sequence 2, a new correction ID and two receipts.
- Offline `ic-agent` verification in the harness checks certificate signature,
  delegation and freshness using the emulator root and intended canister.
  It rejects a modified signature, incorrect root and out-of-range canister;
  the certificate's certified-data leaf must match the canonical snapshot hash.

The expected terminal test marker is:

```text
MIRRA_RUNTIME_OK correction=1 retry=1 upgrade=1 retry_after_upgrade=1 next=2
```

A marker alone is not evidence: retain the complete successful process log,
source/artifact identities and workflow result. Snapshot hash recomputation
still shares the Rust canonical implementation; a separate-client packet,
independent serialization check, authenticated mainnet root and fresh real-network
certificate are additional pilot work. This handoff does not claim those are done.

## Observed limitation on September 3

The current workspace denied `AF_UNIX` socket binding with `Operation not permitted`
and has no Rust toolchain. The updated Rust harness was not compiled or executed
here. Shell/control regressions were tested separately and are not runtime proof.
The connected GitHub account was verified as `Mitosis50`; no MIRRA repository
was returned by the accessible-repository listing or MIRRA name search.

Remaining milestones: execute this workflow in the identified MIRRA repository;
review/reissue P1; complete independent real-network certificate verification;
obtain authoritative review/attestation; then connect verified evidence to Explorer.
