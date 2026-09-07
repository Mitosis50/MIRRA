# P1 compiled-correspondence independent acceptance request

This packet requests an independent review decision. It does not itself constitute
acceptance.

## Immutable review subject

| Field | Value |
| --- | --- |
| Source commit | `b8c56a1744b53aa9dae656bee17bc09fc717733e` |
| Source tree | `1d7bf27932859b42e2fad1736ea1bf47490d7e80` |
| CI merge commit | `82bda8ac5f2cf328ca54d1ea4086311e4fc4d27d` |
| CI Wasm SHA-256 | `2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba` |
| Proof SHA-256 | `37f218d3fb9695dbc62cd60955337067b6e55fa45a4d8c0a93ef8415278a3a2d` |
| Checker SHA-256 | `5fa62f9a54d4d06755c256e52d78e007b59929af30200401fd13956e541b5951` |
| Workflow artifact | `9881447880` |
| Verification run | https://github.com/Mitosis50/MIRRA/actions/runs/33723910513 |
| Runtime run | https://github.com/Mitosis50/MIRRA/actions/runs/33723910558 |

The reviewed source commit and CI merge commit are distinct commit identities but
resolve to the same Git tree. Their comparison reports zero changed files.

## Required independent execution

Use a clean environment. Do not rely only on the author's workflow conclusions.

```bash
git clone https://github.com/Mitosis50/MIRRA.git mirra-p1-independent
cd mirra-p1-independent
git checkout --detach b8c56a1744b53aa9dae656bee17bc09fc717733e

test "$(git rev-parse HEAD^{tree})" =   "1d7bf27932859b42e2fad1736ea1bf47490d7e80"

sha256sum verification/p1/MIRRA_P1_CERTIFICATE.bin   verification/p1/check_candidate.py

python3 verification/p1/check_candidate.py --check
python3 -m unittest discover -s verification -p 'test_p1_candidate.py'
python3 -m unittest discover -s verification -p 'test_release_controls.py'
python3 verification/generate_vectors.py --check
(cd tests && sha256sum --check vectors.sha256)

cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
bash scripts/build-wasm.sh
sha256sum target/wasm32-unknown-unknown/release/mirra_canister.wasm
```

The clean build must produce:

```text
2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba
```

Download and verify the separately preserved CI artifact:

```bash
mkdir ../mirra-p1-artifact
gh run download 33723910513   --repo Mitosis50/MIRRA   --name mirra-p1-candidate-82bda8ac5f2cf328ca54d1ea4086311e4fc4d27d   --dir ../mirra-p1-artifact
(cd ../mirra-p1-artifact && sha256sum --check SHA256SUMS)
cmp target/wasm32-unknown-unknown/release/mirra_canister.wasm   ../mirra-p1-artifact/mirra_canister.wasm
```

Then run the compiled candidate through the upgrade/retry harness:

```bash
MIRRA_WASM="$PWD/target/wasm32-unknown-unknown/release/mirra_canister.wasm"   cargo test --manifest-path integration-tests/Cargo.toml   --locked --test upgrade -- --nocapture
```

Expected semantic witness:

```text
MIRRA_RUNTIME_OK correction=1 retry=1 upgrade=1 retry_after_upgrade=1 next=2
```

## Acceptance rule

Submit **needs-work** if any identity, clean build, artifact comparison, compiled
test, or runtime check fails or cannot be performed.

Submit **accepted** for compiled correspondence only if all required checks pass.
Do not broaden that decision to mainnet verification, three-path reproducibility,
or release attestation.

A PR conversation comment is evidence but is not a formal GitHub approval. The
reviewer must submit a formal `APPROVE` or `REQUEST_CHANGES` review and include
all fields below in its review body:

```json
{
  "schema": "mirra.p1.compiled-correspondence-review.v1",
  "decision": "accepted-or-needs-work",
  "reviewer_identity": "independent reviewer identity",
  "reviewed_source_commit": "b8c56a1744b53aa9dae656bee17bc09fc717733e",
  "reviewed_source_tree": "1d7bf27932859b42e2fad1736ea1bf47490d7e80",
  "reviewed_wasm_sha256": "2edf7aaba7e1e4eaf781349dc99e3425257d6f777b4d333c509c9538d2ac5aba",
  "proof_sha256": "37f218d3fb9695dbc62cd60955337067b6e55fa45a4d8c0a93ef8415278a3a2d",
  "checker_sha256": "5fa62f9a54d4d06755c256e52d78e007b59929af30200401fd13956e541b5951",
  "clean_build_wasm_sha256": "measured value",
  "artifact_sha256sums_passed": true,
  "compiled_tests_passed": true,
  "runtime_sequence_passed": true,
  "reviewed_at_utc": "RFC3339 timestamp",
  "limitations": [
    "not mainnet verification",
    "not release attestation"
  ]
}
```

Any source, proof, checker, build-input or Wasm change invalidates the decision
and requires a new review.
