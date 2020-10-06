use std::f64::INFINITY;
use std::fmt;

use crate::errors::{Error, ErrorKind, Result};
use crate::{High, Next, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Returns the highest value in a given time frame.
///
/// # Parameters
///
/// * _n_ - size of the time frame (integer greater than 0). Default value is 14.
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
    n: usize,
    vec: Vec<f64>,
    max_index: usize,
    cur_index: usize,
}

impl Maximum {
    pub fn new(n: u32) -> Result<Self> {
        let n = n as usize;

        if n == 0 {
            return Err(Error::from_kind(ErrorKind::InvalidParameter));
        }

        let indicator = Self {
            n,
            vec: vec![-INFINITY; n],
            max_index: 0,
            cur_index: 0,
        };
        Ok(indicator)
    }

    fn find_max_index(&self) -> usize {
        let mut max = -INFINITY;
        let mut index: usize = 0;

        for (i, &val) in self.vec.iter().enumerate() {
            if val > max {
                max = val;
                index = i;
            }
        }

        index
    }
}

impl Next<f64> for Maximum {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.vec[self.cur_index] = input;

        if input > self.vec[self.max_index] {
            self.max_index = self.cur_index;
        } else if self.max_index == self.cur_index {
            self.max_index = self.find_max_index();
        }

        self.cur_index = if self.cur_index + 1 < self.n as usize {
            self.cur_index + 1
        } else {
            0
        };

        self.vec[self.max_index]
    }
}

impl<T: High> Next<&T> for Maximum {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.high())
    }
}

impl Reset for Maximum {
    fn reset(&mut self) {
        for i in 0..self.n {
            self.vec[i] = -INFINITY;
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
        write!(f, "MAX({})", self.n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

        assert_eq!(max.next(4.0), 4.0);
        assert_eq!(max.next(1.2), 4.0);
        assert_eq!(max.next(5.0), 5.0);
        assert_eq!(max.next(3.0), 5.0);
        assert_eq!(max.next(4.0), 5.0);
        assert_eq!(max.next(0.0), 4.0);
        assert_eq!(max.next(-1.0), 4.0);
        assert_eq!(max.next(-2.0), 0.0);
        assert_eq!(max.next(-1.5), -1.0);
    }

    #[test]
    fn test_next_with_bars() {
        fn bar(high: f64) -> Bar {
            Bar::new().high(high)
        }

        let mut max = Maximum::new(2).unwrap();

        assert_eq!(max.next(&bar(1.1)), 1.1);
        assert_eq!(max.next(&bar(4.0)), 4.0);
        assert_eq!(max.next(&bar(3.5)), 4.0);
        assert_eq!(max.next(&bar(2.0)), 3.5);
    }

    #[test]
    fn test_reset() {
        let mut max = Maximum::new(100).unwrap();
        assert_eq!(max.next(4.0), 4.0);
        assert_eq!(max.next(10.0), 10.0);
        assert_eq!(max.next(4.0), 10.0);

        max.reset();
        assert_eq!(max.next(4.0), 4.0);
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
