use std::fmt;

use crate::errors::*;
use crate::{Close, Next, Reset};

/// Moving Standard Deviation (MSD).
///
/// # Formula
///
/// ![MSD](https://wikimedia.org/api/rest_v1/media/math/render/svg/9f2b2061a0bda785c7d246d21ec20a17c79185e0)
///
/// Where:
///
/// * _MSD<sub>t</sub>_ - value of standard deviation at a point of time _t_
/// * _n_ - number of periods (length)
/// * _p<sub>t</sub>_ - input value at a point of time _t_
///
/// # Parameters
///
/// * _n_ - number of periods (integer greater than 0)
///
/// # Exampile
///
/// ```
/// use ta::indicators::MovingStandardDeviation;
/// use ta::Next;
///
/// let mut msd = MovingStandardDeviation::new(3).unwrap();
/// assert_eq!(msd.next(1.0), 1.4142135623730951);
/// assert_eq!(msd.next(2.0), 1.1726039399558574);
/// assert_eq!(msd.next(3.0), 0.816496580927726);
/// ```
///
/// # Links
///
/// * [Standard Deviation, Wikipedia](https://en.wikipedia.org/wiki/Standard_deviation)
///
#[derive(Debug, Clone)]
pub struct MovingStandardDeviation {
    n: u32,
    index: usize,
    count: u32,
    sum: f64,
    vec: Vec<f64>,
}

impl MovingStandardDeviation {
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

impl Next<f64> for MovingStandardDeviation {
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

impl<'a, T: Close> Next<&'a T> for MovingStandardDeviation {
    type Output = f64;

    fn next(&mut self, input: &'a T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for MovingStandardDeviation {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.sum = 0.0;
        for elem in self.vec.iter_mut() {
            *elem = 0_f64;
        }
    }
}

impl Default for MovingStandardDeviation {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl fmt::Display for MovingStandardDeviation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SD({})", self.n)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(MovingStandardDeviation);

    #[test]
    fn test_new() {
        assert!(MovingStandardDeviation::new(0).is_err());
        assert!(MovingStandardDeviation::new(1).is_ok());
    }
    
    #[test]
    fn test_next() {
        let mut msd = MovingStandardDeviation::new(4).unwrap();
        assert_eq!(msd.next(1.0), 1.7320508075688772);
        assert_eq!(msd.next(2.0), 1.5811388300841898);
        assert_eq!(msd.next(3.0), 1.4142135623730951);
        assert_eq!(msd.next(4.0), 1.118033988749895);
        assert_eq!(msd.next(10.0), 3.112474899497183);
        assert_eq!(msd.next(20.0), 6.7592529172978875);
    }

    #[test]
    fn test_next_with_bars() {
        fn bar(close: f64) -> Bar {
            Bar::new().close(close)
        }
        let mut msd = MovingStandardDeviation::new(5).unwrap();
        assert_eq!(msd.next(&bar(1.0)), 2.0);
        assert_eq!(msd.next(&bar(2.0)), 1.9039432764659772);
        assert_eq!(msd.next(&bar(3.0)), 1.8257418583505538);
        assert_eq!(msd.next(&bar(4.0)), 1.6770509831248424);
        assert_eq!(msd.next(&bar(100.0)), 39.01281840626232);
    }
    
    #[test]
    fn test_reset() {
        let mut msd = MovingStandardDeviation::new(3).unwrap();
        assert_eq!(msd.next(1.0), 1.4142135623730951);
        assert_eq!(msd.next(2.0), 1.1726039399558574);
        assert_eq!(msd.next(3.0), 0.816496580927726);

        msd.reset();
        // assert_eq!(msd.next(100), 45.963753835676506); // no reset
        assert_eq!(msd.next(100.0), 141.4213562373095);
    }

    #[test]
    fn test_default(){
        MovingStandardDeviation::default();
    }

    #[test]
    fn test_display() {
        let msd = MovingStandardDeviation::new(5).unwrap();
        assert_eq!(format!("{}", msd), "SD(5)");
    }

}

