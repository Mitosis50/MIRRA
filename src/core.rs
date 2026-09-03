//! Integer-exact MIRRA v1 mathematical kernel.

pub const SCALE: u128 = 1u128 << 32;
pub const K_MAX: usize = 64;
pub const B_Q32: i64 = 16i64 << 32;
pub const FLOOR_UNITS: u64 = 4096;
pub const ETA_Q32: u64 = 1u64 << 29; // 1/8 in Q32.32
pub const LOSS_MIN_Q32: u64 = 0;
pub const LOSS_MAX_Q32: u64 = SCALE as u64;
const L64: u128 = 12_786_308_645_202_655_660;
const N_SER: usize = 14;

pub fn rne_div_u(n: u128, d: u128) -> u128 {
    assert!(d > 0, "denominator must be positive");
    let q = n / d;
    let r = n % d;
    let complement = d - r;
    if r > complement || (r == complement && q & 1 == 1) {
        q + 1
    } else {
        q
    }
}

pub fn rne_div_i(n: i128, d: i128) -> i128 {
    assert!(d > 0, "denominator must be positive");
    if n >= 0 {
        rne_div_u(n as u128, d as u128) as i128
    } else {
        let rounded = rne_div_u(n.unsigned_abs(), d as u128);
        if rounded == 1u128 << 127 {
            i128::MIN
        } else {
            -(rounded as i128)
        }
    }
}

pub fn validate_losses(losses_q32: &[u64]) -> Result<(), String> {
    if !(2..=K_MAX).contains(&losses_q32.len()) {
        return Err(format!("loss vector length must be in 2..={K_MAX}"));
    }
    if losses_q32.iter().any(|loss| *loss > LOSS_MAX_Q32) {
        return Err("loss must be an unsigned Q32.32 value in the inclusive range [0,1]".into());
    }
    Ok(())
}

pub fn accumulate_penalties(penalties: &[i128], losses_q32: &[u64]) -> Result<Vec<i128>, String> {
    if penalties.len() != losses_q32.len() {
        return Err("penalty and loss vectors must have equal length".into());
    }
    validate_losses(losses_q32)?;
    if penalties.iter().any(|penalty| *penalty < 0) {
        return Err("penalties must be nonnegative".into());
    }
    let smallest = *losses_q32.iter().min().ok_or("loss vector is empty")?;
    let mut updated = Vec::with_capacity(penalties.len());
    for (penalty, loss) in penalties.iter().zip(losses_q32) {
        let loss_gap = loss
            .checked_sub(smallest)
            .ok_or("loss recentering underflow")?;
        let delta = (ETA_Q32 as i128)
            .checked_mul(loss_gap as i128)
            .ok_or("Q64.64 penalty delta overflow")?;
        updated.push(
            penalty
                .checked_add(delta)
                .ok_or("Q64.64 cumulative penalty overflow")?,
        );
    }
    let common = *updated
        .iter()
        .min()
        .ok_or("updated penalty vector is empty")?;
    Ok(updated.into_iter().map(|value| value - common).collect())
}

pub fn exp_inputs_from_penalties(
    penalties: &[i128],
    bound_q32: i64,
) -> Result<Vec<Option<i64>>, String> {
    if penalties.is_empty() {
        return Err("at least one penalty is required".into());
    }
    if bound_q32 <= 0 {
        return Err("domain bound must be positive".into());
    }
    if penalties.iter().any(|penalty| *penalty < 0) {
        return Err("penalties must be nonnegative".into());
    }
    let best = *penalties.iter().min().unwrap();
    penalties
        .iter()
        .map(|penalty| {
            let gap = penalty.checked_sub(best).ok_or("penalty gap underflow")?;
            let rounded_gap = rne_div_i(gap, SCALE as i128);
            if rounded_gap <= bound_q32 as i128 {
                let gap_i64 = i64::try_from(rounded_gap)
                    .map_err(|_| "in-window exponent does not fit i64")?;
                Ok(Some(-gap_i64))
            } else {
                Ok(None)
            }
        })
        .collect()
}

