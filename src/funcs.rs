

/// Calculates the absolute economic capacity of an economy given the 'value' of each category of resource available.
#[inline]
pub fn capacity(land: f64, capital: f64, labour: f64, enterprise: f64) -> f64 {
    skew([land, capital, labour, enterprise], 0.5)
}

#[inline]
pub fn delta_capital(capital: f64, mpc: f64, depreciation: f64, delta_aggregate: f64) -> f64 {
    debug_assert!(mpc >= 0.0 && mpc <= 1.0);
    (1.0 - mpc) * delta_aggregate - depreciation * capital
}

#[inline]
pub fn consumer_utility(aggregate: f64, mpc: f64) -> f64 {
    debug_assert!(mpc >= 0.0 && mpc <= 1.0);
    aggregate * mpc
}

/// Shuffles the value of the firm depending on its 'optimum' market share.
/// Returns the new value of the firm.
#[inline]
pub fn delta_shuffle(rps: f64, rs: f64, v: f64, k: f64, step: f64) -> f64 {
    let v_opt = rs / rps;
    
    if v_opt > v {
        let delta = v_opt - v;
        delta.powf(k) * step
    } else {
        v_opt
    }
}

/// (i1^pow, i2^pow, ..., in^pow)^(1/pow)
#[inline]
fn skew<const N: usize>(items: [f64; N], pow: f64) -> f64 {
    let mut total: f64 = 0.0;
    for i in items {
        total += i.powf(pow);
    }
    total.powf(1.0/pow)
}