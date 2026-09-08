# Staged P1 replacement: review entry point

Start with [the premise census](PREMISE_CENSUS.md), then the preserved
[full derivation](review/MIRRA_P1_CORRECTED_BUDGET_2026-09-03.md).

This branch stages a proof-artifact pin increment using the surviving outward
bound **0.500000043 output ulp**, with **N=14**. It is pending independent review.
`MIRRA_P1_CERTIFICATE.bin` contains UTF-8 JSON, despite its historical filename.
It is neither a signature nor an attestation. The payload declares its schema,
claim, exact rational calculation, historical input hashes and source bindings.

`check_candidate.py --emit` defines canonical bytes: Python JSON with sorted
keys, two-space indentation, ASCII escaping and a final LF. `--check` regenerates
the entire payload and validates both proof-file and compiled-source pins.
Unknown fields, stale source, missing strata or altered fractions fail byte
comparison. `derive_candidate.py` adapts only the original calculator's input
loading/provenance labels; tests compare its exact arithmetic results to the
preserved historical result. No archive download is required to check the PR.

`REVIEW.json` is intentionally pending. Release intake additionally requires
an accepted decision bound to these proof bytes, the checker and an identified
reviewed commit, plus explicit scope decisions. Those fields record an external
review; they do not authenticate its author. The maintainer must verify the
review's provenance through the referenced GitHub review or retained original
review output. The release workflow remains blocked until this is resolved.

## Request for Grok Bot

The project owner plans to obtain a separate Grok Bot review. No review output,
reviewer identity, acceptance or signature has been supplied yet.

Review the exact PR head commit and record its full 40-character SHA. Start
from the actual Rust kernel and the premise census, and derive or refute the
claim before relying on the supplied calculator. Return:

1. Your model/tool version, run/session identity, date, exact reviewed commit,
   SHA-256 of the proof bytes and SHA-256 of `check_candidate.py`.
2. An accept/reject/needs-work finding for every premise P01–P18, with reasoning,
   counterexamples where applicable, and an independent calculation or checker.
3. Specific treatment of signed RNE, the 256 conversion divisor, k=0 exactness,
   all 24 strata, factorial Horner propagation, tail inequalities, and i128 safety.
4. The surviving exact fraction and an outward decimal bound. Distinguish the
   fresh derivation from the invalid old proof and its diagnostic recomputations.
5. Source correspondence and claim scope, including the x=0 fast path, clipping,
   uniform priors, normalization, and every downstream TV premise you actually
   establish. Do not infer a TV theorem from P1 or from the 99-vector suite.
6. A clear overall decision and remaining blocking findings. Attach full output,
   commands and machine-readable results; a bare “ALL PASS” is insufficient.

Use `python3 verification/p1/check_candidate.py --check` and run both
`test_p1_candidate.py` and `test_release_controls.py` with unittest discovery.
The independent review must evaluate assumptions as well as reproduce numbers.
It must not treat the author's implementation checks as independent acceptance.

After review, any changed proof/derivation/source requires regenerated proof
bytes and both pins, a fresh build, and review of the changed subject. A review
of this candidate does not approve a future candidate implicitly.

## Build evidence and remaining production work

The GitHub CI builds Wasm from this branch and tests that build. Download its
candidate artifact and verify the included hash and source commit. The checked-in
`artifacts/mirra_canister.wasm` is the old RC5 baseline and is not the staged
candidate. It must be replaced by the measured release candidate before release.

Independent acceptance, new packaged-Wasm equality, three-path reproducibility,
RC5-to-new-pin upgrade validation, actual mainnet certificate verification and
release attestation remain required. The source pin changes protocol/snapshot
identity, so it cannot be treated as a metadata-only deployment.
