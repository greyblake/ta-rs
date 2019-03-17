use std::fmt;

use crate::errors::*;
use crate::{Close, Next, Reset};

/// A Bollinger Bands (BB).
/// (BB).
/// It is a type of infinite impulse response filter that calculates Bollinger Bands using Exponential Moving Average.
/// The Bollinger Badns are represented by Average EMA and standard deviaton that is moved 'k' times away in both directions from calculated average value.
///
/// # Formula
///
/// See EMA documentation.
///
/// ![SD formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/8715d659116bec91d48762b9e1f3d9aed36fc028)
///
/// Where:
///
/// * _SD<sub>s</sub>_ - is value of standard deviation for N given probes.
/// * _SD<sub>ٍx</sub>_  - is the mean value of observation.
/// * _SD<sub>N</sub>_ - is number of probes in observation.
/// * _SD<sud>xi</sub>_ - is i-th observed value from N elements observation.
///
/// and then BB is composed as:
///
///  * _BB<sub>Middle Band</sub>_ - Exponential Moving Average (EMA).
///  * _BB<sub>Upper Band</sub>_ = N * (EMA + SD of observation * multipler (usually 2.0))
///  * _BB<sub>Lower Band</sub>_ = N * (EMA - SD of observation * multipler (usually 2.0))
///
/// # Example
///
///```
/// use ta::indicators::{BollingerBands, BollingerBandsOutput};
/// use ta::Next;
///
/// let mut bb = BollingerBands::new(3, 2.0_f64).unwrap();
///
/// let out_0 = bb.next(2.0);
///
/// let out_1 = bb.next(5.0);
///
/// assert_eq!(out_0.average, 2.0);
/// assert_eq!(out_0.upper, 2.0);
/// assert_eq!(out_0.lower, 2.0);
///
/// assert_eq!(out_1.average, 3.5);
/// assert_eq!(out_1.upper, 6.5);
/// assert_eq!(out_1.lower, 0.5);
/// ```
///
/// # Links
///
/// ![Bollinger Bands, Wikipedia](https://en.wikipedia.org/wiki/Bollinger_Bands)
#[derive(Debug, Clone)]
pub struct BollingerBands {
    length: u32,
    multiplier: f64,
    values: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BollingerBandsOutput {
    pub average: f64,
    pub upper: f64,
    pub lower: f64,
}

impl BollingerBands {
    pub fn new(length: u32, multiplier: f64) -> Result<Self> {
        if length < 1 {
            return Err(Error::from_kind(ErrorKind::InvalidParameter));
        }
        Ok(Self {
            length,
            multiplier,
            values: Vec::with_capacity((length + 1) as usize),
        })
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn multiplier(&self) -> f64 {
        self.multiplier
    }
}

impl Next<f64> for BollingerBands {
    type Output = BollingerBandsOutput;

    fn next(&mut self, input: f64) -> Self::Output {
        self.values.push(input);

        if self.values.len() == 1 {
            return Self::Output {
                average: input,
                upper: input,
                lower: input,
            };
        }
        if self.values.len() == (self.length + 1) as usize {
            self.values.remove(0);
        }
        let (mean, sd) = mean_sd(&self.values);

        Self::Output {
            average: mean,
            upper: mean + sd * self.multiplier,
            lower: mean - sd * self.multiplier,
        }
    }
}

impl<'a, T: Close> Next<&'a T> for BollingerBands {
    type Output = BollingerBandsOutput;

    fn next(&mut self, input: &'a T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for BollingerBands {
    fn reset(&mut self) {
        self.values.clear();
    }
}

impl Default for BollingerBands {
    fn default() -> Self {
        Self::new(9, 2_f64).unwrap()
    }
}

impl fmt::Display for BollingerBands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BB({}, {})", self.length, self.multiplier)
    }
}

// Calculate mean and standard deviation
fn mean_sd(numbers: &[f64]) -> (f64, f64) {
    let sum: f64 = numbers.iter().sum();
    let size = numbers.len() as f64;
    let mean = sum / size;

    let quadratic_sum: f64 = numbers.iter().fold(0_f64, |a, v| a + (v - mean).powi(2));

    let sd = (quadratic_sum / size).sqrt();
    (mean, sd)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(BollingerBands);

    #[test]
    fn test_mean_sd() {
        // Test data are based on this online calculator:
        // https://www.mathsisfun.com/data/standard-deviation-calculator.html
        assert_eq!(round(mean_sd(&[5.55]).1), 0.0);
        assert_eq!(round(mean_sd(&[5.0, 6.0]).1), 0.5);
        assert_eq!(round(mean_sd(&[5.0, 6.0, 5.0]).1), 0.471);
        assert_eq!(round(mean_sd(&[5.0, 6.0, 5.0, 2.3]).1), 1.375);
    }

    #[test]
    fn test_new() {
        assert!(BollingerBands::new(0, 2_f64).is_err());
        assert!(BollingerBands::new(1, 2_f64).is_ok());
        assert!(BollingerBands::new(2, 2_f64).is_ok());
    }

    #[test]
    fn test_next() {
        let mut bb = BollingerBands::new(3, 2.0_f64).unwrap();

        let a = bb.next(2.0);
        let b = bb.next(5.0);
        let c = bb.next(1.0);
        let d = bb.next(6.25);

        assert_eq!(round(a.average), 2.0);
        assert_eq!(round(b.average), 3.5);
        assert_eq!(round(c.average), 2.667);
        assert_eq!(round(d.average), 4.083);

        assert_eq!(round(a.upper), 2.0);
        assert_eq!(round(b.upper), 6.5);
        assert_eq!(round(c.upper), 6.066);
        assert_eq!(round(d.upper), 8.562);

        assert_eq!(round(a.lower), 2.0);
        assert_eq!(round(b.lower), 0.5);
        assert_eq!(round(c.lower), -0.733);
        assert_eq!(round(d.lower), -0.395);
    }

    #[test]
    fn test_reset() {
        let mut bb = BollingerBands::new(5, 2.0_f64).unwrap();

        let out = bb.next(3.0);

        assert_eq!(out.average, 3.0);
        assert_eq!(out.upper, 3.0);
        assert_eq!(out.lower, 3.0);

        bb.next(2.5);
        bb.next(3.5);
        bb.next(4.0);

        let out = bb.next(2.0);

        assert_eq!(out.average, 3.0);
        assert_eq!(round(out.upper), 4.414);
        assert_eq!(round(out.lower), 1.586);

        bb.reset();
        let out = bb.next(3.0);
        assert_eq!(out.average, 3.0);
        assert_eq!(out.upper, 3.0);
        assert_eq!(out.lower, 3.0);
    }

    #[test]
    fn test_default() {
        BollingerBands::default();
    }

    #[test]
    fn test_display() {
        let _bb = BollingerBands::new(10, 3.0_f64).unwrap();
        assert_eq!(format!("{}", _bb), "BB(10, 3)");
    }
}
