use std::fmt;

use crate::errors::*;
use crate::{Close, Next, Reset};

/// Standard Deviation (SD).
///
/// # Formula
///
/// ![SD](https://wikimedia.org/api/rest_v1/media/math/render/svg/9f2b2061a0bda785c7d246d21ec20a17c79185e0)
///
/// Where:
///
/// * _SD<sub>t</sub>_ - value of standard deviation at a point of time _t_
/// * _n_ - number of periods (length)
/// * _p<sub>t</sub>_ - input value at a point of time _t_
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
/// let mut s_d = StandardDeviation::new(3).unwrap();
/// assert_eq!(s_d.next(1.0), 1.4142135623730951);
/// assert_eq!(s_d.next(2.0), 1.1726039399558574);
/// assert_eq!(s_d.next(3.0), 0.816496580927726);
/// ```
///
/// # Links
///
/// * [StandardDeviation, Wikipedia](https://en.wikipedia.org/wiki/Standard_deviation)
///
#[derive(Debug, Clone)]
pub struct StandardDeviation {
    n: u32,
    index: usize,
    count: u32,
    sum: f64,
    vec: Vec<f64>,
}

impl StandardDeviation {
    pub fn new(n: u32) -> Result<Self> {
        match n {
            0 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            _ => {
                let indicator = Self {
                    n: n,
                    index: 0,
                    count: 0,
                    sum: 0.0,
                    vec: vec![0.0; n as usize],
                };
                Ok(indicator)
            },
        }
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
        }
        self.sum = self.sum - old_val + input;
        let mean = self.sum / (self.count as f64);
        let mut mean_item_sum_pow = 0_f64;
        for item in self.vec.iter() {
            mean_item_sum_pow += (item - mean).powi(2);
        }
        (mean_item_sum_pow / (self.count as f64)).sqrt()
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
        self.sum = 0.0;
        for elem in self.vec.iter_mut() {
            *elem = 0_f64;
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
mod test {
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
        let mut s_d = StandardDeviation::new(4).unwrap();
        assert_eq!(s_d.next(1.0), 1.7320508075688772);
        assert_eq!(s_d.next(2.0), 1.5811388300841898);
        assert_eq!(s_d.next(3.0), 1.4142135623730951);
        assert_eq!(s_d.next(4.0), 1.118033988749895);
        assert_eq!(s_d.next(10.0), 3.112474899497183);
        assert_eq!(s_d.next(20.0), 6.7592529172978875);
    }

    #[test]
    fn test_next_with_bars() {
        fn bar(close: f64) -> Bar {
            Bar::new().close(close)
        }
        let mut s_d = StandardDeviation::new(5).unwrap();
        assert_eq!(s_d.next(&bar(1.0)), 2.0);
        assert_eq!(s_d.next(&bar(2.0)), 1.9039432764659772);
        assert_eq!(s_d.next(&bar(3.0)), 1.8257418583505538);
        assert_eq!(s_d.next(&bar(4.0)), 1.6770509831248424);
        assert_eq!(s_d.next(&bar(100.0)), 39.01281840626232);
    }
    
    #[test]
    fn test_reset() {
        let mut s_d = StandardDeviation::new(3).unwrap();
        assert_eq!(s_d.next(1.0), 1.4142135623730951);
        assert_eq!(s_d.next(2.0), 1.1726039399558574);
        assert_eq!(s_d.next(3.0), 0.816496580927726);

        s_d.reset();
        // assert_eq!(s_d.next(100), 45.963753835676506); // no reset
        assert_eq!(s_d.next(100.0), 141.4213562373095);
    }

    #[test]
    fn test_default(){
        StandardDeviation::default();
    }

    #[test]
    fn test_display() {
        let s_d = StandardDeviation::new(5).unwrap();
        assert_eq!(format!("{}", s_d), "SD(5)");
    }

}

