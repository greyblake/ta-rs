use std::fmt;

use crate::errors::Result;
use crate::indicators::{ExponentialMovingAverage, FastStochastic};
use crate::{Close, High, Low, Next, Reset};

/// Slow stochastic oscillator.
///
/// Basically it is a fast stochastic oscillator smoothed with exponential moving average.
///
/// # Parameters
///
/// * _stochastic_n_ - number of periods for fast stochastic (integer greater than 0). Default is 14.
/// *_ema_n_ - length for EMA (integer greater than 0). Default is 3.
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
#[derive(Clone, Debug)]
pub struct SlowStochastic {
    fast_stochastic: FastStochastic,
    ema: ExponentialMovingAverage,
}

impl SlowStochastic {
    pub fn new(stochastic_n: u32, ema_n: u32) -> Result<Self> {
        let indicator = Self {
            fast_stochastic: FastStochastic::new(stochastic_n)?,
            ema: ExponentialMovingAverage::new(ema_n)?,
        };
        Ok(indicator)
    }
}

impl Next<f64> for SlowStochastic {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.ema.next(self.fast_stochastic.next(input))
    }
}

impl<'a, T: High + Low + Close> Next<&'a T> for SlowStochastic {
    type Output = f64;

    fn next(&mut self, input: &'a T) -> Self::Output {
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
            self.fast_stochastic.length(),
            self.ema.length()
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
