/// Equation 11 from [Salkuyeh and Khojasteh (2006)](https://doi.org/10.12988/imf.2006.06086).
pub fn matrix_power_salkuyeh(
    mat_size: usize,
    mat_power: usize,
    upper_diag: f64,
    diag: f64,
    lower_diag: f64,
) -> impl Fn(usize, usize) -> f64 {
    let n = mat_size;
    let m = mat_power;

    let a = upper_diag;
    let b = diag;
    let c = lower_diag;

    // pi / (n + 1)
    let pi_np1 = std::f64::consts::PI / (n as f64 + 1.0);

    move |i, j| {
        debug_assert!(1 <= i && i <= n);
        debug_assert!(1 <= j && j <= n);

        2.0 / (n as f64 + 1.0)
            * (c / a).powi((i as i32 - j as i32) / 2)
            * (1..=n)
                .map(|k| k as f64)
                .map(|k| {
                    // lambda_k = b + 2a sqrt(c/a) cos(k pi / (n + 1))
                    let lambda = b + 2.0 * a * f64::sqrt(c / a) * f64::cos(k * pi_np1);

                    lambda.powi(m as i32)
                        * f64::sin(i as f64 * k * pi_np1)
                        * f64::sin(j as f64 * k * pi_np1)
                })
                .sum::<f64>()
    }
}

/// Chebyshev polynomial of the 2nd kind.
fn cheby_2k(n: usize, x: f64) -> f64 {
    use std::mem::replace;
    match n {
        0 => 1.0,
        1 => 2.0 * x,
        _ => {
            let mut last = 1.0;
            let mut current = 2.0 * x;
            for _ in 1..n {
                let new = 2.0 * x * current - last;
                last = replace(&mut current, new);
            }
            current
        }
    }
}

/// See [Encinas and Jiménez (2018)](https://doi.org/10.1016/j.laa.2017.06.010).
pub(crate) fn tridiagonal_toeplitz_inverse_element(
    i: usize,
    j: usize,
    n: usize,
    alpha: f64,
    beta: f64,
    gamma: f64,
) -> f64 {
    if i > j {
        // swap i <-> j and alpha <-> gamma
        tridiagonal_toeplitz_inverse_element(j, i, n, gamma, beta, alpha)
    } else {
        let sqrt_ag = f64::sqrt(alpha * gamma);
        let q = (beta / 2.0) / sqrt_ag;

        f64::powi(alpha / sqrt_ag, (j - i) as i32) * cheby_2k(i, q) * cheby_2k(n - j - 1, q)
            / (sqrt_ag * cheby_2k(n, q))
    }
}
