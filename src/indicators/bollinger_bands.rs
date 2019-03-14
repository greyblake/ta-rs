use std::fmt;
use std::cmp::PartialEq;

use crate::errors::*;
use crate::{Close, Next, Reset};
use crate::indicators::ExponentialMovingAverage;

/// A Bollinger Bands (BB).
/// (BB).
/// It is a type of infinite impulse response filter that calculates Bollinger Bands using Exponential Moving Average.
/// The Bollinger Badns are represented by Average EMA and standard deviaton that is moved 'k' times away in both directions from calculated average value.
/// 
/// # Formula
///
/// Bollinger Bands are calculated based on EMA combined with Standard Deviaiation(SD).
///
/// See EMA doumentation.
///
/// ![SD formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/8715d659116bec91d48762b9e1f3d9aed36fc028)
///
/// Where:
///
/// * _SD<sub>s</sub>_ - is value of standard deviation for N given probes.
/// * _SD<sub>ٍ-</sub>_  - is the mean value of observation.
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
/// let mut bb = BollingerBands::new(20, 2.0_f64).unwrap();
///
///
/// assert_eq!(bb.next(2.0), BollingerBandsOutput {
///     average: 2.0_f64,
///     upper: 2.0_f64,
///     lower: 2.0_f64,
/// });
///
/// assert_eq!(bb.next(4.0), BollingerBandsOutput {
///     average: 3.0_f64,
///     upper: 0.0_f64,
///     lower: 4.0_f64,
/// });
/// ```
///
/// # Links
///
/// ![Bollinger Bands, Wikipedia](https://en.wikipedia.org/wiki/Bollinger_Bands)
///
///

#[derive(Debug, Clone)]
pub struct BollingerBandsOutput {
    pub average: f64,
    pub upper: f64,
    pub lower: f64,
}

impl BollingerBandsOutput {
    pub fn new(average: f64, upper: f64, lower: f64) -> Self {
        Self {average, upper, lower}
    }
}

impl PartialEq for BollingerBandsOutput {
    fn eq(&self, other: &Self) -> bool {
        self.average == other.average &&
            self.upper == self.upper &&
            self.lower == self.lower
    }
}

#[derive(Debug, Clone)]
pub struct BollingerBands {
    length: usize,
    distance_multiplier: f64,
    values: Vec<f64>,
    average: ExponentialMovingAverage,
}

impl BollingerBands {
    pub fn new(length: usize, distance_multiplier: f64) -> Result<Self> {
        match length {
            0 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            1 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            _ => {
                Ok(Self {
                    length,
                    distance_multiplier,
                    values: Vec::new(),
                    average: ExponentialMovingAverage::new(length as u32).unwrap(),
                })
            }
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn distance_multiplier(&self) -> f64 {
        self.distance_multiplier
    }
}

impl Next<f64> for BollingerBands {
    type Output = BollingerBandsOutput;

    fn next(&mut self, input: f64) -> Self::Output {
        self.values.push(input);
        if self.values.len() == 1 {
            self.average.next(input);
            return Self::Output::new(input, input, input);
        }
        if self.values.len() == self.length + 1 {
            self.values.remove(0);
        }
        let mean = self.average.next(input);
        let mut quadratic_sum = 0_f64;
        for v in &self.values {
            quadratic_sum += (*v - mean).powi(2);
        }
        let deviation = (quadratic_sum / (self.values.len() - 1) as f64).sqrt();
        Self::Output::new(
            mean,
            mean + deviation * self.distance_multiplier,
            mean - deviation * self.distance_multiplier)
    }
}

impl fmt::Display for BollingerBands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BB({}, {})", self.length, self.distance_multiplier)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    // test_indicator!(BollingerBands);

    #[test]
    fn test_new() {
        assert!(BollingerBands::new(0, 2_f64).is_err());
        assert!(BollingerBands::new(1, 2_f64).is_err());
        assert!(BollingerBands::new(2, 2_f64).is_ok());
    }

    fn test_next() {
        let mut bb = BollingerBands::new(4, 1.0_f64).unwrap();

        assert_eq!(bb.next(2.0), BollingerBandsOutput::new(2.0_f64, 2.0_f64, 2.0_f64));
        assert_eq!(bb.next(5.0), BollingerBandsOutput::new(3.5_f64, 2.0_f64, 5.0_f64));

    }

}
