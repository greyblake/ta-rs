use std::fmt;

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
/// assert_eq!(out_1.upper, 7.742640687119285);
/// assert_eq!(out_1.lower, -0.7426406871192848);
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
        let mean = self.average.next(input);
        if self.values.len() == 1 {
            return Self::Output::new(mean, mean, mean);
        }
        if self.values.len() == self.length + 1 {
            self.values.remove(0);
        }
        let quadratic_sum: f64 = self.values.iter()
            .fold(0_f64, |a, v| a + (v - mean).powi(2) );
        let deviation = (quadratic_sum / (self.values.len() - 1) as f64).sqrt();
        Self::Output::new(
            mean,
            mean + deviation * self.distance_multiplier,
            mean - deviation * self.distance_multiplier)
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
        self.values = Vec::new();
        self.average = ExponentialMovingAverage::new(self.length as u32).unwrap();
    }
}

impl Default for BollingerBands {
    fn default() -> Self {
        Self::new(9, 2_f64).unwrap()
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

    #[test]
    fn test_next() {
        let mut _bb = BollingerBands::new(3, 2.0_f64).unwrap();

        let a = _bb.next(2.0);
        let b = _bb.next(5.0);
        let c = _bb.next(1.0);
        let d = _bb.next(6.25);

        assert_eq!(a.average, 2.0);
        assert_eq!(b.average, 3.5);
        assert_eq!(c.average, 2.25);
        assert_eq!(d.average, 4.25);

        assert_eq!(a.upper, 2.0);
        assert_eq!(b.upper, 7.742640687119285);
        assert_eq!(c.upper, 6.536607049870562);
        assert_eq!(d.upper, 9.75);

        assert_eq!(a.lower, 2.0);
        assert_eq!(b.lower, -0.7426406871192848);
        assert_eq!(c.lower, -2.036607049870562);
        assert_eq!(d.lower, -1.25);

    }

    #[test]
    fn test_reset() {
        let mut _bb = BollingerBands::new(5, 2.0_f64).unwrap();

        let out = _bb.next(3.0);

        assert_eq!(out.average, 3.0);
        assert_eq!(out.upper, 3.0);
        assert_eq!(out.lower, 3.0);

        _bb.next(2.5);
        _bb.next(3.5);
        _bb.next(4.0);

        let out = _bb.next(2.0);

        assert_eq!(out.average, 2.9135802469135803);
        assert_eq!(out.upper, 4.506483843688222);
        assert_eq!(out.lower, 1.320676650138939);

        _bb.reset();
        let out = _bb.next(4.0);

        assert_eq!(out.average, 4.0);
        assert_eq!(out.upper, 4.0);
        assert_eq!(out.lower, 4.0);

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
