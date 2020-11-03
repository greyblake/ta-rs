use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::errors::Result;
use crate::indicators::{AverageTrueRange, Maximum, Minimum};
use crate::{Close, High, Low, Next, Period, Reset};

/// Chandelier Exit (CE).
///
/// Developed by Charles Le Beau and featured in Alexander Elder's books, the Chandelier Exit sets
/// a trailing stop-loss based on the Average True Range (ATR). The indicator is designed to keep
/// traders in a trend and prevent an early exit as long as the trend extends. Typically, the
/// Chandelier Exit will be above prices during a downtrend and below prices during an uptrend.
///
/// # Formula
///
/// Chandelier Exit (long) = Max(_period_) - ATR(_period_) * _multipler_
/// Chandelier Exit (short) = Min(_period_) + ATR(_period_) * _multipler_
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0). Default is 22.
/// * _multipler_ - ATR factor. Default is 3.
///
/// # Example
///
/// ```
/// use ta::indicators::ChandelierExit;
/// use ta::{Next, DataItem};
///
/// let value1 = DataItem::builder()
/// .open(21.0).high(22.0).low(20.0).close(21.0).volume(1.0).build().unwrap();
/// let value2 = DataItem::builder()
/// .open(23.0).high(24.0).low(22.0).close(23.0).volume(1.0).build().unwrap();
///
/// let mut ce = ChandelierExit::default();
///
/// let first = ce.next(&value1);
/// assert_eq!(first.long, 16.0);
/// assert_eq!(first.short, 26.0);
///
/// let second = ce.next(&value2);
/// assert_eq!((second.long * 100.0).round() / 100.0, 17.74);
/// assert_eq!((second.short * 100.0).round() / 100.0, 26.26);
/// ```
///
/// # Links
///
/// * [Chandelier Exit, StockCharts](https://school.stockcharts.com/doku.php?id=technical_indicators:chandelier_exit)
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct ChandelierExit {
    atr: AverageTrueRange,
    min: Minimum,
    max: Maximum,
    multiplier: f64,
}

impl ChandelierExit {
    pub fn new(period: usize, multiplier: f64) -> Result<Self> {
        Ok(Self {
            atr: AverageTrueRange::new(period)?,
            min: Minimum::new(period)?,
            max: Maximum::new(period)?,
            multiplier,
        })
    }

    pub fn multiplier(&self) -> f64 {
        self.multiplier
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChandelierExitOutput {
    pub long: f64,
    pub short: f64,
}

impl From<ChandelierExitOutput> for (f64, f64) {
    fn from(ce: ChandelierExitOutput) -> Self {
        (ce.long, ce.short)
    }
}

impl Period for ChandelierExit {
    fn period(&self) -> usize {
        self.atr.period()
    }
}

impl<T: Low + High + Close> Next<&T> for ChandelierExit {
    type Output = ChandelierExitOutput;

    fn next(&mut self, input: &T) -> Self::Output {
        let atr = self.atr.next(input) * self.multiplier;
        let min = self.min.next(input);
        let max = self.max.next(input);

        ChandelierExitOutput {
            long: max - atr,
            short: min + atr,
        }
    }
}

impl Reset for ChandelierExit {
    fn reset(&mut self) {
        self.atr.reset();
        self.min.reset();
        self.max.reset();
    }
}

impl Default for ChandelierExit {
    fn default() -> Self {
        Self::new(22, 3.0).unwrap()
    }
}

impl fmt::Display for ChandelierExit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CE({}, {})", self.atr.period(), self.multiplier)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::*;

    use super::*;

    type Ce = ChandelierExit;

    fn round(nums: (f64, f64)) -> (f64, f64) {
        let n0 = (nums.0 * 100.0).round() / 100.0;
        let n1 = (nums.1 * 100.0).round() / 100.0;
        (n0, n1)
    }

    #[test]
    fn test_new() {
        assert!(Ce::new(0, 0.0).is_err());
        assert!(Ce::new(1, 1.0).is_ok());
        assert!(Ce::new(22, 3.0).is_ok());
    }

    #[test]
    fn test_next_bar() {
        let mut ce = Ce::new(5, 2.0).unwrap();

        let bar1 = Bar::new().high(2).low(1).close(1.5);
        assert_eq!(round(ce.next(&bar1).into()), (0.0, 3.0));

        let bar2 = Bar::new().high(5).low(3).close(4);
        assert_eq!(round(ce.next(&bar2).into()), (1.33, 4.67));

        let bar3 = Bar::new().high(9).low(7).close(8);
        assert_eq!(round(ce.next(&bar3).into()), (3.22, 6.78));

        let bar4 = Bar::new().high(5).low(3).close(4);
        assert_eq!(round(ce.next(&bar4).into()), (1.81, 8.19));

        let bar5 = Bar::new().high(5).low(3).close(4);
        assert_eq!(round(ce.next(&bar5).into()), (2.88, 7.12));

        let bar6 = Bar::new().high(2).low(1).close(1.5);
        assert_eq!(round(ce.next(&bar6).into()), (2.92, 7.08));
    }

    #[test]
    fn test_reset() {
        let mut ce = Ce::new(5, 2.0).unwrap();

        let bar1 = Bar::new().high(2).low(1).close(1.5);
        let bar2 = Bar::new().high(5).low(3).close(4);

        assert_eq!(round(ce.next(&bar1).into()), (0.0, 3.0));
        assert_eq!(round(ce.next(&bar2).into()), (1.33, 4.67));

        ce.reset();

        assert_eq!(round(ce.next(&bar1).into()), (0.0, 3.0));
        assert_eq!(round(ce.next(&bar2).into()), (1.33, 4.67));
    }

    #[test]
    fn test_default() {
        Ce::default();
    }

    #[test]
    fn test_display() {
        let indicator = Ce::new(10, 5.0).unwrap();
        assert_eq!(format!("{}", indicator), "CE(10, 5)");
    }
}
