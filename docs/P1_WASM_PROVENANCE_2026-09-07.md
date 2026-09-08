# MIRRA P1 Wasm provenance reconciliation

Observed: 2026-09-07 UTC

Status: **evidence recorded; not independent acceptance or release attestation**

## Binding

| Subject | Identity |
| --- | --- |
| Grok-reviewed commit | `b8c56a1744b53aa9dae656bee17bc09fc717733e` |
| Reviewed Git tree | `1d7bf27932859b42e2fad1736ea1bf47490d7e80` |
| GitHub Actions merge commit | `82bda8ac5f2cf328ca54d1ea4086311e4fc4d27d` |
| CI Git tree | `1d7bf27932859b42e2fad1736ea1bf47490d7e80` |
| Candidate Wasm SHA-256 | `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba` |
| P1 proof SHA-256 | `37f218d3fb9695dbc62cd60955337067b6e55fa45a4d8c0a93ef8415278a3a2d` |
| Checker SHA-256 | `5fa62f9a54d4d06755c256e52d78e007b59929af30200401fd13956e541b5951` |

The GitHub Actions merge commit is one commit ahead of the reviewed head but changes
no files relative to it. Both commit identities resolve to the same tree. The comparison
is recorded at:

https://github.com/Mitosis50/MIRRA/compare/b8c56a1744b53aa9dae656bee17bc09fc717733e...82bda8ac5f2cf328ca54d1ea4086311e4fc4d27d

## Workflow evidence

The conclusions below were checked from GitHub's run/job records and decoded job
logs, not inferred from a merge message.

- [Verification run 33723910513](https://github.com/Mitosis50/MIRRA/actions/runs/33723910513),
  job `100548609991`: completed successfully on the reviewed head. Its log records
  the 99-vector byte-for-byte pass, vulnerability scan pass, Candid equality pass,
  PocketIC runtime sequence, artifact packaging and measured Wasm digest.
- [Runtime run 33723910558](https://github.com/Mitosis50/MIRRA/actions/runs/33723910558),
  job `100548610042`: completed successfully on the reviewed head. Its log records
  candidate digest `2edf7a…ac5aba` and
  `MIRRA_RUNTIME_OK correction=1 retry=1 upgrade=1 retry_after_upgrade=1 next=2`.

## Artifact verification

Artifact `9881447880`,
`mirra-p1-candidate-82bda8ac5f2cf328ca54d1ea4086311e4fc4d27d`,
was freshly downloaded. Every entry in its `SHA256SUMS` passed:

- `mirra_canister.wasm`: `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba`
- `mirra.did`: `b9f031d8dc354d93935b1bc29001b875a3616cc55a7dfc95ec3ccae6250765f8`
- `MIRRA_P1_CERTIFICATE.bin`: `37f218d3fb9695dbc62cd60955337067b6e55fa45a4d8c0a93ef8415278a3a2d`

The artifact's `BUILD_RECORD.json` names merge commit `82bda8…`, the shared
tree `1d7bf2…`, workflow run `33723910513`, and Rust 1.88.0.

## Remaining gates

This closes the missing repository record for the available CI Wasm provenance.
It does **not** change Grok's overall `needs-work` decision. Independent acceptance
must re-evaluate compiled correspondence against this binding.

Still open:

- independent P1 acceptance of the exact source, proof, checker and Wasm;
- three-clean-path reproducibility;
- RC5-to-new-pin upgrade evidence;
- ICP mainnet verification under a trusted root;
- signed release attestation.
