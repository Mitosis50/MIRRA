#!/usr/bin/env python3
"""Repository adapter of the preserved September 3 MIRRA P1 derivation.

Only input loading and provenance labels differ from the archived calculator.
The exact-rational formulas and sample checks are preserved.
No input files are modified.

All bounds, decisions, and interval checks use integers/Fraction. Decimal is
used only to display results and by the pre-existing empirical RC5 oracle.
This does not issue a certificate, modify a pin, run Rust/Wasm, or sign a release.
"""

import argparse
import hashlib
import json
import random
from decimal import Decimal, localcontext
from fractions import Fraction as F
from math import factorial
from pathlib import Path


S = 1 << 32
Q = 1 << 56
L = 12786308645202655660
N = 14
H = F(1, 2 * Q)
RC5_SHA = "0b28a300bb6438f951df3585a649cc63defe9645a5064ac47e0a2c43143f16f9"
SCRIPTS_SHA = "08a4fbbebfc4ae08cd78c7c17309788d9ae9a7b5b3e80d2c214fc71a0259d763"
WASM_SHA = "acf963dadeb9eeb4c7ee5e6065045a1dcdc93af0700ce47868b5b2aa7485dfa2"
PROOF_SHA = "b9d1c0d5e0d8144d88f6e1375429fc844967731090d878ebd311072ee456a57b"


def require(ok, message):
    if not ok:
        raise ValueError(message)


def rne(n, d):
    require(d > 0, "invalid divisor")
    # Fraction.__round__ is exact nearest integer with half ties to even.
    return round(F(n, d))


def reduction(u):
    require(0 <= u <= 16 * S, "input outside frozen domain")
    k = rne(u * S, L)
    r = u * S - k * L
    v = -rne(r, 256)
    return k, r, v


def kernel_model(u):
    k, _, v = reduction(u)
    series = Q
    for n in range(N, 0, -1):
        product = v * series
        require(abs(product) < 1 << 127, "sampled product overflow")
        series = Q + rne(product, n * Q)
    return rne(series, 1 << (24 + k)), F(series, Q)


def polynomial(x, degree):
    term = total = F(1)
    for n in range(1, degree + 1):
        term *= x / n
        total += term
    return total


def exp_interval_nonnegative(x, degree=48):
    """P_m <= exp(x) <= P_m + first_omitted/(1-x/(m+2))."""
    require(0 <= x < degree + 2, "invalid exponential tail ratio")
    partial = polynomial(x, degree)
    first_omitted = x ** (degree + 1) / factorial(degree + 1)
    return partial, partial + first_omitted / (1 - x / (degree + 2))


def ln2_interval(terms=240):
    lower = sum((F(1, (1 << j) * j) for j in range(1, terms + 1)), F())
    return lower, lower + F(1, (terms + 1) * (1 << terms))


def ln2_alternate_interval(terms=120):
    # ln2 = 2*atanh(1/3). Used only to cross-check the primary enclosure.
    lower = sum((F(2, (2*j + 1) * 3**(2*j + 1)) for j in range(terms)), F())
    tail = F(2, (2*terms + 1) * 3**(2*terms + 1)) / (1 - F(1, 9))
    return lower, lower + tail


