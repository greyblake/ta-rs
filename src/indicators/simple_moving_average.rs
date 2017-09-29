use {Close, Next, Reset};
use errors::*;

/// Simple moving average (SMA).
///
/// # Formula
///
/// ![SMA](https://wikimedia.org/api/rest_v1/media/math/render/svg/e2bf09dc6deaf86b3607040585fac6078f9c7c89)
///
/// Where:
///
/// * _SMA<sub>t</sub>_ - value of simple moving average at a point of time _t_
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
/// use ta::indicators::SimpleMovingAverage;
/// use ta::Next;
///
/// let mut sma = SimpleMovingAverage::new(3).unwrap();
/// assert_eq!(sma.next(10), 10.0);
/// assert_eq!(sma.next(11), 10.5);
/// assert_eq!(sma.next(12), 11.0);
/// assert_eq!(sma.next(13), 12.0);
/// ```
///
/// # Links
///
/// * [Simple Moving Average, Wikipedia](https://en.wikipedia.org/wiki/Moving_average#Simple_moving_average)
///
pub struct SimpleMovingAverage {
    n: u32,
    index: usize,
    count: u32,
    sum: f64,
    vec: Vec<f64>
}

impl SimpleMovingAverage {
    pub fn new(n: u32) -> Result<Self> {
        match n {
            0 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            _ => {
                let indicator = Self {
                    n: n,
                    index: 0,
                    count: 0,
                    sum: 0.0,
                    vec: vec![0.0; n as usize]
                };
                Ok(indicator)
            }
        }
    }
}

impl Next<f64> for SimpleMovingAverage {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.index = (self.index + 1) % (self.n as usize);

        let old_val = self.vec[self.index];
        self.vec[self.index] = input;

        if self.count < self.n {
            self.count += 1;
        }

        self.sum = self.sum - old_val + input;
        self.sum / (self.count as f64)
    }
}

impl Next<i32> for SimpleMovingAverage {
    type Output = f64;

    fn next(&mut self, input: i32) -> Self::Output {
        let input: f64 = input.into();
        self.next(input)
    }
}

impl Reset for SimpleMovingAverage {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.sum = 0.0;
        for i in 0..(self.n as usize) {
            self.vec[i] = 0.0;
        }
    }
}

impl Default for SimpleMovingAverage {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_helper::*;

    #[test]
    fn test_next() {
        let mut sma = SimpleMovingAverage::new(4).unwrap();
        assert_eq!(sma.next(4), 4.0);
        assert_eq!(sma.next(5), 4.5);
        assert_eq!(sma.next(6), 5.0);
        assert_eq!(sma.next(6), 5.25);
        assert_eq!(sma.next(6), 5.75);
        assert_eq!(sma.next(6), 6.0);
        assert_eq!(sma.next(2), 5.0);
    }

    #[test]
    fn test_reset() {
        let mut sma = SimpleMovingAverage::new(4).unwrap();
        assert_eq!(sma.next(4), 4.0);
        assert_eq!(sma.next(5), 4.5);
        assert_eq!(sma.next(6), 5.0);

        sma.reset();
        assert_eq!(sma.next(99), 99.0);
    }
}
