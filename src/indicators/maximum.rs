use std::fmt;

use crate::errors::{Result, TaError};
use crate::{High, Next, NumberType, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Returns the highest value in a given time frame.
///
/// # Parameters
///
/// * _period_ - size of the time frame (integer greater than 0). Default value is 14.
///
/// # Example
///
/// ```
/// use ta::indicators::Maximum;
/// use ta::Next;
///
/// let mut max = Maximum::new(3).unwrap();
/// assert_eq!(max.next(7.0), 7.0);
/// assert_eq!(max.next(5.0), 7.0);
/// assert_eq!(max.next(4.0), 7.0);
/// assert_eq!(max.next(4.0), 5.0);
/// assert_eq!(max.next(8.0), 8.0);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Maximum {
    period: usize,
    max_index: usize,
    cur_index: usize,
    deque: Box<[NumberType]>,
}

impl Maximum {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                period,
                max_index: 0,
                cur_index: 0,
                deque: vec![f64::NEG_INFINITY; period].into_boxed_slice(),
            }),
        }
    }

    fn find_max_index(&self) -> usize {
        let mut max = f64::NEG_INFINITY;
        let mut index: usize = 0;

        for (i, &val) in self.deque.iter().enumerate() {
            if val > max {
                max = val;
                index = i;
            }
        }

        index
    }
}

impl Period for Maximum {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<NumberType> for Maximum {
    type Output = NumberType;

    fn next(&mut self, input: NumberType) -> Self::Output {
        self.deque[self.cur_index] = input;

        if input > self.deque[self.max_index] {
            self.max_index = self.cur_index;
        } else if self.max_index == self.cur_index {
            self.max_index = self.find_max_index();
        }

        self.cur_index = if self.cur_index + 1 < self.period {
            self.cur_index + 1
        } else {
            0
        };

        self.deque[self.max_index]
    }
}

impl<T: High> Next<&T> for Maximum {
    type Output = NumberType;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.high())
    }
}

impl Reset for Maximum {
    fn reset(&mut self) {
        for i in 0..self.period {
            self.deque[i] = f64::NEG_INFINITY;
        }
    }
}

impl Default for Maximum {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for Maximum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MAX({})", self.period)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lit;
    use crate::test_helper::*;

    test_indicator!(Maximum);

    #[test]
    fn test_new() {
        assert!(Maximum::new(0).is_err());
        assert!(Maximum::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut max = Maximum::new(3).unwrap();

        assert_eq!(max.next(lit!(4.0)), lit!(4.0));
        assert_eq!(max.next(lit!(1.2)), lit!(4.0));
        assert_eq!(max.next(lit!(5.0)), lit!(5.0));
        assert_eq!(max.next(lit!(3.0)), lit!(5.0));
        assert_eq!(max.next(lit!(4.0)), lit!(5.0));
        assert_eq!(max.next(lit!(0.0)), lit!(4.0));
        assert_eq!(max.next(lit!(-1.0)), lit!(4.0));
        assert_eq!(max.next(lit!(-2.0)), lit!(0.0));
        assert_eq!(max.next(lit!(-1.5)), lit!(-1.0));
    }

    #[test]
    fn test_next_with_bars() {
        fn bar(high: NumberType) -> Bar {
            Bar::new().high(high)
        }

        let mut max = Maximum::new(2).unwrap();

        assert_eq!(max.next(&bar(lit!(1.1))), lit!(1.1));
        assert_eq!(max.next(&bar(lit!(4.0))), lit!(4.0));
        assert_eq!(max.next(&bar(lit!(3.5))), lit!(4.0));
        assert_eq!(max.next(&bar(lit!(2.0))), lit!(3.5));
    }

    #[test]
    fn test_reset() {
        let mut max = Maximum::new(100).unwrap();
        assert_eq!(max.next(lit!(4.0)), lit!(4.0));
        assert_eq!(max.next(lit!(10.0)), lit!(10.0));
        assert_eq!(max.next(lit!(4.0)), lit!(10.0));

        max.reset();
        assert_eq!(max.next(lit!(4.0)), lit!(4.0));
    }

    #[test]
    fn test_default() {
        Maximum::default();
    }

    #[test]
    fn test_display() {
        let indicator = Maximum::new(7).unwrap();
        assert_eq!(format!("{}", indicator), "MAX(7)");
    }
}
