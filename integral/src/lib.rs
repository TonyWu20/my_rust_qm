/// Composite Simpson's 1/3 rule implementation
/// Suppose the interval `[a,b]` is split up into
/// n subintervals, with `n` an even number.
pub fn simpson_rule(fx: &[f64], dx: f64) -> f64 {
    // n is assume to be even
    let n = fx.len();
    dx / 3.0
        * (1..=(n / 2)) // Σ_1^N/2
            .map(|i| {
                // f(2i-2) + f(2i-1) + f(2i)
                fx[2 * i - 2] + 4.0 * fx[2 * i - 1] + fx[2 * i]
            })
            .sum::<f64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simpson_13_rule() {
        let x = [2, 4, 6, 8, 10];
        let fx: [f64; 5] = x.map(|x| (x as f64).powi(3));
        let dx = ((x[4] - x[0]) / (x.len() - 1)) as f64;
        assert_eq!(2496.0, simpson_rule(&fx, dx));
        let x = (0..=30)
            .step_by(5)
            .map(|i: usize| i * 60)
            .collect::<Vec<usize>>();
        let fx = [25_f64, 28_f64, 32_f64, 30_f64, 29_f64, 26_f64, 23_f64];
        let dx = ((x[x.len() - 1] - x[0]) / (x.len() - 1)) as f64;
        let s = simpson_rule(&fx, dx);
        assert_eq!(50600.0, s);
    }
}
