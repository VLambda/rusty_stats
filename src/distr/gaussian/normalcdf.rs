/// Calculates Cumulative Distribution Function (CDF) on a Normal/Gaussian distribution <br>
/// Learn more about Normal Distributions at: <https://wikipedia.org/wiki/Normal_distribution#Definitions>
///
/// # Example:
///
/// ```
/// use rusty_stats::distr::gaussian::normalcdf;
/// let lower = 45 as f64;
/// let upper = 56 as f64;
/// let mean = 42 as f64;
/// let sd = 3.6;
///
/// let normalcdf = normalcdf(lower, upper, mean, sd);
///
/// assert_eq!(normalcdf, 0.20227802886072038);
/// ```
use crate::extra::erf::{ upper_erf, lower_erf };

pub fn normalcdf(lower: impl Into<f64> + Copy, upper: impl Into<f64> + Copy, mean: impl Into<f64> + Copy, sd: impl Into<f64> + Copy) -> f64 {
    let z1 = (lower.into() - mean.into()) / (sd.into() * std::f64::consts::SQRT_2);
    let z2 = (upper.into() - mean.into()) / (sd.into() * std::f64::consts::SQRT_2);

    let low_erf = lower_erf(z1);
    let up_erf = upper_erf(z2);

    (low_erf + up_erf) / 2_f64
}
