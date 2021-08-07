use std::fmt;

use crate::errors::{Result, TaError};
use crate::{Close, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Hull Moving Average (HMA).
///
/// A moving average that attemps to reduce or remove price lag while maintaining curve smoothness.
///
/// # Example
///
/// ```
/// use ta::indicators::HullMovingAverage;
/// use ta::Next;
///
/// let mut hma = HullMovingAverage::new(3).unwrap();
/// assert_eq!(hma.next(10.0), 10.0);
/// assert_eq!(hma.next(13.0), 12.0);
/// assert_eq!(hma.next(16.0), 14.0);
/// assert_eq!(hma.next(14.0), 14.5);
/// ```
///
/// # Links
///
/// * [Hull Moving Average, Alan Hull](https://alanhull.com/hull-moving-average)
///

#[doc(alias = "HMA")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct HullMovingAverage {
    period: usize,
}

impl HullMovingAverage {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                period,
            }),
        }
    }
}

impl Period for HullMovingAverage {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for HullMovingAverage {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        input
    }
}

impl<T: Close> Next<&T> for HullMovingAverage {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for HullMovingAverage {
    fn reset(&mut self) {

    }
}

impl Default for HullMovingAverage {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl fmt::Display for HullMovingAverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HMA({})", self.period)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(HullMovingAverage);

    #[test]
    fn test_new() {
        assert!(HullMovingAverage::new(0).is_err());
        assert!(HullMovingAverage::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut hma = HullMovingAverage::new(3).unwrap();

        assert_eq!(hma.next(12.0), 12.0);
        assert_eq!(hma.next(3.0), 6.0);
        assert_eq!(hma.next(3.0), 4.5);
        assert_eq!(hma.next(5.0), 4.0);

        let mut hma = HullMovingAverage::new(3).unwrap();
        let bar1 = Bar::new().close(2);
        let bar2 = Bar::new().close(5);
        assert_eq!(hma.next(&bar1), 2.0);
        assert_eq!(hma.next(&bar2), 4.0);
    }

    #[test]
    fn test_reset() {
        let mut hma = HullMovingAverage::new(5).unwrap();

        assert_eq!(hma.next(4.0), 4.0);
        hma.next(10.0);
        hma.next(15.0);
        hma.next(20.0);
        assert_ne!(hma.next(4.0), 4.0);

        hma.reset();
        assert_eq!(hma.next(4.0), 4.0);
    }

    #[test]
    fn test_default() {
        HullMovingAverage::default();
    }

    #[test]
    fn test_display() {
        let hma = HullMovingAverage::new(7).unwrap();
        assert_eq!(format!("{}", hma), "HMA(7)");
    }
}
