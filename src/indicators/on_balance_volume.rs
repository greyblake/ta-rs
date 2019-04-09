use std::fmt;

use crate::{Close, Next, Reset, Volume};

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

#[derive(Debug, Clone)]
pub struct OnBalanceVolume {
    obv: f64,
    prev_close: f64,
}

impl OnBalanceVolume {
    pub fn new() -> Self {
        Self {
            obv: 0.0,
            prev_close: 0.0,
        }
    }
}

impl<'a, T: Close + Volume> Next<&'a T> for OnBalanceVolume {
    type Output = f64;

    fn next(&mut self, input: &'a T) -> f64 {
        if input.close() > self.prev_close {
            self.obv = self.obv + input.volume();
        } else if input.close() < self.prev_close {
            self.obv = self.obv - input.volume();
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
        self.obv = 0.0;
        self.prev_close = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    #[test]
    fn test_next_bar() {
        let mut obv = OnBalanceVolume::new();

        let bar1 = Bar::new().close(1.5).volume(1000.0);
        let bar2 = Bar::new().close(5).volume(5000.0);
        let bar3 = Bar::new().close(4).volume(9000.0);
        let bar4 = Bar::new().close(4).volume(4000.0);

        assert_eq!(obv.next(&bar1), 1000.0);

        //close > prev_close
        assert_eq!(obv.next(&bar2), 6000.0);

        // close < prev_close
        assert_eq!(obv.next(&bar3), -3000.0);

        // close == prev_close
        assert_eq!(obv.next(&bar4), -3000.0);
    }

    #[test]
    fn test_reset() {
        let mut obv = OnBalanceVolume::new();

        let bar1 = Bar::new().close(1.5).volume(1000.0);
        let bar2 = Bar::new().close(4).volume(2000.0);
        let bar3 = Bar::new().close(8).volume(3000.0);

        assert_eq!(obv.next(&bar1), 1000.0);
        assert_eq!(obv.next(&bar2), 3000.0);
        assert_eq!(obv.next(&bar3), 6000.0);

        obv.reset();

        assert_eq!(obv.next(&bar1), 1000.0);
        assert_eq!(obv.next(&bar2), 3000.0);
        assert_eq!(obv.next(&bar3), 6000.0);
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