def ceiling9(x):
    scale = 10**9
    units = -(-(x.numerator * scale) // x.denominator)
    require(F(units - 1, scale) < x <= F(units, scale), "ceiling failure")
    return f"{units // scale}.{units % scale:09d}"


def display(x):
    with localcontext() as context:
        context.prec = 70
        return str(Decimal(x.numerator) / Decimal(x.denominator))


def quantity(x):
    return {"exact_fraction": str(x), "decimal_display_not_a_directed_bound": display(x)}


def old_e_upper(a):
    """Faithful old formula for diagnostic comparison, not the new proof."""
    total = term = F(1)
    for n in range(1, 40):
        term *= a / n
        total += term
        if term * F(3, n + 1) < F(1, 10**30):
            break
    return total + term * F(3, n + 1)


def old_stack_with_corrected_t(t, old_split):
    # Preserve its extra-conservative T**14/15! truncation expression.
    truncation = t**N / factorial(N + 1) * old_e_upper(t)
    horner = F(3, 2) * H / (1 - t)
    return F(1, 2) + S * (23 * old_split + (horner + truncation) * old_e_upper(t))


def derive_budget(t, delta_bound):
    require(0 < t < 1, "invalid argument enclosure")
    horner = H * polynomial(t, N - 1)
    truncation = t**(N + 1) / factorial(N + 1) / (1 - t / (N + 2))
    exp_t = exp_interval_nonnegative(t)[1]
    rows = []
    for k in range(24):
        # k=0 => r=u*S is divisible by 256, so argument conversion is exact.
        argument_shift = F() if k == 0 else H + k * delta_bound
        require(0 <= argument_shift < 1, "invalid perturbation enclosure")
        # exp(A)-1 <= A/(1-A), from the positive exponential series.
        argument_error = exp_t * argument_shift / (1 - argument_shift)
        total = F(1, 2) + F(S, 1 << k) * (horner + truncation + argument_error)
        rows.append((k, total, argument_shift, argument_error))
    worst_k, worst, _, _ = max(rows, key=lambda row: row[1])
    return horner, truncation, exp_t, rows, worst_k, worst


def compute(root):
    source = (root / "verification/p1/exp_q32_production.py").read_bytes()
    rust = (root / "src/core.rs").read_bytes()
    require(hashlib.sha256(rust).hexdigest() ==
            "585fee4d98c533fda08fcb10a072f397c23302f7187abe9b0906b0ec3c755b21",
            "Rust arithmetic source drift; redo source correspondence review")
    require(hashlib.sha256(source).hexdigest() ==
            "f2e19e3a4b32c715e2a749f2d8d3596408323637883540fb10878720a291a9cd",
            "Python arithmetic source drift; redo source correspondence review")
    rc5 = {"__name__": "reviewed_rc5_kernel"}
    exec(compile(source, "hash-pinned RC5 Python kernel", "exec"), rc5)
    require((rc5["SCALE"], rc5["Q56"], rc5["L64"], rc5["N_SER"]) == (S, Q, L, N), "kernel constants drift")
    old_bytes = (root / "verification/p1/review/WITHDRAWN_p1_proof_artifact.json").read_bytes()
    require(hashlib.sha256(old_bytes).hexdigest() == PROOF_SHA, "withdrawn historical proof drift")
    old = json.loads(old_bytes)

    require(rne(16 * S * S, L) == 23, "wrong maximum k")
    require((L // 2) % 4 == 2, "tie-impossibility premise changed")
    candidates = {0, 16*S}
    for m in range(24):
        left = ((2*m + 1)*L) // (2*S)
        candidates.update(u for u in (left, left + 1) if 0 <= u <= 16*S)
    # On each constant-k interval r is affine, RNE(r/256) is monotone,
    # and its absolute maximum lies at one of that interval's endpoints.
    peak = max(sorted(candidates), key=lambda u: abs(reduction(u)[2]))
    exact_max = F(abs(reduction(peak)[2]), Q)
    analytic_t = F(L, 512*Q) + H
    require(peak == 16373744595, "unexpected discrete maximum witness")
    require(exact_max < analytic_t, "range enclosure failure")

    log_lo, log_hi = ln2_interval()
    alt_lo, alt_hi = ln2_alternate_interval()
    require(log_lo <= alt_lo < alt_hi <= log_hi, "independent log enclosure mismatch")
    delta = max(abs(F(L, 1 << 64) - log_lo), abs(F(L, 1 << 64) - log_hi))
    require(delta <= F(1, 1 << 65), "L64 not enclosed as nearest log constant")
    horner, truncation, exp_t, rows, worst_k, worst = derive_budget(analytic_t, delta)
    _, _, _, _, discrete_worst_k, discrete_bound = derive_budget(exact_max, delta)
    require(worst_k == discrete_worst_k == 0, "unexpected worst budget stratum")
    require(F("0.500000042") < worst < F("0.500000043"), "derived-budget regression")
    require(worst < F("0.500000065"), "original decimal no longer covers new derivation")

    # Analytic interval invariants for every Horner stage, not sampled only.
    stage_abs = (1 + H) / (1 - analytic_t)
    require(1 + analytic_t * stage_abs + H == stage_abs, "stage-invariant algebra")
    require(Q*Q*analytic_t*stage_abs < 1 << 127, "analytic signed-product bound")
    require(Q*stage_abs < 1 << 127, "analytic accumulator bound")
    require(1 / exp_t > horner + truncation, "positive final series not established")

    rng = random.Random(20260903)
    samples = candidates | {rng.randrange(16*S + 1) for _ in range(2000)}
    for u in sorted(samples):
        k, residual, v = reduction(u)
        output, series = kernel_model(u)
        a = F(v, Q)
        require(output == rc5["exp_q32"](-u), "model/RC5 disagreement")
        require(abs(a) <= exact_max, "sample exceeds maximum")
        require(abs(a + F(residual, 1 << 64)) <= H, "argument rounding bound")
        require(abs(series - polynomial(a, N)) <= horner, "Horner error bound")
        if k == 0:
            require(a == -F(u, S), "k=0 argument exactness")

    witness = 16373744595
    k, residual, v = reduction(witness)
    output = rc5["exp_q32"](-witness)
    require(output == rc5["oracle_exp_q32"](-witness) == 94906266, "witness oracle mismatch")
    positive_lo, positive_hi = exp_interval_nonnegative(F(witness, S), 96)
    real_lo, real_hi = F(S) / positive_hi, F(S) / positive_lo
    require(output - F(1, 2) < real_lo <= real_hi < output + F(1, 2), "witness exact rounding enclosure failed")

    old_fixed = old_stack_with_corrected_t(exact_max, F(old["exact_bounds"]["split_real"]))
    require(F("0.500000100") < old_fixed < F("0.500000101"), "mechanical old-stack regression")
    return {
        "status": "DERIVED_REVIEW_CANDIDATE_NOT_RELEASE_ATTESTATION",
        "scope": "absolute error versus S*exp(x/S), all integer x in [-16S,0], frozen N=14",
        "inputs": {"rc5_archive_sha256": RC5_SHA, "scripts_zip_sha256": SCRIPTS_SHA,
                   "old_proof_sha256": PROOF_SHA, "baseline_rc5_wasm_sha256_not_staged_build": WASM_SHA,
                   "rust_arithmetic_source_sha256": hashlib.sha256(rust).hexdigest(),
                   "rc5_python_source_sha256": hashlib.sha256(source).hexdigest()},
        "constants": {"S": S, "Q56": Q, "L64": L, "N": N, "domain_min": -16*S},
        "range": {"boundary_candidates": len(candidates), "maximum_witness_u": peak,
                  "true_discrete_max": quantity(exact_max), "safe_analytic_t": quantity(analytic_t),
                  "halfway_ties_possible": False,
                  "witness_residual_gap_below_L64_over_2": L//2 - abs(residual)},
        "witness": {"u": witness, "k": k, "q64": residual, "v_q56": v,
                    "kernel_raw": output, "decimal90_oracle_raw": output,
                    "real_exponential_output_lower": quantity(real_lo),
                    "real_exponential_output_upper": quantity(real_hi),
                    "real_output_interval_width": quantity(real_hi - real_lo),
                    "absolute_error_upper": quantity(max(abs(output-real_lo), abs(output-real_hi))),
                    "correct_rounding_proven_by_rational_interval": True},
        "mechanical_old_formula_with_true_tmax_not_new_proof": {
            "bound": quantity(old_fixed), "ceiling_9dp": ceiling9(old_fixed)},
        "new_derivation": {
            "ln2_lower": quantity(log_lo), "ln2_upper": quantity(log_hi),
            "ln2_secondary_enclosure_inside_primary": True,
            "split_delta_upper": quantity(delta), "horner_real": quantity(horner),
            "truncation_real": quantity(truncation), "exp_t_upper": quantity(exp_t),
            "worst_budget_k": worst_k, "global_bound_safe_analytic_t": quantity(worst),
            "global_bound_true_discrete_t": quantity(discrete_bound),
            "ceiling_9dp": ceiling9(worst), "original_decimal_covered": worst < F("0.500000065"),
            "k_cases": [{"k": kk, "bound_output_ulps": quantity(bb),
                         "argument_shift_upper": quantity(aa), "argument_error_real_upper": quantity(ee)}
                        for kk, bb, aa, ee in rows]},
        "checks": {"model_matches_rc5_and_horner_bound_at_points": len(samples),
                   "analytic_horner_i128_overflow_guard": True,
                   "frozen_N_14": True, "new_rust_or_wasm_execution": False,
                   "full_151640_sweep_rerun": False,
                   "calculator_modifies_files": False},
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args()
    result = compute(args.root)
    if args.json:
        print(json.dumps(result, indent=2))
    else:
        print("Witness correctly rounded:", result["witness"]["kernel_raw"])
        print("Actual discrete maximum:", result["range"]["true_discrete_max"]["decimal_display_not_a_directed_bound"])
        print("Old formula, corrected T:", result["mechanical_old_formula_with_true_tmax_not_new_proof"]["bound"]["decimal_display_not_a_directed_bound"])
        print("Fresh derived bound:", result["new_derivation"]["global_bound_safe_analytic_t"]["decimal_display_not_a_directed_bound"])
        print("Outward 9-dp ceiling:", result["new_derivation"]["ceiling_9dp"])
        print("Worst bound stratum:", result["new_derivation"]["worst_budget_k"])
        print("Model/RC5 checks:", result["checks"]["model_matches_rc5_and_horner_bound_at_points"])
        print(result["status"])


if __name__ == "__main__":
    main()
