use std::f64::consts::PI;
use crate::extra::erf::upper_erf;

pub(crate) fn inverse_error_function(x: f64) -> f64 {
    let w = -((1.0 - x) * (1.0 + x)).ln();
    let mut p;

    if w < 5.0 {
        let w_minus_2_5 = w - 2.5;
        p = 2.81022636e-08;
        p = 3.43273939e-07 + p * w_minus_2_5;
        p = -3.5233877e-06 + p * w_minus_2_5;
        p = -4.39150654e-06 + p * w_minus_2_5;
        p = 0.00021858087 + p * w_minus_2_5;
        p = -0.00125372503 + p * w_minus_2_5;
        p = -0.00417768164 + p * w_minus_2_5;
        p = 0.246640727 + p * w_minus_2_5;
        p = 1.50140941 + p * w_minus_2_5;
    } else {
        let sqrt_w_minus_3 = (w).sqrt() - 3.0;
        p = -0.000200214257;
        p = 0.000100950558 + p * sqrt_w_minus_3;
        p = 0.00134934322 + p * sqrt_w_minus_3;
        p = -0.00367342844 + p * sqrt_w_minus_3;
        p = 0.00573950773 + p * sqrt_w_minus_3;
        p = -0.0076224613 + p * sqrt_w_minus_3;
        p = 0.00943887047 + p * sqrt_w_minus_3;
        p = 1.00167406 + p * sqrt_w_minus_3;
        p = 2.83297682 + p * sqrt_w_minus_3;
    }

    let res_ra = p * x;

    // Halley's method to refine estimate of inverse erf
    let mut res_hm = 0.0;
    let fx = (upper_erf(res_ra) - x) as f64;
    let df = (2.0 / (PI).sqrt()) * (-res_ra * res_ra).exp();
    let d2f = -2.0 * res_ra * df;

    res_hm = res_ra - (2.0 * fx * df) / ((2.0 * df * df) - (fx * d2f));
    res_hm
}