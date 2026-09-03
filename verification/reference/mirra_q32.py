"""Independent integer-only MIRRA v1 readout reference.

This intentionally favors auditability over speed. Consensus code is Rust;
this module exists to regenerate and independently check the frozen vectors.
"""

from verification.p1.exp_q32_production import exp_q32, rne_div

SCALE = 1 << 32
K_MAX = 64


def uniform_priors(k: int) -> list[int]:
    if not 2 <= k <= K_MAX:
        raise ValueError("K must be in [2,64]")
    base, remainder = divmod(SCALE, k)
    return [base + (1 if index < remainder else 0) for index in range(k)]


def fixed_share_normalize(
    priors: list[int], factors: list[int], floor: int
) -> list[int]:
    if len(priors) != len(factors) or not 2 <= len(priors) <= K_MAX:
        raise ValueError("prior and factor vectors require the same K in [2,64]")
    if sum(priors) != SCALE:
        raise ValueError("priors must sum exactly to SCALE")
    if floor < 1 or len(priors) * floor > SCALE:
        raise ValueError("invalid fixed-share floor")
    if any(factor < 0 or factor > SCALE for factor in factors):
        raise ValueError("factor is outside [0,SCALE]")
    products = [prior * factor for prior, factor in zip(priors, factors)]
    total = sum(products)
    if total <= 0:
        raise ValueError("normalization total must be positive")
    distributable = SCALE - len(priors) * floor
    divisions = [divmod(product * distributable, total) for product in products]
    quotas = [division[0] for division in divisions]
    remainders = [division[1] for division in divisions]
    deficit = distributable - sum(quotas)
    order = sorted(range(len(priors)), key=lambda index: (-remainders[index], index))
    result = [floor + quota for quota in quotas]
    for index in order[:deficit]:
        result[index] += 1
    if sum(result) != SCALE:
        raise AssertionError("exact-sum invariant failed")
    return result


def weights_from_penalties(
    penalties: list[int], priors: list[int], bound_q32: int, floor: int
) -> list[int]:
    if len(penalties) != len(priors) or not penalties:
        raise ValueError("penalties and priors require the same nonzero length")
    if any(penalty < 0 for penalty in penalties):
        raise ValueError("penalties must be nonnegative")
    best = min(penalties)
    factors = []
    for penalty in penalties:
        gap_q32 = rne_div(penalty - best, SCALE)
        factors.append(exp_q32(-gap_q32) if gap_q32 <= bound_q32 else 0)
    return fixed_share_normalize(priors, factors, floor)
