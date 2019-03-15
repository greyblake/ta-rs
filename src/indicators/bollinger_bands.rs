use std::fmt;

use crate::errors::*;
use crate::{Close, Next, Reset};


/// Bollinger Bands (BB).
///
/// # Formula
///
/// Where:
///
/// * _bb<sub>t</sub>_ - value of bollinger bands(bbt, bbm, bbb) at time _t_ based upon SMA after n periods
/// * _n_ - number of periods (length)
/// * _p<sub>t</sub>_ - input value at a point of time _t_
///
/// # Parameters
///
/// * _n_ - number of periods (integer greater than 0)
/// * _n_sd_- - number of standard deviations
///
/// # Example
///
/// ```
/// use ta::indicators::BollingerBands;
/// use ta::Next;
///
/// let mut bb = BollingerBands::new(5,2).unwrap();
///
/// assert_eq!(bb.next(10.0), (0.0,0.0,0.0));
/// assert_eq!(bb.next(11.0), (0.0,0.0,0.0));
/// assert_eq!(bb.next(12.0), (0.0,0.0,0.0));
/// assert_eq!(bb.next(13.0), (0.0,0.0,0.0));
/// assert_eq!(round(bb.next(14.0)), (14.83,12.0,9.17));
///
/// fn round(nums: (f64, f64, f64)) -> (f64, f64, f64) {
///     let n0 = (nums.0 * 100.0).round() / 100.0;
///     let n1 = (nums.1 * 100.0).round() / 100.0;
///     let n2 = (nums.2 * 100.0).round() / 100.0;
///     (n0, n1, n2)
/// }
/// ```
///
/// # Links
///
/// * [Bollinger Bands, Wikipedia](https://en.wikipedia.org/wiki/Bollinger_Bands)
///

#[derive(Debug, Clone)]
pub struct  BollingerBands {
    n: u32,
    n_sd: u32,
    vec: Vec<f64>,
    index: usize,
    count: u32,
    mean: f64,
    is_new: bool,
}


impl BollingerBands {

    pub fn new(n: u32, n_sd: u32) -> Result<Self> {
        match (n,n_sd) {
            (0,_) | (_,0) => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            (_,_) =>  {

                let indicator = Self {
                    n: n,
                    n_sd: n_sd,
                    vec: vec![0.0; n as usize],
                    index: 0,
                    count: 1,
                    mean: 0.0,
                    is_new: true,
                };
                Ok(indicator)
            }

        }

    }

}


impl Next<f64> for  BollingerBands {
    type Output = (f64, f64, f64);

    fn next(&mut self, input: f64) -> Self::Output {
        // Updating var and sum

        let old_price = self.vec[self.index];
        self.vec[self.index] = input;
        self.mean = ((self.mean * self.n as f64) + input - old_price)/ self.n as f64;
        self.index = (self.index + 1) % (self.n as usize);

        if !self.is_new {
            let mut var: f64 = 0.0;
            for i in &self.vec{
                var = var + (*i - self.mean) * (*i - self.mean);
            }
            let sd: f64 = (var/self.n as f64).sqrt();

            ((self.mean +(sd * self.n_sd as f64)),self.mean, (self.mean - (sd * self.n_sd as f64)))
        }else{
            if self.count == (self.n-1){
                self.is_new = false;
            }
            self.count = self.count + 1;
            (0.0,0.0,0.0)
        }
    }
}


impl<'a, T:Close> Next<&'a T> for  BollingerBands {
    type Output = (f64, f64, f64);

    fn next(&mut self, input: &'a T) -> Self::Output {
        self.next(input.close())
    }
}


impl Reset for BollingerBands {
    fn reset(&mut self) {
        for i in 0..(self.n as usize) {
            self.vec[i] = 0.0;
        }
        self.index = 0;
        self.count = 1;
        self.mean = 0.0;
        self.is_new = true;
    }
}

impl Default for  BollingerBands {
    fn default() -> Self {
        Self::new(20,2).unwrap()
    }
}

impl fmt::Display for BollingerBands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BB({},{})", self.n, self.n_sd)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(BollingerBands);

    fn round(nums: (f64, f64, f64)) -> (f64, f64, f64) {
        let n0 = (nums.0 * 100.0).round() / 100.0;
        let n1 = (nums.1 * 100.0).round() / 100.0;
        let n2 = (nums.2 * 100.0).round() / 100.0;
        (n0, n1, n2)
    }

    #[test]
    fn test_new() {
        assert!(BollingerBands::new(0,3).is_err());
        assert!(BollingerBands::new(3,0).is_err());
        assert!(BollingerBands::new(0,0).is_err());
        assert!(BollingerBands::new(1,2).is_ok());
    }

    #[test]
    fn test_next_with_f64() {
        let mut bb = BollingerBands::new(3,1).unwrap();
        assert_eq!(bb.next(35.0),(0.0, 0.0 ,0.0));
        assert_eq!(bb.next(67.0), (0.0, 0.0, 0.0));
        assert_eq!(round(bb.next(98.0)), (92.39, 66.67, 40.95));

    }
    #[test]
    fn test_next_with_bars() {
        fn bar(close: f64) -> Bar {
            Bar::new().close(close)
        }

        let mut bb = BollingerBands::new(3,1).unwrap();
        assert_eq!(bb.next(200.0), (0.0, 0.0, 0.0));

    }

    #[test]
    fn test_reset() {
        let mut bb = BollingerBands::new(5,2).unwrap();

        assert_eq!(bb.next(10.0), (0.0,0.0,0.0));
        assert_eq!(bb.next(11.0), (0.0,0.0,0.0));
        assert_eq!(bb.next(12.0), (0.0,0.0,0.0));
        assert_eq!(bb.next(13.0), (0.0,0.0,0.0));
        assert_eq!(round(bb.next(14.00)), (14.83,12.0,9.17));

        bb.reset();
        assert_eq!(bb.next(200.0), (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_default() {
        BollingerBands::default();
    }

    #[test]
    fn test_display() {
        let bb = BollingerBands::new(4,2).unwrap();
        assert_eq!(format!("{}", bb), "BB(4,2)");
    }
}
