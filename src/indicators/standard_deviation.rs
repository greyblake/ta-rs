use std::fmt;

use crate::errors::{Result, TaError};
use crate::{int, lit, Close, Next, NumberType, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "decimal")]
use sqrt::Sqrt;

/// Standard deviation (SD).
///
/// Returns the standard deviation of the last n values.
///
/// # Formula
///
/// ![SD formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/2845de27edc898d2a2a4320eda5f57e0dac6f650)
///
/// Where:
///
/// * _σ_ - value of standard deviation for N given probes.
/// * _N_ - number of probes in observation.
/// * _x<sub>i</sub>_ - i-th observed value from N elements observation.
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0)
///
/// # Example
///
/// ```
/// use ta::indicators::StandardDeviation;
/// use ta::Next;
///
/// let mut sd = StandardDeviation::new(3).unwrap();
/// assert_eq!(sd.next(10.0), 0.0);
/// assert_eq!(sd.next(20.0), 5.0);
/// ```
///
/// # Links
///
/// * [Standard Deviation, Wikipedia](https://en.wikipedia.org/wiki/Standard_deviation)
///
#[doc(alias = "SD")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct StandardDeviation {
    period: usize,
    index: usize,
    count: usize,
    m: NumberType,
    m2: NumberType,
    deque: Box<[NumberType]>,
}

impl StandardDeviation {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                period,
                index: 0,
                count: 0,
                m: lit!(0.0),
                m2: lit!(0.0),
                deque: vec![lit!(0.0); period].into_boxed_slice(),
            }),
        }
    }

    pub(super) fn mean(&self) -> NumberType {
        self.m
    }
}

impl Period for StandardDeviation {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<NumberType> for StandardDeviation {
    type Output = NumberType;

    fn next(&mut self, input: NumberType) -> Self::Output {
        let old_val = self.deque[self.index];
        self.deque[self.index] = input;

        self.index = if self.index + 1 < self.period {
            self.index + 1
        } else {
            0
        };

        if self.count < self.period {
            self.count += 1;
            let delta = input - self.m;
            self.m += delta / int!(self.count);
            let delta2 = input - self.m;
            self.m2 += delta * delta2;
        } else {
            let delta = input - old_val;
            let old_m = self.m;
            self.m += delta / int!(self.period);
            let delta2 = input - self.m + old_val - old_m;
            self.m2 += delta * delta2;
        }
        if self.m2 < lit!(0.0) {
            self.m2 = lit!(0.0);
        }

        (self.m2 / int!(self.count)).sqrt()
    }
}

impl<T: Close> Next<&T> for StandardDeviation {
    type Output = NumberType;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for StandardDeviation {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.m = lit!(0.0);
        self.m2 = lit!(0.0);
        for i in 0..self.period {
            self.deque[i] = lit!(0.0);
        }
    }
}

impl Default for StandardDeviation {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl fmt::Display for StandardDeviation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SD({})", self.period)
    }
}

#[cfg(feature = "decimal")]
mod sqrt {
    use crate::lit;
    use num_traits::Pow;
    use rust_decimal::Decimal;

    pub(super) trait Sqrt {
        fn sqrt(self) -> Self;
    }

    impl Sqrt for Decimal {
        fn sqrt(self) -> Self {
            self.pow(lit!(0.5))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(StandardDeviation);

    #[test]
    fn test_new() {
        assert!(StandardDeviation::new(0).is_err());
        assert!(StandardDeviation::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut sd = StandardDeviation::new(4).unwrap();
        assert_eq!(sd.next(lit!(10.0)), lit!(0.0));
        assert_eq!(sd.next(lit!(20.0)), lit!(5.0));
        assert_eq!(round(sd.next(lit!(30.0))), lit!(8.165));
        assert_eq!(round(sd.next(lit!(20.0))), lit!(7.071));
        assert_eq!(round(sd.next(lit!(10.0))), lit!(7.071));
        assert_eq!(round(sd.next(lit!(100.0))), lit!(35.355));
    }

    #[test]
    fn test_next_floating_point_error() {
        let mut sd = StandardDeviation::new(6).unwrap();
        assert_eq!(sd.next(lit!(1.872)), lit!(0.0));
        assert_eq!(round(sd.next(lit!(1.0))), lit!(0.436));
        assert_eq!(round(sd.next(lit!(1.0))), lit!(0.411));
        assert_eq!(round(sd.next(lit!(1.0))), lit!(0.378));
        assert_eq!(round(sd.next(lit!(1.0))), lit!(0.349));
        assert_eq!(round(sd.next(lit!(1.0))), lit!(0.325));
        assert_eq!(round(sd.next(lit!(1.0))), lit!(0.0));
    }

    #[test]
    fn test_next_with_bars() {
        fn bar(close: NumberType) -> Bar {
            Bar::new().close(close)
        }

        let mut sd = StandardDeviation::new(4).unwrap();
        assert_eq!(sd.next(&bar(lit!(10.0))), lit!(0.0));
        assert_eq!(sd.next(&bar(lit!(20.0))), lit!(5.0));
        assert_eq!(round(sd.next(&bar(lit!(30.0)))), lit!(8.165));
        assert_eq!(round(sd.next(&bar(lit!(20.0)))), lit!(7.071));
        assert_eq!(round(sd.next(&bar(lit!(10.0)))), lit!(7.071));
        assert_eq!(round(sd.next(&bar(lit!(100.0)))), lit!(35.355));
    }

    #[test]
    fn test_next_same_values() {
        let mut sd = StandardDeviation::new(3).unwrap();
        assert_eq!(sd.next(lit!(4.2)), lit!(0.0));
        assert_eq!(sd.next(lit!(4.2)), lit!(0.0));
        assert_eq!(sd.next(lit!(4.2)), lit!(0.0));
        assert_eq!(sd.next(lit!(4.2)), lit!(0.0));
    }

    #[test]
    fn test_reset() {
        let mut sd = StandardDeviation::new(4).unwrap();
        assert_eq!(sd.next(lit!(10.0)), lit!(0.0));
        assert_eq!(sd.next(lit!(20.0)), lit!(5.0));
        assert_eq!(round(sd.next(lit!(30.0))), lit!(8.165));

        sd.reset();
        assert_eq!(sd.next(lit!(20.0)), lit!(0.0));
    }

    #[test]
    fn test_default() {
        StandardDeviation::default();
    }

    #[test]
    fn test_display() {
        let sd = StandardDeviation::new(5).unwrap();
        assert_eq!(format!("{}", sd), "SD(5)");
    }
}
