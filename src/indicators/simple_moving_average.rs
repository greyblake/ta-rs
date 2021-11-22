use std::fmt;

use crate::errors::{Result, TaError};
use crate::{Close, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Simple moving average (SMA).
///
/// # Formula
///
/// ![SMA](https://wikimedia.org/api/rest_v1/media/math/render/svg/e2bf09dc6deaf86b3607040585fac6078f9c7c89)
///
/// Where:
///
/// * _SMA<sub>t</sub>_ - value of simple moving average at a point of time _t_
/// * _period_ - number of periods (period)
/// * _p<sub>t</sub>_ - input value at a point of time _t_
///
/// # Parameters
///
/// * `period` - Number of periods (integer greater than 0).
///
/// # Example
///
/// ```
/// use ta::indicators::SimpleMovingAverage;
/// use ta::Next;
///
/// let mut sma = SimpleMovingAverage::new(3).unwrap();
/// assert_eq!(sma.next(10.0), 10.0);
/// assert_eq!(sma.next(11.0), 10.5);
/// assert_eq!(sma.next(12.0), 11.0);
/// assert_eq!(sma.next(13.0), 12.0);
/// ```
///
/// # Links
///
/// * [Simple Moving Average, Wikipedia](https://en.wikipedia.org/wiki/Moving_average#Simple_moving_average)
#[doc(alias = "SMA")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct SimpleMovingAverage {
    period: usize,
    index: usize,
    count: usize,
    sum: f64,
    deque: Box<[f64]>,
}

impl SimpleMovingAverage {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                period,
                index: 0,
                count: 0,
                sum: 0.0,
                deque: vec![0.0; period].into_boxed_slice(),
            }),
        }
    }
}

impl Period for SimpleMovingAverage {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for SimpleMovingAverage {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let old_val = self.deque[self.index];
        self.deque[self.index] = input;

        self.index = if self.index + 1 < self.period {
            self.index + 1
        } else {
            0
        };

        if self.count < self.period {
            self.count += 1;
        }

        self.sum = self.sum - old_val + input;
        self.sum / (self.count as f64)
    }
}

impl<T: Close> Next<&T> for SimpleMovingAverage {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for SimpleMovingAverage {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.sum = 0.0;
        for i in 0..self.period {
            self.deque[i] = 0.0;
        }
    }
}

impl Default for SimpleMovingAverage {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl fmt::Display for SimpleMovingAverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SMA({})", self.period)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(SimpleMovingAverage);

    #[test]
    fn test_new() {
        assert!(SimpleMovingAverage::new(0).is_err());
        assert!(SimpleMovingAverage::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut sma = SimpleMovingAverage::new(4).unwrap();
        assert_eq!(sma.next(4.0), 4.0);
        assert_eq!(sma.next(5.0), 4.5);
        assert_eq!(sma.next(6.0), 5.0);
        assert_eq!(sma.next(6.0), 5.25);
        assert_eq!(sma.next(6.0), 5.75);
        assert_eq!(sma.next(6.0), 6.0);
        assert_eq!(sma.next(2.0), 5.0);
    }

    #[test]
    fn test_next_with_bars() {
        fn bar(close: f64) -> Bar {
            Bar::new().close(close)
        }

        let mut sma = SimpleMovingAverage::new(3).unwrap();
        assert_eq!(sma.next(&bar(4.0)), 4.0);
        assert_eq!(sma.next(&bar(4.0)), 4.0);
        assert_eq!(sma.next(&bar(7.0)), 5.0);
        assert_eq!(sma.next(&bar(1.0)), 4.0);
    }

    #[test]
    fn test_reset() {
        let mut sma = SimpleMovingAverage::new(4).unwrap();
        assert_eq!(sma.next(4.0), 4.0);
        assert_eq!(sma.next(5.0), 4.5);
        assert_eq!(sma.next(6.0), 5.0);

        sma.reset();
        assert_eq!(sma.next(99.0), 99.0);
    }

    #[test]
    fn test_default() {
        SimpleMovingAverage::default();
    }

    #[test]
    fn test_display() {
        let sma = SimpleMovingAverage::new(5).unwrap();
        assert_eq!(format!("{}", sma), "SMA(5)");
    }
}
