use std::fmt;

use crate::errors::*;
use crate::indicators::StandardDeviation as Sd;
use crate::{Close, Next, Reset};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// A Bollinger Bands (BB).
/// (BB).
/// It is a type of infinite impulse response filter that calculates Bollinger Bands using Exponential Moving Average.
/// The Bollinger Bands are represented by Average EMA and standard deviaton that is moved 'k' times away in both directions from calculated average value.
///
/// # Formula
///
/// See SMA, SD documentation.
///
/// BB is composed as:
///
///  * _BB<sub>Middle Band</sub>_ - Simple Moving Average (SMA).
///  * _BB<sub>Upper Band</sub>_ = SMA + SD of observation * multipler (usually 2.0)
///  * _BB<sub>Lower Band</sub>_ = SMA - SD of observation * multipler (usually 2.0)
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
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BollingerBands {
    length: u32,
    multiplier: f64,
    sd: Sd,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BollingerBandsOutput {
    pub average: f64,
    pub upper: f64,
    pub lower: f64,
}

impl BollingerBands {
    pub fn new(length: u32, multiplier: f64) -> Result<Self> {
        if multiplier <= 0.0 {
            return Err(Error::from_kind(ErrorKind::InvalidParameter));
        }
        Ok(Self {
            length,
            multiplier,
            sd: Sd::new(length)?,
        })
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn multiplier(&self) -> f64 {
        self.multiplier
    }
}

impl<'a> Next<f64> for BollingerBands {
    type Output = BollingerBandsOutput;

    fn next(&mut self, input: f64) -> Self::Output {
        let sd = self.sd.next(input);
        let mean = self.sd.mean();

        Self::Output {
            average: mean,
            upper: mean + sd * self.multiplier,
            lower: mean - sd * self.multiplier,
        }
    }
}

impl<'a, T: Close> Next<&'a T> for BollingerBands {
    type Output = BollingerBandsOutput;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for BollingerBands {
    fn reset(&mut self) {
        self.sd.reset();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(BollingerBands);

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
        let bb = BollingerBands::new(10, 3.0_f64).unwrap();
        assert_eq!(format!("{}", bb), "BB(10, 3)");
    }
}
