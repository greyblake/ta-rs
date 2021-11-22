use std::fmt;

use crate::errors::Result;
use crate::indicators::{ExponentialMovingAverage, FastStochastic};
use crate::{Close, High, Low, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Slow stochastic oscillator.
///
/// Basically it is a fast stochastic oscillator smoothed with exponential
/// moving average.
///
/// # Parameters
///
/// * `stochastic_period` - number of periods for fast stochastic (integer
///   greater than 0). Default is 14.
/// * `ema_period` - period for EMA (integer greater than 0). Default is 3.
///
/// # Example
///
/// ```
/// use ta::indicators::SlowStochastic;
/// use ta::Next;
///
/// let mut stoch = SlowStochastic::new(3, 2).unwrap();
/// assert_eq!(stoch.next(10.0), 50.0);
/// assert_eq!(stoch.next(50.0).round(), 83.0);
/// assert_eq!(stoch.next(50.0).round(), 94.0);
/// assert_eq!(stoch.next(30.0).round(), 31.0);
/// assert_eq!(stoch.next(55.0).round(), 77.0);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct SlowStochastic {
    fast_stochastic: FastStochastic,
    ema: ExponentialMovingAverage,
}

impl SlowStochastic {
    pub fn new(stochastic_period: usize, ema_period: usize) -> Result<Self> {
        Ok(Self {
            fast_stochastic: FastStochastic::new(stochastic_period)?,
            ema: ExponentialMovingAverage::new(ema_period)?,
        })
    }
}

impl Next<f64> for SlowStochastic {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.ema.next(self.fast_stochastic.next(input))
    }
}

impl<T: High + Low + Close> Next<&T> for SlowStochastic {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.ema.next(self.fast_stochastic.next(input))
    }
}

impl Reset for SlowStochastic {
    fn reset(&mut self) {
        self.fast_stochastic.reset();
        self.ema.reset();
    }
}

impl Default for SlowStochastic {
    fn default() -> Self {
        Self::new(14, 3).unwrap()
    }
}

impl fmt::Display for SlowStochastic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SLOW_STOCH({}, {})",
            self.fast_stochastic.period(),
            self.ema.period()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(SlowStochastic);

    #[test]
    fn test_new() {
        assert!(SlowStochastic::new(0, 1).is_err());
        assert!(SlowStochastic::new(1, 0).is_err());
        assert!(SlowStochastic::new(1, 1).is_ok());
    }

    #[test]
    fn test_next_with_f64() {
        let mut stoch = SlowStochastic::new(3, 2).unwrap();
        assert_eq!(stoch.next(10.0), 50.0);
        assert_eq!(stoch.next(50.0).round(), 83.0);
        assert_eq!(stoch.next(50.0).round(), 94.0);
        assert_eq!(stoch.next(30.0).round(), 31.0);
        assert_eq!(stoch.next(55.0).round(), 77.0);
    }

    #[test]
    fn test_next_with_bars() {
        let test_data = vec![
            // high, low , close, expected
            (30.0, 10.0, 25.0, 75.0),
            (20.0, 20.0, 20.0, 58.0),
            (40.0, 20.0, 16.0, 33.0),
            (35.0, 15.0, 19.0, 22.0),
            (30.0, 20.0, 25.0, 34.0),
            (35.0, 25.0, 30.0, 61.0),
        ];

        let mut stoch = SlowStochastic::new(3, 2).unwrap();

        for (high, low, close, expected) in test_data {
            let input_bar = Bar::new().high(high).low(low).close(close);
            assert_eq!(stoch.next(&input_bar).round(), expected);
        }
    }

    #[test]
    fn test_reset() {
        let mut stoch = SlowStochastic::new(3, 2).unwrap();
        assert_eq!(stoch.next(10.0), 50.0);
        assert_eq!(stoch.next(50.0).round(), 83.0);
        assert_eq!(stoch.next(50.0).round(), 94.0);

        stoch.reset();
        assert_eq!(stoch.next(10.0), 50.0);
    }

    #[test]
    fn test_default() {
        SlowStochastic::default();
    }

    #[test]
    fn test_display() {
        let indicator = SlowStochastic::new(10, 2).unwrap();
        assert_eq!(format!("{}", indicator), "SLOW_STOCH(10, 2)");
    }
}
