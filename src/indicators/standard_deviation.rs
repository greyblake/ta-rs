use std::fmt;

use crate::errors::*;
use crate::{Close, Next, Reset};

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
/// * _n_ - number of periods (integer greater than 0)
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
#[derive(Debug, Clone)]
pub struct StandardDeviation {
    n: u32,
    index: usize,
    count: u32,
    m: f64,
    m2: f64,
    vec: Vec<f64>,
}

impl StandardDeviation {
    pub fn new(n: u32) -> Result<Self> {
        match n {
            0 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            _ => {
                let std = StandardDeviation {
                    n,
                    index: 0,
                    count: 0,
                    m: 0.0,
                    m2: 0.0,
                    vec: vec![0.0; n as usize],
                };
                Ok(std)
            }
        }
    }

    pub(super) fn mean(&self) -> f64 {
        self.m
    }
}

impl Next<f64> for StandardDeviation {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.index = (self.index + 1) % (self.n as usize);

        let old_val = self.vec[self.index];
        self.vec[self.index] = input;

        if self.count < self.n {
            self.count += 1;
            let delta = input - self.m;
            self.m += delta / self.count as f64;
            let delta2 = input - self.m;
            self.m2 += delta * delta2;
        } else {
            let delta = input - old_val;
            let old_m = self.m;
            self.m += delta / self.n as f64;
            let delta2 = input - self.m + old_val - old_m;
            self.m2 += delta * delta2;
        }

        (self.m2 / self.count as f64).sqrt()
    }
}

impl<'a, T: Close> Next<&'a T> for StandardDeviation {
    type Output = f64;

    fn next(&mut self, input: &'a T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for StandardDeviation {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.m = 0.0;
        self.m2 = 0.0;
        for i in 0..(self.n as usize) {
            self.vec[i] = 0.0;
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
        write!(f, "SD({})", self.n)
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
        assert_eq!(sd.next(10.0), 0.0);
        assert_eq!(sd.next(20.0), 5.0);
        assert_eq!(round(sd.next(30.0)), 8.165);
        assert_eq!(round(sd.next(20.0)), 7.071);
        assert_eq!(round(sd.next(10.0)), 7.071);
        assert_eq!(round(sd.next(100.0)), 35.355);
    }

    #[test]
    fn test_next_with_bars() {
        fn bar(close: f64) -> Bar {
            Bar::new().close(close)
        }

        let mut sd = StandardDeviation::new(4).unwrap();
        assert_eq!(sd.next(&bar(10.0)), 0.0);
        assert_eq!(sd.next(&bar(20.0)), 5.0);
        assert_eq!(round(sd.next(&bar(30.0))), 8.165);
        assert_eq!(round(sd.next(&bar(20.0))), 7.071);
        assert_eq!(round(sd.next(&bar(10.0))), 7.071);
        assert_eq!(round(sd.next(&bar(100.0))), 35.355);
    }

    #[test]
    fn test_next_same_values() {
        let mut sd = StandardDeviation::new(3).unwrap();
        assert_eq!(sd.next(4.2), 0.0);
        assert_eq!(sd.next(4.2), 0.0);
        assert_eq!(sd.next(4.2), 0.0);
        assert_eq!(sd.next(4.2), 0.0);
    }

    #[test]
    fn test_reset() {
        let mut sd = StandardDeviation::new(4).unwrap();
        assert_eq!(sd.next(10.0), 0.0);
        assert_eq!(sd.next(20.0), 5.0);
        assert_eq!(round(sd.next(30.0)), 8.165);

        sd.reset();
        assert_eq!(sd.next(20.0), 0.0);
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
