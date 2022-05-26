use std::fmt;

use crate::{lit, Close, Next, NumberType, Reset, Volume};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// On Balance Volume (OBV).
///
/// The OBV is an volume and price based oscillator which gives cumulative total volumes.
/// OBV measures buying and selling pressure as a cumulative indicator,
/// adding volume on up days and subtracting it on down days.
///
/// # Formula
///
/// If the closing price is above the prior close price then:
/// Current OBV = Previous OBV + Current Volume
///
/// If the closing price is below the prior close price then:
/// Current OBV = Previous OBV  -  Current Volume
///
/// If the closing prices equals the prior close price then:
/// Current OBV = Previous OBV
///
/// Where:
///
/// obv - on the balance volume
///
/// # Example
///
/// ```
/// use ta::indicators::OnBalanceVolume;
/// use ta::{Next, DataItem};
///
/// let mut obv = OnBalanceVolume::new();
///
/// let di1 = DataItem::builder()
///             .high(3.0)
///             .low(1.0)
///             .close(2.0)
///             .open(1.5)
///             .volume(1000.0)
///             .build().unwrap();
///
/// let di2 = DataItem::builder()
///             .high(3.0)
///             .low(1.0)
///             .close(1.5)
///             .open(1.5)
///             .volume(300.0)
///             .build().unwrap();
///
/// assert_eq!(obv.next(&di1), 1000.0);
/// assert_eq!(obv.next(&di2), 700.0);
/// ```
///
/// # Links
///
/// * [On Balance Volume, Wikipedia](https://en.wikipedia.org/wiki/On-balance_volume)
/// * [On Balance Volume, stockcharts](https://stockcharts.com/school/doku.php?id=chart_school:technical_indicators:on_balance_volume_obv)

#[doc(alias = "OBV")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct OnBalanceVolume {
    obv: NumberType,
    prev_close: NumberType,
}

impl OnBalanceVolume {
    pub fn new() -> Self {
        Self {
            obv: lit!(0.0),
            prev_close: lit!(0.0),
        }
    }
}

impl<T: Close + Volume> Next<&T> for OnBalanceVolume {
    type Output = NumberType;

    fn next(&mut self, input: &T) -> NumberType {
        if input.close() > self.prev_close {
            self.obv += input.volume();
        } else if input.close() < self.prev_close {
            self.obv -= input.volume();
        }
        self.prev_close = input.close();
        self.obv
    }
}

impl Default for OnBalanceVolume {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for OnBalanceVolume {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OBV")
    }
}

impl Reset for OnBalanceVolume {
    fn reset(&mut self) {
        self.obv = lit!(0.0);
        self.prev_close = lit!(0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    #[test]
    fn test_next_bar() {
        let mut obv = OnBalanceVolume::new();

        let bar1 = Bar::new().close(lit!(1.5)).volume(1000);
        let bar2 = Bar::new().close(5).volume(5000);
        let bar3 = Bar::new().close(4).volume(9000);
        let bar4 = Bar::new().close(4).volume(4000);

        assert_eq!(obv.next(&bar1), lit!(1000.0));

        //close > prev_close
        assert_eq!(obv.next(&bar2), lit!(6000.0));

        // close < prev_close
        assert_eq!(obv.next(&bar3), lit!(-3000.0));

        // close == prev_close
        assert_eq!(obv.next(&bar4), lit!(-3000.0));
    }

    #[test]
    fn test_reset() {
        let mut obv = OnBalanceVolume::new();

        let bar1 = Bar::new().close(lit!(1.5)).volume(1000);
        let bar2 = Bar::new().close(4).volume(2000);
        let bar3 = Bar::new().close(8).volume(3000);

        assert_eq!(obv.next(&bar1), lit!(1000.0));
        assert_eq!(obv.next(&bar2), lit!(3000.0));
        assert_eq!(obv.next(&bar3), lit!(6000.0));

        obv.reset();

        assert_eq!(obv.next(&bar1), lit!(1000.0));
        assert_eq!(obv.next(&bar2), lit!(3000.0));
        assert_eq!(obv.next(&bar3), lit!(6000.0));
    }

    #[test]
    fn test_default() {
        OnBalanceVolume::default();
    }

    #[test]
    fn test_display() {
        let obv = OnBalanceVolume::new();
        assert_eq!(format!("{}", obv), "OBV");
    }
}
