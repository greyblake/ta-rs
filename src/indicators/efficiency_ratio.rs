use std::collections::VecDeque;
use std::fmt;

use crate::errors::{Error, ErrorKind, Result};
use crate::traits::{Close, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Kaufman's Efficiency Ratio (ER).
///
/// It is calculated by dividing the price change over a period by the absolute sum of the price movements that occurred to achieve that change.
/// The resulting ratio ranges between 0.0 and 1.0 with higher values representing a more efficient or trending market.
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0)
///
/// # Example
///
/// ```
/// use ta::indicators::EfficiencyRatio;
/// use ta::Next;
///
/// let mut er = EfficiencyRatio::new(4).unwrap();
/// assert_eq!(er.next(10.0), 1.0);
/// assert_eq!(er.next(13.0), 1.0);
/// assert_eq!(er.next(12.0), 0.5);
/// assert_eq!(er.next(13.0), 0.6);
/// assert_eq!(er.next(18.0), 0.8);
/// assert_eq!(er.next(19.0), 0.75);
/// ```

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EfficiencyRatio {
    period: usize,
    prices: VecDeque<f64>,
}

impl EfficiencyRatio {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            _ => Ok(Self {
                period,
                prices: VecDeque::with_capacity(period + 1),
            }),
        }
    }
}

impl Period for EfficiencyRatio {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for EfficiencyRatio {
    type Output = f64;

    fn next(&mut self, input: f64) -> f64 {
        self.prices.push_back(input);

        if self.prices.len() <= 2 {
            return 1.0;
        }

        let first = self.prices[0];

        // Calculate volatility
        let volatility = self
            .prices
            .iter()
            .skip(1)
            .fold((first, 0.0), |(prev, sum), &val| {
                (val, sum + (prev - val).abs())
            })
            .1;

        // Calculate direction
        let last_index = self.prices.len() - 1;
        let direction = (first - self.prices[last_index]).abs();

        // Get rid of the first element
        if self.prices.len() > self.period {
            self.prices.pop_front();
        }

        // Return actual efficiency ratio
        direction / volatility
    }
}

impl<T: Close> Next<&T> for EfficiencyRatio {
    type Output = f64;

    fn next(&mut self, input: &T) -> f64 {
        self.next(input.close())
    }
}

impl Reset for EfficiencyRatio {
    fn reset(&mut self) {
        self.prices.clear();
    }
}

impl Default for EfficiencyRatio {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for EfficiencyRatio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ER({})", self.period)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(EfficiencyRatio);

    #[test]
    fn test_new() {
        assert!(EfficiencyRatio::new(0).is_err());
        assert!(EfficiencyRatio::new(1).is_ok());
    }

    #[test]
    fn test_next_f64() {
        let mut er = EfficiencyRatio::new(3).unwrap();

        assert_eq!(round(er.next(3.0)), 1.0);
        assert_eq!(round(er.next(5.0)), 1.0);
        assert_eq!(round(er.next(2.0)), 0.2);
        assert_eq!(round(er.next(3.0)), 0.0);
        assert_eq!(round(er.next(1.0)), 0.667);
        assert_eq!(round(er.next(3.0)), 0.2);
        assert_eq!(round(er.next(4.0)), 0.2);
        assert_eq!(round(er.next(6.0)), 1.0);

        er.reset();
        assert_eq!(round(er.next(3.0)), 1.0);
        assert_eq!(round(er.next(5.0)), 1.0);
        assert_eq!(round(er.next(2.0)), 0.2);
        assert_eq!(round(er.next(3.0)), 0.0);
    }

    #[test]
    fn test_display() {
        let er = EfficiencyRatio::new(17).unwrap();
        assert_eq!(format!("{}", er), "ER(17)");
    }
}
