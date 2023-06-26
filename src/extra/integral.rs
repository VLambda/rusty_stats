pub(crate) fn integral(lower_bound: f64, upper_bound: f64, f: fn(f64) -> f64) -> Result<f64, String> {
    let lower = lower_bound;
    let upper = upper_bound;

    let n = 100000;
    let h = (upper - lower) / n as f64;

    let mut sum = f(lower) + f(upper);

    for i in 1..n {
        let x = lower + i as f64 * h;
        sum += if i % 2 == 0 {
            2.0 * f(x)
        } else {
            4.0 * f(x)
        };
    }

    let result = (h / 3.0) * sum;

    if result.is_infinite() || result.is_nan() {
        return Err("Function is Divergent".to_string());
    }

    if (result.floor() + 1e-6) > result || (result.ceil() - 1e-6) < result {
        let rounded_result = result.round();
        Ok(rounded_result)
    } else {
        Ok(result)
    }
}