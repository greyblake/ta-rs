//use std::fmt;
//use {Next, Reset, High, Low, Close};
use indicators::ExponentialMovingAverage as Ema;
use errors::*;


#[derive(Debug,Clone)]
pub struct MovingAverageConvergenceDivergence {
    fast_ema: Ema,
    slow_ema: Ema,
    signal_ema: Ema
}

impl MovingAverageConvergenceDivergence {
    pub fn new(fast_length: u32, slow_length: u32, signal_length: u32) -> Result<Self> {
        let indicator = Self {
            fast_ema: Ema::new(fast_length)?,
            slow_ema: Ema::new(slow_length)?,
            signal_ema: Ema::new(signal_length)?
        };
        Ok(indicator)
    }
}

#[cfg(test)]
mod tests {
    use super::MovingAverageConvergenceDivergence as Macd;
    //use test_helper::*;

    #[test]
    fn test_new() {
        assert!(Macd::new(0, 1, 1).is_err());
        assert!(Macd::new(1, 0, 1).is_err());
        assert!(Macd::new(1, 1, 0).is_err());
        assert!(Macd::new(1, 1, 1).is_ok());
    }
}
