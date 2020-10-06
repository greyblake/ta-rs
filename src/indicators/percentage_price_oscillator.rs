use std::fmt;

use crate::errors::Result;
use crate::indicators::ExponentialMovingAverage as Ema;
use crate::{Close, Next, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Percentage Price Oscillator (PPO).
///
/// The PPO indicator (or "oscillator") is a collection of three time series
/// calculated from historical price data, most often the closing price.
/// These three series are:
///
/// * The PPO series proper
/// * The "signal" or "average" series
/// * The "divergence" series which is the difference between the two
///
/// The PPO series is the difference between a "fast" (short period) exponential
/// moving average (EMA), and a "slow" (longer period) EMA of the price series.
/// The average series is an EMA of the PPO series itself.
///
/// # Formula
///
/// # Parameters
///
/// * _fast_length_ - length for the fast EMA. Default is 12.
/// * _slow_length_ - length for the slow EMA. Default is 26.
/// * _signal_length_ - length for the signal EMA. Default is 9.
///
/// # Example
///
/// ```
/// use ta::indicators::PercentagePriceOscillator as Ppo;
/// use ta::Next;
///
/// let mut ppo = Ppo::new(3, 6, 4).unwrap();
///
/// assert_eq!(round(ppo.next(2.0).into()), (0.0, 0.0, 0.0));
/// assert_eq!(round(ppo.next(3.0).into()), (9.38, 3.75, 5.63));
/// assert_eq!(round(ppo.next(4.2).into()), (18.26, 9.56, 8.71));
/// assert_eq!(round(ppo.next(7.0).into()), (28.62, 17.18, 11.44));
/// assert_eq!(round(ppo.next(6.7).into()), (24.01, 19.91, 4.09));
/// assert_eq!(round(ppo.next(6.5).into()), (17.84, 19.08, -1.24));
///
/// fn round(nums: (f64, f64, f64)) -> (f64, f64, f64) {
///     let n0 = (nums.0 * 100.0).round() / 100.0;
///     let n1 = (nums.1 * 100.0).round() / 100.0;
///     let n2 = (nums.2 * 100.0).round() / 100.0;
///     (n0, n1, n2)
/// }
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct PercentagePriceOscillator {
    fast_ema: Ema,
    slow_ema: Ema,
    signal_ema: Ema,
}

impl PercentagePriceOscillator {
    pub fn new(fast_length: u32, slow_length: u32, signal_length: u32) -> Result<Self> {
        Ok(PercentagePriceOscillator {
            fast_ema: Ema::new(fast_length)?,
            slow_ema: Ema::new(slow_length)?,
            signal_ema: Ema::new(signal_length)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PercentagePriceOscillatorOutput {
    pub ppo: f64,
    pub signal: f64,
    pub histogram: f64,
}

impl From<PercentagePriceOscillatorOutput> for (f64, f64, f64) {
    fn from(po: PercentagePriceOscillatorOutput) -> Self {
        (po.ppo, po.signal, po.histogram)
    }
}

impl Next<f64> for PercentagePriceOscillator {
    type Output = PercentagePriceOscillatorOutput;

    fn next(&mut self, input: f64) -> Self::Output {
        let fast_val = self.fast_ema.next(input);
        let slow_val = self.slow_ema.next(input);

        let ppo = (fast_val - slow_val) / slow_val * 100.0;
        let signal = self.signal_ema.next(ppo);
        let histogram = ppo - signal;

        PercentagePriceOscillatorOutput {
            ppo,
            signal,
            histogram,
        }
    }
}

impl<T: Close> Next<&T> for PercentagePriceOscillator {
    type Output = PercentagePriceOscillatorOutput;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for PercentagePriceOscillator {
    fn reset(&mut self) {
        self.fast_ema.reset();
        self.slow_ema.reset();
        self.signal_ema.reset();
    }
}

impl Default for PercentagePriceOscillator {
    fn default() -> Self {
        Self::new(12, 26, 9).unwrap()
    }
}

impl fmt::Display for PercentagePriceOscillator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PPO({}, {}, {})",
            self.fast_ema.length(),
            self.slow_ema.length(),
            self.signal_ema.length()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;
    type Ppo = PercentagePriceOscillator;

    test_indicator!(Ppo);

    fn round(nums: (f64, f64, f64)) -> (f64, f64, f64) {
        let n0 = (nums.0 * 100.0).round() / 100.0;
        let n1 = (nums.1 * 100.0).round() / 100.0;
        let n2 = (nums.2 * 100.0).round() / 100.0;
        (n0, n1, n2)
    }

    #[test]
    fn test_new() {
        assert!(Ppo::new(0, 1, 1).is_err());
        assert!(Ppo::new(1, 0, 1).is_err());
        assert!(Ppo::new(1, 1, 0).is_err());
        assert!(Ppo::new(1, 1, 1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut ppo = Ppo::new(3, 6, 4).unwrap();

        assert_eq!(round(ppo.next(2.0).into()), (0.0, 0.0, 0.0));
        assert_eq!(round(ppo.next(3.0).into()), (9.38, 3.75, 5.63));
        assert_eq!(round(ppo.next(4.2).into()), (18.26, 9.56, 8.71));
        assert_eq!(round(ppo.next(7.0).into()), (28.62, 17.18, 11.44));
        assert_eq!(round(ppo.next(6.7).into()), (24.01, 19.91, 4.09));
        assert_eq!(round(ppo.next(6.5).into()), (17.84, 19.08, -1.24));
    }

    #[test]
    fn test_reset() {
        let mut ppo = Ppo::new(3, 6, 4).unwrap();

        assert_eq!(round(ppo.next(2.0).into()), (0.0, 0.0, 0.0));
        assert_eq!(round(ppo.next(3.0).into()), (9.38, 3.75, 5.63));

        ppo.reset();

        assert_eq!(round(ppo.next(2.0).into()), (0.0, 0.0, 0.0));
        assert_eq!(round(ppo.next(3.0).into()), (9.38, 3.75, 5.63));
    }

    #[test]
    fn test_default() {
        Ppo::default();
    }

    #[test]
    fn test_display() {
        let indicator = Ppo::new(13, 30, 10).unwrap();
        assert_eq!(format!("{}", indicator), "PPO(13, 30, 10)");
    }
}