pub fn exp_q32(x_q32: i64) -> Result<u64, String> {
    if !(-B_Q32..=0).contains(&x_q32) {
        return Err("outside certified domain [-16S,0]".into());
    }
    if x_q32 == 0 {
        return Ok(SCALE as u64);
    }
    let u = (-(x_q32 as i128)) as u128;
    let k = rne_div_u(u << 32, L64);
    let q64 = (u << 32) as i128 - k as i128 * L64 as i128;
    let v = -rne_div_i(q64, 1 << 8);
    let unit: i128 = 1 << 56;
    let mut series: i128 = unit;
    for n in (1..=N_SER).rev() {
        let product = v.checked_mul(series).ok_or("P1 Horner product overflow")?;
        series = unit + rne_div_i(product, n as i128 * unit);
    }
    if series < 0 {
        return Err("P1 produced a negative intermediate".into());
    }
    Ok(rne_div_u(series as u128, 1u128 << (24 + k)) as u64)
}

pub fn fixed_share_normalize(
    priors: &[u64],
    factors: &[u64],
    floor: u64,
) -> Result<Vec<u64>, String> {
    if priors.len() != factors.len() || !(2..=K_MAX).contains(&priors.len()) {
        return Err(format!(
            "prior and factor vectors require the same K in 2..={K_MAX}"
        ));
    }
    if floor < 1 {
        return Err("floor must be at least one".into());
    }
    if priors.iter().map(|value| *value as u128).sum::<u128>() != SCALE {
        return Err("Q32.32 priors must sum exactly to SCALE".into());
    }
    let k = priors.len() as u128;
    let reserve = k.checked_mul(floor as u128).ok_or("K*floor overflow")?;
    if reserve > SCALE {
        return Err("K*floor must not exceed SCALE".into());
    }
    if factors.iter().any(|factor| *factor > SCALE as u64) {
        return Err("P1 factors must lie in [0,SCALE]".into());
    }
    let products: Vec<u128> = priors
        .iter()
        .zip(factors)
        .map(|(prior, factor)| *prior as u128 * *factor as u128)
        .collect();
    let total: u128 = products.iter().sum();
    if total == 0 {
        return Err("normalization total must be positive".into());
    }
    let distributable = SCALE - reserve;
    let mut quotas = Vec::with_capacity(products.len());
    let mut remainders = Vec::with_capacity(products.len());
    for product in &products {
        let numerator = product
            .checked_mul(distributable)
            .ok_or("Hamilton numerator overflow")?;
        quotas.push(numerator / total);
        remainders.push(numerator % total);
    }
    let assigned: u128 = quotas.iter().sum();
    let deficit = distributable - assigned;
    let mut order: Vec<usize> = (0..products.len()).collect();
    order.sort_by(|a, b| remainders[*b].cmp(&remainders[*a]).then(a.cmp(b)));
    let mut output: Vec<u64> = quotas
        .iter()
        .map(|quota| (floor as u128 + quota) as u64)
        .collect();
    for index in order.into_iter().take(deficit as usize) {
        output[index] += 1;
    }
    if output.iter().map(|value| *value as u128).sum::<u128>() != SCALE {
        return Err("exact-sum invariant failed".into());
    }
    let ceiling = SCALE - (k - 1) * floor as u128;
    if output
        .iter()
        .any(|value| (*value as u128) < floor as u128 || (*value as u128) > ceiling)
    {
        return Err("envelope invariant failed".into());
    }
    Ok(output)
}

pub fn weights_from_penalties(
    penalties: &[i128],
    priors: &[u64],
    bound_q32: i64,
    floor: u64,
) -> Result<Vec<u64>, String> {
    let factors = exp_inputs_from_penalties(penalties, bound_q32)?
        .into_iter()
        .map(|input| match input {
            Some(value) => exp_q32(value),
            None => Ok(0),
        })
        .collect::<Result<Vec<_>, _>>()?;
    fixed_share_normalize(priors, &factors, floor)
}

pub fn uniform_priors(k: usize) -> Result<Vec<u64>, String> {
    if !(2..=K_MAX).contains(&k) {
        return Err(format!("K must be in 2..={K_MAX}"));
    }
    let base = (SCALE / k as u128) as u64;
    let remainder = (SCALE % k as u128) as usize;
    Ok((0..k)
        .map(|index| base + u64::from(index < remainder))
        .collect())
}

pub fn weights_v1(penalties: &[i128], priors: &[u64]) -> Result<Vec<u64>, String> {
    weights_from_penalties(penalties, priors, B_Q32, FLOOR_UNITS)
}
