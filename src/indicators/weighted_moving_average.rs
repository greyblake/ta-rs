use std::fmt;

use crate::errors::{Result, TaError};
use crate::{Close, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Weighted moving average (WMA).
///
/// A moving average that assigns weights that decrease in arithmetical
/// progression. In an _n_-day WMA the latest day has weight _n_, the second
/// latest _n−1_, etc., down to one.
///
/// # Formula
///
/// ![WMA formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/7780333af18da7e27a1186a3d566e28da21b2840)
///
/// Where:
///
/// * _WMA<sub>M</sub>_ - is the value of the WMA at time _m_
/// * _n_ - is the period.
/// * _p<sub>M</sub>_ - is the input value at a time period t.
///
/// # Example
///
/// ```
/// use ta::indicators::WeightedMovingAverage;
/// use ta::Next;
///
/// let mut wma = WeightedMovingAverage::new(3).unwrap();
/// assert_eq!(wma.next(10.0), 10.0);
/// assert_eq!(wma.next(13.0), 12.0);
/// assert_eq!(wma.next(16.0), 14.0);
/// assert_eq!(wma.next(14.0), 14.5);
/// ```
///
/// # Links
///
/// * [Weighted moving average, Wikipedia](https://en.wikipedia.org/wiki/Moving_average#Weighted_moving_average)

#[doc(alias = "WMA")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct WeightedMovingAverage {
    period: usize,
    index: usize,
    count: usize,
    weight: f64,
    sum: f64,
    sum_flat: f64,
    deque: Box<[f64]>,
}

impl WeightedMovingAverage {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                period,
                index: 0,
                count: 0,
                weight: 0.0,
                sum: 0.0,
                sum_flat: 0.0,
                deque: vec![0.0; period].into_boxed_slice(),
            }),
        }
    }
}

impl Period for WeightedMovingAverage {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for WeightedMovingAverage {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let old_val: f64 = self.deque[self.index];
        self.deque[self.index] = input;

        self.index = if self.index + 1 < self.period {
            self.index + 1
        } else {
            0
        };

        if self.count < self.period {
            self.count += 1;
            self.weight = self.count as f64;
            self.sum += input * self.weight
        } else {
            self.sum = self.sum - self.sum_flat + (input * self.weight);
        }
        self.sum_flat = self.sum_flat - old_val + input;
        self.sum / (self.weight * (self.weight + 1.0) / 2.0)
    }
}

impl<T: Close> Next<&T> for WeightedMovingAverage {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for WeightedMovingAverage {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.weight = 0.0;
        self.sum = 0.0;
        self.sum_flat = 0.0;
        for i in 0..self.period {
            self.deque[i] = 0.0;
        }
    }
}

impl Default for WeightedMovingAverage {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl fmt::Display for WeightedMovingAverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WMA({})", self.period)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(WeightedMovingAverage);

    #[test]
    fn test_new() {
        assert!(WeightedMovingAverage::new(0).is_err());
        assert!(WeightedMovingAverage::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut wma = WeightedMovingAverage::new(3).unwrap();

        assert_eq!(wma.next(12.0), 12.0);
        assert_eq!(wma.next(3.0), 6.0);
        assert_eq!(wma.next(3.0), 4.5);
        assert_eq!(wma.next(5.0), 4.0);

        let mut wma = WeightedMovingAverage::new(3).unwrap();
        let bar1 = Bar::new().close(2);
        let bar2 = Bar::new().close(5);
        assert_eq!(wma.next(&bar1), 2.0);
        assert_eq!(wma.next(&bar2), 4.0);
    }

    #[test]
    fn test_reset() {
        let mut wma = WeightedMovingAverage::new(5).unwrap();

        assert_eq!(wma.next(4.0), 4.0);
        wma.next(10.0);
        wma.next(15.0);
        wma.next(20.0);
        assert_ne!(wma.next(4.0), 4.0);

        wma.reset();
        assert_eq!(wma.next(4.0), 4.0);
    }

    #[test]
    fn test_default() {
        WeightedMovingAverage::default();
    }

    #[test]
    fn test_display() {
        let wma = WeightedMovingAverage::new(7).unwrap();
        assert_eq!(format!("{}", wma), "WMA(7)");
    }
}
