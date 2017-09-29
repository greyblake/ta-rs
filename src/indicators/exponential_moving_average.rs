use {Close, Next, Reset};
use errors::*;

#[derive(Debug,Clone)]
pub struct ExponentialMovingAverage {
    k:  f64,
    current: f64,
    is_new: bool
}

impl ExponentialMovingAverage {
    pub fn new(n : u32) -> Result<Self> {
        match n {
            0 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            _ => {
                let k = 2f64 / (n as f64 + 1f64);
                let indicator = Self { k: k, current: 0f64, is_new: true };
                Ok(indicator)
            }
        }
    }

}

impl Next<f64> for ExponentialMovingAverage {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        if self.is_new {
            self.is_new = false;
            self.current = input;
        } else {
            self.current = self.k * input + (1.0 - self.k) * self.current;
        }
        self.current
    }
}

impl<T: Close> Next<T> for ExponentialMovingAverage {
    type Output = f64;

    fn next(&mut self, input: T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for ExponentialMovingAverage {
    fn reset(&mut self) {
        self.current = 0.0;
        self.is_new = true;
    }
}

impl Default for ExponentialMovingAverage {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_helper::*;

    #[test]
    fn test_new() {
        assert!(ExponentialMovingAverage::new(0).is_err());
        assert!(ExponentialMovingAverage::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut ema = ExponentialMovingAverage::new(3).unwrap();

        assert_eq!(ema.next(2.0), 2.0);
        assert_eq!(ema.next(5.0), 3.5);
        assert_eq!(ema.next(1.0), 2.25);
        assert_eq!(ema.next(6.25), 4.25);

        let mut ema = ExponentialMovingAverage::new(3).unwrap();
        let bar1 = Bar::new().close(2);
        let bar2 = Bar::new().close(5);
        assert_eq!(ema.next(bar1), 2.0);
        assert_eq!(ema.next(bar2), 3.5);
    }

    #[test]
    fn test_reset() {
        let mut ema = ExponentialMovingAverage::new(5).unwrap();

        assert_eq!(ema.next(4.0), 4.0);
        ema.next(10.0);
        ema.next(15.0);
        ema.next(20.0);
        assert_ne!(ema.next(4.0), 4.0);

        ema.reset();
        assert_eq!(ema.next(4.0), 4.0);
    }

    #[test]
    fn test_default() {
        ExponentialMovingAverage::default();
    }
}
