# MIRRA P1 independent review — formal decision

**Reviewer:** MIRRA Reviewer (Grok Bot agent `2558ecf1-0890-46d4-ab0e-e477f2aa067b`)  
**Tool/session:** Grok Bot box desktop assistant; Python 3.13.5; host Linux 6.12.94+ (`cursor`/`box`)  
**Date:** 2026-09-06 07:24 PT / 14:24 UTC  
**Repo:** Mitosis50/MIRRA  
**PR context:** https://github.com/Mitosis50/MIRRA/pull/8 (draft)  
**Exact reviewed commit:** `b8c56a1744b53aa9dae656bee17bc09fc717733e`  
**Tree materialization:** GitHub archive zip of that commit (folder `MIRRA-b8c56a1744b53aa9dae656bee17bc09fc717733e`)  

**Proof bytes SHA-256:** `37f218d3fb9695dbc62cd60955337067b6e55fa45a4d8c0a93ef8415278a3a2d`  
**`check_candidate.py` SHA-256:** `5fa62f9a54d4d06755c256e52d78e007b59929af30200401fd13956e541b5951`  
**Compiled `P1_CERT_SHA256` in `src/lib.rs`:** matches proof bytes hash  

**Commands re-run on this tree (exit codes):**
- `python3 verification/p1/check_candidate.py --check` → PASS (0)
- `python3 -m unittest discover -s verification -p 'test_p1_candidate.py'` → OK, 5 tests (0)
- `python3 -m unittest discover -s verification -p 'test_release_controls.py'` → OK, 6 tests (0)
- `bash scripts/verify-p1-certificate.sh` → FAIL pending review (1) — expected

Author CI is **not** treated as independent acceptance. CI Wasm artifact was **not** independently verified in this session (prior download failure on box; no live `gh` auth).

Supporting audits (local): `math_audit.md`, `source_audit.md`, `integer_safety_audit.md`.

---

## Overall decision

**NEEDS-WORK** for release intake / full `REVIEW.json` acceptance.

**Mathematics of the staged absolute-error candidate (bound `0.500000043` ulp, N=14): ACCEPT** under the RNE Horner analytic model with standard series lemmas and the frozen arithmetic contract.

This decision does **not** merge, push main, change rules, claim mainnet/release attestation, or close production gates. Patterns remain proposals until human reconciliation of `REVIEW.json` / owner process.

### Proposed scope fields (for owner reconciliation — not written into the repo by this reviewer)

| Scope axis | Proposed | Rationale |
| --- | --- | --- |
| mathematics | **accepted** | Exact-rational recomputation matches certificate; outward ceiling verified |
| source_correspondence | **needs-work** | Structure matches; sample Python↔Rust-logic OK; no compiled Rust/Wasm execution here |
| integer_safety | **accepted** | P12 inequalities and domain `k∈0..23` / shift≤47 verified; residual hardening notes only |
| scope_and_downstream | **accepted** | Claim correctly excludes universal CR and TV; reviewer agrees P1 alone establishes neither |
| provenance | **needs-work** | Commit/archive pins and checker verified locally; CI artifact + mainnet attestation not verified |

---

## Surviving claim (verified)

For every integer `x_q32` in `[-16·2^32, 0]`, under the RNE Horner model with frozen `S,Q,L64,N=14`:

`|exp_q32(x) − 2^32·exp(x/2^32)| ≤ 0.5000000425665135563095684854886… < 0.500000043` output ulp.

Worst analytic budget uniquely at **k=0** with `A_0 = G_0 = 0`. Fresh derivation ≠ withdrawn ~0.500000066 and ≠ diagnostic-only ~0.500000101.

---

## Premise findings P01–P18

| ID | Finding | Reasoning |
| --- | --- | --- |
| P01 Frozen arithmetic contract | **accept** | `S=2^32`, `Q=2^56`, `L64=12786308645202655660`, `N=14`, domain `[-16S,0]` match Rust/Python/certificate |
| P02 Signed/unsigned RNE = nearest, ties-even | **needs-work** | Algorithm matches on samples + Fraction ties; not an exhaustive compiled-Rust proof |
| P03 Old half-sized domain | **reject** | Conversion `/256` ⇒ safe `T≈0.34657`, not ~0.17328; witness refutes old enclosure |
| P04 Discrete max = ln(2)/2 | **reject** | Exact max `780414346016661/2251799813685248 < ln2/2` |
| P05 All strata covered | **accept** | `k∈0..23`; all 24 `B_k` present; endpoint census rechecked |
| P06 Rounded decimal log diff | **reject** (old method); **accept** rational `D` replacement | Series enclosure + secondary atanh cross-check |
| P07 Horner without factorial propagation | **reject** (old); **accept** `H=h·Σ T^j/j!` | |
| P08 Truncation on true domain | **accept** | `R=(T^15/15!)/(1−T/16)` valid for `T<16` |
| P09 No cross terms | **reject** (old); **accept** `G_k=E·A_k/(1−A_k)` | |
| P10 k=0 incurs generic arg/split error | **reject** | `A_0=G_0=0` exactly (divisibility by 256); required for tightness |
| P11 Shared worst output scale | **reject** (old); **accept** per-k `B_k` (max at k=0) | |
| P12 Integer headroom | **accept** | `Q²TM` and `QM` ≪ `2^127`; shift ≤47; residual unchecked non-Horner ops noted |
| P13 Oracle agree ⇒ zero/CR | **reject** | Witness `u=16373744595` real error ≈0.375579484 ulp |
| P14 Inward decimal as upper bound | **reject** | Outward ceiling `0.500000043` |
| P15 Patch old decimal/domain only | **reject** | 0.500000066 / 0.500000101 are not the fresh derivation |
| P16 Proof digest = mathematical approval | **reject** | Pin identifies bytes only |
| P17 P1 ⇒ TV theorem | **reject** | Downstream recentering/window/Hamilton/priors not established here |
| P18 Emulator ⇒ mainnet/release | **reject** | Mainnet trusted-root + attestation remain open |

