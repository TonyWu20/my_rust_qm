/// Composite Simpson's 1/3 rule implementation
/// Suppose the interval `[a,b]` is split up into
/// n subintervals, with `n` an even number.
pub fn simpson_rule(fx: &[f64], x: &[f64]) -> f64 {
    // n is assume to be even
    let n = x.len() - 1;
    let dx = (x[n] - x[0]) / n as f64;
    1.0 / 3.0
        * dx
        * (1..=(n / 2)) // Σ_1^N/2
            .map(|i| {
                // f(2i-2) + f(2i-1) + f(2i)
                fx[2 * i - 2] + 4.0 * fx[2 * i - 1] + fx[2 * i]
            })
            .sum::<f64>()
}

pub fn adaptive_simpson(fx: &[f64], x: &[f64]) -> f64 {
    let n = x.len() - 1;
    let h = (x[n] - x[0]) / n as f64;
    1.0 / 48.0
        * h
        * (17.0 * fx[0]
            + 59.0 * fx[1]
            + 43.0 * fx[2]
            + 49.0 * fx[3]
            + 48.0 * dbg!((4..=n - 4).map(|i| fx[i]).sum::<f64>())
            + 49.0 * fx[n - 3]
            + 43.0 * fx[n - 2]
            + 59.0 * fx[n - 1]
            + 17.0 * fx[n])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simpson_13_rule() {
        let x: [f64; 5] = [2.0, 4.0, 6.0, 8.0, 10.0];
        let fx: [f64; 5] = x.map(|x| x.powi(3));
        assert_eq!(2496.0, simpson_rule(&fx, &x));
        let x = (2..=16).step_by(2).map(|i| i as f64).collect::<Vec<_>>();
        let fx = x.iter().map(|x| x.powi(3)).collect::<Vec<_>>();
        assert_eq!(
            (16_f64.powi(4) - 2_f64.powi(4)) / 4.0,
            adaptive_simpson(&fx, &x)
        );
        let x = (0..=30)
            .step_by(5)
            .map(|i: usize| i as f64 * 60_f64)
            .collect::<Vec<f64>>();
        let fx = [25_f64, 28_f64, 32_f64, 30_f64, 29_f64, 26_f64, 23_f64];
        let s = simpson_rule(&fx, &x);
        assert_eq!(50600.0, s);
        let x = (0..=45)
            .step_by(5)
            .map(|i: usize| i as f64 * 60_f64)
            .collect::<Vec<f64>>();
        let fx = [
            25_f64, 28_f64, 32_f64, 30_f64, 29_f64, 26_f64, 23_f64, 22_f64, 39_f64, 30_f64,
        ];
        let s = adaptive_simpson(&fx, &x);
        dbg!(s);
    }
}
