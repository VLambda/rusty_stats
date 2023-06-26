use crate::extra::integral::integral;

pub(crate) fn lower_erf(bound: f64) -> f64 {
    let f = |x: f64| (std::f64::consts::E).powf(-1_f64 * (x.powi(2)));

    let error_function: f64 = (2.0 / std::f64::consts::PI.sqrt()) * integral(bound, 0_f64, f).unwrap();
    error_function
}

pub(crate) fn upper_erf(bound: f64) -> f64 {
    let f = |x: f64| (std::f64::consts::E).powf(-1_f64 * (x.powi(2)));

    let error_function: f64 = (2.0 / std::f64::consts::PI.sqrt()) * integral(0_f64, bound, f).unwrap();
    error_function
}