/// This function recreates the InvNorm function in the Ti-83 & 84 calculators. <br>
/// This is not to be confused with an Inverse Gaussian Distribution though. <br>
/// This uses an approximation of the Inverse Error Function found at: <https://scistatcalc.blogspot.com/2013/09/numerical-estimate-of-inverse-error.html> <br>
/// Learn more at: <https://www.statology.org/inverse-normal-distribution/>
///
/// #Example #1: Right Side Non-Standard Normal Distribution
///
/// ```
/// use rusty_stats::distr::gaussian::invnorm;
///
/// let area = 0.589255651;
/// let mean = 42;
/// let sd = 3.6;
/// let tail = "Right";
///
/// let invnorm = invnorm(area, mean, sd, tail).unwrap();
/// assert_eq!(invnorm, 39_f64);
/// ```
///
/// #Example #2: Left Side Standard Normal Distribution
///
/// ```
/// use rusty_stats::distr::gaussian::invnorm;
///
/// let area = 0.975;
/// let mean = 0;
/// let sd = 1;
/// let tail = "Left";
///
/// let invnorm = invnorm(area, mean, sd, tail).unwrap();
/// assert_eq!(invnorm, 1.9599639845401289);
/// ```
///

use crate::extra::erfinv::inverse_error_function;

pub fn invnorm<T>(area: f64, mean: impl Into<f64> + Copy, sd: impl Into<f64> + Copy, tail: T) -> Result<f64, String>
    where
        T: AsRef<str> + Copy,
{
    let tail_str = tail.as_ref();

    let function: f64 = if sd.into() != 1_f64 && mean.into() != 0_f64 {
        area * sd.into() * std::f64::consts::SQRT_2
    } else {
        std::f64::consts::SQRT_2 * (inverse_error_function(-2_f64 * (area) + 1_f64))
    };

    let tail_val: f64 = if tail_str == "Right" || tail_str == "right" {
        function + mean.into()
    } else if tail_str == "Left" || tail_str == "left" {
        (-1_f64 * function) + mean.into()
    } else {
        return Err("Tail side Invalid, the only options are:\nRight\nright\nLeft\nleft".to_string());
    };

    let rounded_val = if (tail_val.ceil() - 1e-7) < tail_val || (tail_val.floor() + 1e-7) > tail_val {
        (tail_val * 1e+7).round() / 1e+7
    } else {
        tail_val
    };

    Ok(rounded_val)
}