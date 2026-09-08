# P1 premise census and surviving bound

Status: staged review candidate, 2026-09-03. Independent reviewer decision pending.
Baseline: main commit `4ad47db0671cad3a151b4b77d6f2248f5ed84bdc`.

The baseline repository contains the corrected-bound summary in
`docs/P1_PROOF_WITHDRAWAL_2026-09-03.md`. The full calculator, derivation and
exact result were recovered from `MIRRA_PROJECT_REVIEW_PACKAGE_2026-09-03.zip`
and are preserved byte-for-byte in `review/`. There was no separate census
file in the baseline repository; this document makes the reconciliation explicit.

## Premises

“Retained” means supported by the inspected source and the reproduced analytic
candidate. It does not mean an independent reviewer has signed off.

| ID | Premise | Disposition and evidence |
| --- | --- | --- |
| P01 | Frozen arithmetic contract | Retained: `src/core.rs::exp_q32` uses S=2^32, Q=2^56, L64=12786308645202655660, N=14; integer inputs are [-16S,0]. Rust arithmetic and Python reference are byte-identical to the original RC5 archive. |
| P02 | Signed and unsigned division use nearest, ties to even | Retained: `rne_div_u` compares remainder with its complement; `rne_div_i` restores sign. The checker uses exact `round(Fraction)` and compares the actual Python kernel on 2,048 boundary/seeded inputs. This sample correspondence is not a proof of the Rust implementation. |
| P03 | Old half-sized argument domain | Rejected: conversion divides Q64 residual by 256. Safe T=L64/(512Q)+1/(2Q), approximately 0.34657359027997266, not approximately 0.17328679. Witness u=16373744595 refutes the old enclosure. |
| P04 | Discrete maximum equals ln(2)/2 | Rejected: exact maximum is 780414346016661/2251799813685248, slightly below ln(2)/2. Affine residuals and monotone rounding reduce the endpoint census to 48 candidates; the exact maximum is below safe T. |
| P05 | Every possible reduction stratum is covered | Retained: monotone RNE and domain endpoints give k=0..23. Every stratum is evaluated; no unconditional `or k_max == 23` shortcut. |
| P06 | Rounded decimal log difference is a rigorous bound | Replaced by rational enclosure: 240-term logarithm series with geometric tail, cross-checked by the 2*atanh(1/3) series. D bounds the difference from L64/2^64. |
| P07 | Horner rounding error can omit factorial propagation | Replaced by H=h*sum(T^j/j!, j=0..13). Expanding the stage error recurrence retains the factors 1/n. |
| P08 | Truncation is covered on the true argument domain | Retained in the fresh derivation: R=(T^15/15!)/(1-T/16), a positive-series tail majorant with a ratio below one. |
| P09 | Argument and split errors combine without cross terms | Replaced by G_k=E*A_k/(1-A_k), where E>=exp(T), A_k=h+kD. The exponential remainder is retained. |
| P10 | k=0 incurs generic argument-rounding/split error | Rejected: residual u*S is divisible by 256 and k*D=0, so A_0=0 exactly. This is required for the surviving tighter bound. |
| P11 | All strata share the worst output scale | Replaced by B_k=1/2+(S/2^k)*(H+R+G_k). Exact comparison over 24 cases gives the largest bound at k=0, not necessarily the largest actual error there. |
| P12 | Arithmetic intermediates and final divisor fit | Retained: Q^2*T*M and Q*M are below 2^127 for M=(1+h)/(1-T); 1/E>H+R gives positive final series. Reduction operands fit below 2^69 and final shift is at most 47. Reviewer must inspect every Rust conversion/addition against these bounds. |
| P13 | Oracle agreement means zero real error or universal correct rounding | Rejected: witness raw output 94906266 agrees with rounded oracle but has approximately 0.375579484 ulp real error. The claim is absolute error only, not correct rounding at every input. |
| P14 | An inward decimal can serve as an upper bound | Rejected: compute the ceiling with integers. The fresh bound is 0.5000000425665135563095684854886... and its outward nine-place ceiling is 0.500000043. |
| P15 | Repairing only the old decimal or old domain repairs the proof | Rejected: 0.500000066 merely ceilings the invalid old stack. Changing only its domain yields approximately 0.5000001002531813 (ceiling 0.500000101), a diagnostic rather than the fresh derivation. Neither is selected for this increment. |
| P16 | A proof digest establishes mathematical approval | Rejected: the pin identifies bytes. The original digest stays withdrawn; the new deterministic bytes are explicitly a review candidate. Review identity, scope, findings and decision remain separate. |
| P17 | P1 alone certifies the downstream TV theorem | Rejected: no total-variation theorem is issued here. `src/core.rs` uses cumulative recentering, an RNE window test, zero factors outside the window, fixed-share floor and Hamilton allocation. The public canister uses uniform priors. Each downstream inequality must explicitly account for these operations. |
| P18 | Emulator evidence closes mainnet or release provenance | Rejected: GitHub job logs establish the executed PocketIC checks only. Mainnet trusted-root verification and artifact attestation remain open. |

## Surviving claim

For every integer `x_q32` in `[-16*2^32,0]`, the candidate derivation bounds
`abs(exp_q32(x_q32) - 2^32*exp(x_q32/2^32))` by `0.500000043` output ulp.
It keeps N=14 and changes no arithmetic instructions. The exact fraction and
all 24 budgets are in the staged proof bytes.

The archived result was regenerated byte-for-byte from the hash-verified RC5
archive and original scripts ZIP. The repository adapter changes only input
loading and provenance labels; its entire `new_derivation` and witness results
must match the preserved result. Source correspondence still needs independent
review, especially signed RNE, loop indexing, overflow and the x=0 fast path.

The Python reference's historical header still advertises the old unsupported
claim. Its bytes are preserved for traceability; that header is superseded by
this census and must not be cited as proof approval.

## Pin and release consequences

The staged pin is SHA-256 of the exact UTF-8 JSON in
`MIRRA_P1_CERTIFICATE.bin`, not a certificate envelope. `check_candidate.py`
defines the deterministic JSON encoding and checks all arithmetic/source
bindings. To avoid a hash cycle, only the 64 pin digits in `src/lib.rs` are
zeroed for that file's source hash; the real compiled pin is checked separately.

Changing this pin changes protocol/snapshot identity and requires a new measured
Wasm. The checked-in RC5 Wasm remains historical baseline evidence. New workflow
builds must be labeled with their actual source commit and hash. RC5-to-new-pin
upgrade compatibility, packaging, three-path reproducibility, independent review,
mainnet verification and release attestation remain production gates.

## Reproduce

From the repository root, using Python 3.11 or later:

```bash
python3 verification/p1/derive_candidate.py
python3 verification/p1/check_candidate.py --check
python3 -m unittest discover -s verification -p 'test_p1_candidate.py'
bash scripts/verify-p1-certificate.sh
```

The final command is expected to reject this staged candidate while the review
decision is pending. An automated arithmetic pass is not a substitute for that
decision or its authenticated provenance.
