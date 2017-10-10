use indicators::{TrueRange, ExponentialMovingAverage};
use errors::*;
use {Next, High, Low, Close};

pub struct AverageTrueRange {
    true_range: TrueRange,
    ema: ExponentialMovingAverage
}

impl AverageTrueRange {
    pub fn new(length: u32) -> Result<Self> {
        let indicator = Self {
            true_range: TrueRange::new(),
            ema: ExponentialMovingAverage::new(length)?
        };
        Ok(indicator)
    }
}

impl<'a, T: High + Low + Close>  Next<&'a T> for AverageTrueRange {
    type Output = f64;

    fn next(&mut self, input: &'a T) -> Self::Output {
        self.ema.next(
            self.true_range.next(input)
        )
    }
}

impl Reset for AverageTrueRange {
    fn
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_helper::*;

    #[test]
    fn test_new() {
        assert!(AverageTrueRange::new(0).is_err());
        assert!(AverageTrueRange::new(1).is_ok());
    }
    #[test]
    fn test_next() {
        let mut atr = AverageTrueRange::new(3).unwrap();

        let bar1 = Bar::new().high(10).low(7.5).close(9);
        let bar2 = Bar::new().high(11).low(9).close(9.5);
        let bar3 = Bar::new().high(9).low(5).close(8);

        assert_eq!(atr.next(&bar1), 2.5);
        assert_eq!(atr.next(&bar2), 2.25);
        assert_eq!(atr.next(&bar3), 3.375);
    }

    #[test]
    fn test_reset() {
        let mut atr = AverageTrueRange::new(9).unwrap();

        let bar1 = Bar::new().high(10).low(7.5).close(9);
        let bar2 = Bar::new().high(11).low(9).close(9.5);

        atr.next(&bar1);
        atr.next(&bar2);

        atr.reset();
        let bar3 = Bar::new().high(60).low(15).close(51);
        assert_eq!(atr.next(&bar3), 45.0);
    }
}
