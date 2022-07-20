use crate::{errors::Result, indicators::DirectionalMovementIndex, High, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

/// Average Directional Index (ADX)
///
/// A direction indicator, originally developed by J. Welles Wilder. The
/// average directional movement index is an N-sample smoothed moving average of
/// a combination of positive & negative directional indicator (DI) values.
///
/// # Parameters
///
/// * `period` - Smoothing period (samples) of SDM and ATR (nonzero integer)
///   used in the DIs.
///
/// # Links
///
/// * [Averager directional movement index, Wikipedia](https://en.wikipedia.org/wiki/Average_directional_movement_index)
#[doc(alias = "ADX")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct AverageDirectionalIndex {
    previous: f64,
    dx: DirectionalMovementIndex,
}

impl AverageDirectionalIndex {
    pub fn new(period: usize) -> Result<Self> {
        Ok(Self {
            previous: 0.0,
            dx: DirectionalMovementIndex::new(period)?,
        })
    }
}

impl Period for AverageDirectionalIndex {
    fn period(&self) -> usize {
        self.dx.period()
    }
}

impl Next<f64> for AverageDirectionalIndex {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let current = self.dx.next(input);
        let adx = (self.previous * (self.dx.period() - 1) as f64 + current) / self.dx.period() as f64;
        self.previous = current;

        adx
    }
}

impl<T: High> Next<&T> for AverageDirectionalIndex {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.high())
    }
}

impl Reset for AverageDirectionalIndex {
    fn reset(&mut self) {
        self.previous = 0.0;
        self.dx.reset()
    }
}

impl Default for AverageDirectionalIndex {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for AverageDirectionalIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ADX({})", self.period())
    }
}

// TODO: implement AverageDirectionalIndexDetailed where next() returns a tuple
// of (DI-, ADX, DI+)

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(ExponentialMovingAverage);

    #[test]
    fn test_new() {
        assert!(ExponentialMovingAverage::new(0).is_err());
        assert!(ExponentialMovingAverage::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut ema = ExponentialMovingAverage::new(3).unwrap();

        assert_eq!(ema.next(2.0), 2.0);
        assert_eq!(ema.next(5.0), 3.5);
        assert_eq!(ema.next(1.0), 2.25);
        assert_eq!(ema.next(6.25), 4.25);

        let mut ema = ExponentialMovingAverage::new(3).unwrap();
        let bar1 = Bar::new().close(2);
        let bar2 = Bar::new().close(5);
        assert_eq!(ema.next(&bar1), 2.0);
        assert_eq!(ema.next(&bar2), 3.5);
    }

    #[test]
    fn test_reset() {
        let mut ema = ExponentialMovingAverage::new(5).unwrap();

        assert_eq!(ema.next(4.0), 4.0);
        ema.next(10.0);
        ema.next(15.0);
        ema.next(20.0);
        assert_ne!(ema.next(4.0), 4.0);

        ema.reset();
        assert_eq!(ema.next(4.0), 4.0);
    }

    #[test]
    fn test_default() {
        ExponentialMovingAverage::default();
    }

    #[test]
    fn test_display() {
        let ema = ExponentialMovingAverage::new(7).unwrap();
        assert_eq!(format!("{}", ema), "EMA(7)");
    }
}
*/
