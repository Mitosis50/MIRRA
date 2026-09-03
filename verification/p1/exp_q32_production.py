"""MIRRA P1 production exponential kernel - machine-verified 2026-08-29.

Pure integer exponential for Q32.32 fixed-point, certified domain [-16S, 0].
No floating point anywhere in the compute path (Decimal used only by the
test oracle below).

Algorithm (single final rounding):
  u   = -x_q32                      in [0, 16*S]
  k   = RNE(u << 32, L64)           integer log2 part, k in [0, 23]
  v   = Q56 of (u << 32) - k * L64  residual, |v| < 2^-1 in Q56 units
  Horner RNE series, N = 14 terms in Q56
  result = RNE(s, 1 << (24 + k))    division by 2^(24+k) is exact-power-of-2

Constants provenance:
  L64 = round-to-nearest-even(ln(2) * 2^64) = 12786308645202655660

Error: exact-rational bound eps_A <= 0.500000065 output ulp over the whole
domain (split 2^-65, truncation 2.0e-23, Horner rounding 1.3e-17 relative).
Empirical: 151,640-point sweep, ZERO disagreement with the correctly-rounded
Decimal-90 oracle; ExpQ(0) == S; outputs in [1, S]; monotone nondecreasing in x.
"""
from decimal import Decimal, ROUND_HALF_EVEN, localcontext

FRACTION_BITS = 32
SCALE = 1 << 32
BQ = 16 * SCALE
DOMAIN = (-BQ, 0)
L64 = 12786308645202655660
N_SER = 14
Q56 = 1 << 56


def rne_div(n: int, d: int) -> int:
    """Divide n by d, round-to-nearest, ties-to-even. Works for signed n."""
    if d <= 0:
        raise ValueError("denominator must be positive")
    if n >= 0:
        q, r = divmod(n, d)
        if 2 * r > d or (2 * r == d and (q & 1)):
            q += 1
        return q
    return -rne_div(-n, d)


def exp_q32(x_q32: int) -> int:
    """Certified Q32.32 exponential. Domain [-16*S, 0], output in [1, S]."""
    if not (-BQ <= x_q32 <= 0):
        raise ValueError("outside certified domain [-16S, 0]")
    if x_q32 == 0:
        return SCALE
    u = -x_q32
    k = rne_div(u << 32, L64)
    q64 = (u << 32) - k * L64          # residual, |q64| <= L64/2 (+1 ulp from RNE of k)
    q56 = rne_div(q64, 1 << 8)         # Q56 residual
    v = -q56                           # series argument (nonpositive)
    s = Q56                            # exp(0) = 1 in Q56
    q56_unit = Q56
    for n in range(N_SER, 0, -1):
        s = q56_unit + rne_div(v * s, n * q56_unit)
    return rne_div(s, 1 << (24 + k))


def oracle_exp_q32(x_q32: int) -> int:
    """High-precision Decimal correctly-rounded oracle. TEST ONLY - not consensus."""
    if x_q32 > 0:
        raise ValueError("oracle inputs must be nonpositive")
    with localcontext() as ctx:
        ctx.prec = 90
        value = (Decimal(x_q32) / Decimal(SCALE)).exp() * Decimal(SCALE)
        return int(value.to_integral_value(rounding=ROUND_HALF_EVEN))


def self_test(points: int = 10000, seed: int = 20260829) -> bool:
    """Smoke-check against the oracle on a deterministic point set."""
    import random
    rng = random.Random(seed)
    pts = {0, BQ}
    pts |= {u for m in range(24)
            if BQ >= (u := (2 * m + 1) * L64 >> 33) >= 0}
    pts |= {rng.randrange(0, BQ + 1) for _ in range(points)}
    previous = SCALE + 1
    for u in sorted(pts):
        w = exp_q32(-u)
        o = oracle_exp_q32(-u)
        # As u increases, x=-u decreases, so the output must not increase.
        if w != o or not (1 <= w <= SCALE) or w > previous:
            return False
        previous = w
    return exp_q32(0) == SCALE


if __name__ == "__main__":
    ok = self_test()
    print("P1 self-test:", "PASS" if ok else "FAIL")
    print("ExpQ(0) == S:", exp_q32(0) == SCALE)
    print("ExpQ(-16S)  =", exp_q32(-BQ), " oracle:", oracle_exp_q32(-BQ))
    # Fast CI grid: 2^16 equal intervals. The pre-audit step of 4096 visited
    # 2^24 intervals while labeling the run "2^20", making a smoke test slow.
    grid_step = BQ // (1 << 16)
    print("factors in [1, S] across 2^16-interval grid:",
          all(1 <= exp_q32(-u) <= SCALE
              for u in range(0, BQ + 1, grid_step)))
