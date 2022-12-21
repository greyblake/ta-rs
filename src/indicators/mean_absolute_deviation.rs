use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::errors::{Result, TaError};
use crate::{int, lit, Close, Next, NumberType, Period, Reset};

/// Mean Absolute Deviation (MAD)
///
/// The mean absolute deviation of a data set is the average of the absolute deviations from a
/// central point. It is a summary statistic of statistical dispersion or variability.
/// In the general form, the central point can be a mean, median, mode, or the result of any other
/// measure of central tendency or any random data point related to the given data set.
/// The absolute values of the differences between the data points and their central tendency are
/// totaled and divided by the number of data points.
///
/// # Formula
///
/// MAD(_period_) = { x<sub>1</sub> - ABS(AVG(_period_)), ..., x<sub>_period_</sub> - ABS(AVG(_period_)) } / _period_
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0). Default is 9.
///
/// # Links
///
/// * [Mean Absolute Deviation, Wikipedia](https://en.wikipedia.org/wiki/Mean_absolute_deviation)
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct MeanAbsoluteDeviation {
    period: usize,
    index: usize,
    count: usize,
    sum: NumberType,
    deque: Box<[NumberType]>,
}

impl MeanAbsoluteDeviation {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                period,
                index: 0,
                count: 0,
                sum: lit!(0.0),
                deque: vec![lit!(0.0); period].into_boxed_slice(),
            }),
        }
    }
}

impl Period for MeanAbsoluteDeviation {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<NumberType> for MeanAbsoluteDeviation {
    type Output = NumberType;

    fn next(&mut self, input: NumberType) -> Self::Output {
        self.sum = if self.count < self.period {
            self.count += 1;
            self.sum + input
        } else {
            self.sum + input - self.deque[self.index]
        };

        self.deque[self.index] = input;
        self.index = if self.index + 1 < self.period {
            self.index + 1
        } else {
            0
        };

        let mean = self.sum / int!(self.count);

        let mut mad = lit!(0.0);
        for value in &self.deque[..self.count] {
            mad += (value - mean).abs();
        }
        mad / int!(self.count)
    }
}

impl<T: Close> Next<&T> for MeanAbsoluteDeviation {
    type Output = NumberType;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for MeanAbsoluteDeviation {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.sum = lit!(0.0);
        for i in 0..self.period {
            self.deque[i] = lit!(0.0);
        }
    }
}

impl Default for MeanAbsoluteDeviation {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl fmt::Display for MeanAbsoluteDeviation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MAD({})", self.period)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(MeanAbsoluteDeviation);

    #[test]
    fn test_new() {
        assert!(MeanAbsoluteDeviation::new(0).is_err());
        assert!(MeanAbsoluteDeviation::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut mad = MeanAbsoluteDeviation::new(5).unwrap();

        assert_eq!(round(mad.next(lit!(1.5))), lit!(0.0));
        assert_eq!(round(mad.next(lit!(4.0))), lit!(1.25));
        assert_eq!(round(mad.next(lit!(8.0))), lit!(2.333));
        assert_eq!(round(mad.next(lit!(4.0))), lit!(1.813));
        assert_eq!(round(mad.next(lit!(4.0))), lit!(1.48));
        assert_eq!(round(mad.next(lit!(1.5))), lit!(1.48));
    }

    #[test]
    fn test_reset() {
        let mut mad = MeanAbsoluteDeviation::new(5).unwrap();

        assert_eq!(round(mad.next(lit!(1.5))), lit!(0.0));
        assert_eq!(round(mad.next(lit!(4.0))), lit!(1.25));

        mad.reset();

        assert_eq!(round(mad.next(lit!(1.5))), lit!(0.0));
        assert_eq!(round(mad.next(lit!(4.0))), lit!(1.25));
    }

    #[test]
    fn test_default() {
        MeanAbsoluteDeviation::default();
    }

    #[test]
    fn test_display() {
        let indicator = MeanAbsoluteDeviation::new(10).unwrap();
        assert_eq!(format!("{}", indicator), "MAD(10)");
    }
}