---

## 1. Mathematics

Independent `Fraction` recomputation of `T`, `D`, `H`, `R`, `E`, all 24 `B_k`, global bound, and nine-place outward ceiling **exact-matches** the staged certificate. No fracture found in the fresh N=14 stack. Gaps: checker self-consistency ≠ independent math truth; series lemmas treated as standard analysis; absolute error only.

## 2. Source correspondence

Rust `exp_q32` implements the same reduction / signed residual `/256` / N=14 Horner / final `2^(24+k)` RNE structure as the budget. `x=0` fast path returns `S` (exact, error 0), inside the claim and outside nontrivial series budgeting. Domain is hard reject, not soft clip. Sample (~4k points) Python production ↔ Rust-logic-in-Python ↔ Fraction model: 0 disagreements, including the discrete-max witness.

**Blocking for correspondence accept:** this review did not execute compiled Rust or the staged Wasm build; census correctly states sample correspondence is not a Rust proof. Python header still advertises withdrawn `0.500000065` — superseded by census; do not cite.

## 3. Integer safety

Divisors `L64`, `256`, `n·Q`, `2^(24+k)` nonzero on domain. `k_max=23` at `u=16S` (RNE does not round up to 24). Final shift ≤47. P12 headroom large. Residual needs-work (non-blocking hardening): unchecked residual mul/sub and series add; no explicit `k≤23` assert; truncating `as u64`.

## 4. Scope

Established (conditionally on model+source hypotheses): absolute-error bound for frozen `exp_q32` on `[-16S,0]`.  
**Not** established: universal correct rounding; total-variation theorem; uniform-prior / normalization / Hamilton consequences; mainnet certificate verification; release attestation; RC5→new-pin upgrade; three-path Wasm reproducibility. Checked-in `artifacts/mirra_canister.wasm` remains RC5 baseline, not this candidate.

## 5. Provenance

| Item | Verified this session? |
| --- | --- |
| Reviewed commit SHA (archive folder pin) | Yes — `b8c56a1744b53aa9dae656bee17bc09fc717733e` |
| Proof bytes hash | Yes — `37f218d3…3a2d` |
| Checker hash | Yes — `5fa62f9a…5951` |
| Proof-file pin + compiled pin | Yes — both match |
| `check_candidate --check` + named unittests | Yes — PASS |
| `REVIEW.json` accepted decision | No — still pending (this output is external review material) |
| GitHub CI Wasm candidate artifact | **No** — not independently verified here |
| Mainnet / release attestation | **No** — out of scope / open |

---

## Blocking findings (release)

1. Do **not** set all `REVIEW.json` scope fields to `accepted` until source correspondence is closed with compiled Rust/Wasm evidence the owner trusts, or the owner explicitly accepts the residual risk.
2. CI/packaged Wasm for this commit must be independently hashed and bound to this source commit (checked-in Wasm is not the candidate).
3. Human owner must authenticate and record review provenance (this chat/decision file is not self-authenticating).
4. Downstream TV / packaging / mainnet / attestation gates remain open (P17/P18).
5. Any later change to proof/derivation/source requires regenerated bytes, both pins, fresh build, and a **new** review.

## Non-blocking notes

- Unchecked non-Horner i128 ops / missing `k≤23` assert (hardening).
- Superseded Python header claim string.
- Checker imports derive (self-consistency), as expected.

---

## Machine-readable proposal (owner may adapt; reviewer did not edit the repo)

```json
{
  "schema": "mirra.p1.review-decision.v1",
  "decision": "needs-work",
  "proof_sha256": "37f218d3fb9695dbc62cd60955337067b6e55fa45a4d8c0a93ef8415278a3a2d",
  "checker_sha256": "5fa62f9a54d4d06755c256e52d78e007b59929af30200401fd13956e541b5951",
  "reviewer_identity": "MIRRA Reviewer (Grok Bot) 2558ecf1-0890-46d4-ab0e-e477f2aa067b",
  "reviewed_commit": "b8c56a1744b53aa9dae656bee17bc09fc717733e",
  "review_url": null,
  "reviewed_at_utc": "2026-09-06T14:24:11Z",
  "scope": {
    "mathematics": "accepted",
    "source_correspondence": "needs-work",
    "integer_safety": "accepted",
    "scope_and_downstream": "accepted",
    "provenance": "needs-work"
  },
  "overall": "needs-work",
  "math_bound_outward": "0.500000043",
  "notes": "Mathematics of staged absolute-error candidate accepted; release intake blocked on compiled correspondence, CI Wasm provenance, and human REVIEW.json reconciliation."
}
```
