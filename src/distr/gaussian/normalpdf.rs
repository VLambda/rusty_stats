/// Calculates a Probability Density Function (PDF) on a Normal/Gaussian distribution <br>
/// Learn more about Normal Distributions at: <https://wikipedia.org/wiki/Normal_distribution#Definitions>
/// <hr/>
///
/// # Example:
///
/// ```
/// use rusty_stats::distr::gaussian::normalpdf;
/// let xvalue = 0.5;
/// let mean = 0 as f64;
/// let sd = 1 as f64;
///
/// let normalpdf = normalpdf(xvalue,mean,sd);
///
/// assert_eq!(normalpdf, 0.3520653267642995);
/// ```
pub fn normalpdf(x_value: f64, mean: f64, sd: f64) -> f64 {
    (1.0 / (sd * (std::f64::consts::TAU).sqrt()))
        *
        (std::f64::consts::E).powf((-1.0 / 2.0) * ((x_value - mean) / sd).powi(2))
}