use std::collections::VecDeque;
use std::fmt;

use crate::errors::*;
use crate::{Close, High, Low, Next, Reset, Volume};

/// Money Flow Index (MFI).
///
/// The MFI is an volume and price based oscillator which gives moneyflow over n periods.
/// MFI is used to measure buying and selling pressure.
/// MFI is also known as volume-weighted RSI.
///
/// # Formula
///
/// Typical Price(TP) = (High + Low + Close)/3
///
/// Money Flow(MF) = Typical Price x Volume
///
/// MF is positive when currennt TP is greater that previous period TP and
/// negative when current TP is less than preivous TP.
///
/// Positive money flow (PMF)- calculated by adding the money flow of all the days RMF is positive.
///
/// Negative money flow (NMF)- calculated by adding the money flow of all the days RMF is negative.
///
/// Money Flow Index(MFI) = PMF / (PMF + NMF) * 100
///
///
/// # Parameters
///
/// * _n_ - number of periods, integer greater than 0
///
/// # Example
///
/// ```
/// use ta::indicators::MoneyFlowIndex;
/// use ta::{Next, DataItem};
///
/// let mut mfi = MoneyFlowIndex::new(3).unwrap();
/// let di = DataItem::builder()
///             .high(3.0)
///             .low(1.0)
///             .close(2.0)
///             .open(1.5)
///             .volume(1000.0)
///             .build().unwrap();
/// mfi.next(&di);
///
/// ```
/// # Links
/// * [Money Flow Index, Wikipedia](https://en.wikipedia.org/wiki/Money_flow_index)
/// * [Money Flow Index, stockcharts](https://stockcharts.com/school/doku.php?id=chart_school:technical_indicators:money_flow_index_mfi)

#[derive(Debug, Clone)]
pub struct MoneyFlowIndex {
    n: u32,
    money_flows: VecDeque<f64>,
    prev_typical_price: f64,
    total_positive_money_flow: f64,
    total_absolute_money_flow: f64,
    is_new: bool,
}

impl MoneyFlowIndex {
    pub fn new(n: u32) -> Result<Self> {
        match n {
            0 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            _ => {
                let indicator = Self {
                    n: n,
                    money_flows: VecDeque::with_capacity(n as usize + 1),
                    prev_typical_price: 0.0,
                    total_positive_money_flow: 0.0,
                    total_absolute_money_flow: 0.0,
                    is_new: true,
                };
                Ok(indicator)
            }
        }
    }
}

impl<'a, T: High + Low + Close + Volume> Next<&'a T> for MoneyFlowIndex {
    type Output = f64;

    fn next(&mut self, input: &'a T) -> f64 {
        let typical_price = (input.high() + input.low() + input.close()) / 3.0;

        if self.is_new {
            // money flow is 0, because without having previous typical_price
            // it is not possible to determine is it positive or negative.
            self.money_flows.push_back(0.0);
            self.prev_typical_price = typical_price;
            self.is_new = false;
            return 50.0;
        } else {
            let money_flow = typical_price * input.volume();

            let signed_money_flow = if typical_price >= self.prev_typical_price {
                self.total_positive_money_flow += money_flow;
                money_flow
            } else {
                -money_flow
            };

            self.total_absolute_money_flow += money_flow;

            if self.money_flows.len() == (self.n as usize) {
                let old_signed_money_flow = self.money_flows.pop_front().unwrap();
                if old_signed_money_flow > 0.0 {
                    self.total_positive_money_flow -= old_signed_money_flow;
                    self.total_absolute_money_flow -= old_signed_money_flow;
                } else {
                    // it is actually subtraction, because old_signed_money_flow is negative
                    self.total_absolute_money_flow += old_signed_money_flow;
                }
            }

            self.money_flows.push_back(signed_money_flow);
            self.prev_typical_price = typical_price;

            (self.total_positive_money_flow / self.total_absolute_money_flow) * 100.0
        }
    }
}

impl Default for MoneyFlowIndex {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for MoneyFlowIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MFI({})", self.n)
    }
}

impl Reset for MoneyFlowIndex {
    fn reset(&mut self) {
        self.money_flows.clear();
        self.prev_typical_price = 0.0;
        self.total_positive_money_flow = 0.0;
        self.total_absolute_money_flow = 0.0;
        self.is_new = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    #[test]
    fn test_new() {
        assert!(MoneyFlowIndex::new(0).is_err());
        assert!(MoneyFlowIndex::new(1).is_ok());
    }

    #[test]
    fn test_next_bar() {
        let mut mfi = MoneyFlowIndex::new(3).unwrap();

        // tp = 2.0
        let bar1 = Bar::new().high(3).low(1).close(2).volume(500.0);
        assert_eq!(mfi.next(&bar1), 50.0);

        // tp = 2.2, fm = 2.2*1000 = 2200, abs_total = 2200, pos_total = 2200
        let bar2 = Bar::new().high(2.3).low(2.0).close(2.3).volume(1000.0);
        assert_eq!(mfi.next(&bar2), 100.0);

        // tp = 8.0, fm = 8*200 = 1600, abs_total = 3800, pos_total = 3800
        let bar3 = Bar::new().high(9).low(7).close(8).volume(200.0);
        assert_eq!(mfi.next(&bar3), 100.0);

        // tp = 4.0, fm = -4.0*500 = -2000, abs_total = 5800 , pos_total = 3800
        let bar4 = Bar::new().high(5).low(3).close(4).volume(500.0);
        assert_eq!(mfi.next(&bar4), 3800.0 / 5800.0 * 100.0);

        // tp = 3.0, fm = -3 * 5000 = -15000, abs_total = 5800+15000-2200=18600, pos_total=3800-2200=1600
        let bar5 = Bar::new().high(4).low(2).close(3).volume(5000.0);
        assert_eq!(mfi.next(&bar5), 1600.0 / 18600.0 * 100.0);

        // tp = 1.5, fm = -1.5*6000= -9000, abs_total=18600+9000-1600=26000, pos_total=0
        let bar6 = Bar::new().high(2).low(1).close(1.5).volume(6000.0);
        assert_eq!(mfi.next(&bar6), 0.0 / 23800.0 * 100.0);

        // tp = 2, fm = 2*7000=14000, abs_total=26000+14000-2000=38000, pos_total=14000
        let bar7 = Bar::new().high(2).low(2).close(2).volume(7000.0);
        assert_eq!(mfi.next(&bar7), 14000.0 / 38000.0 * 100.0);
    }

    #[test]
    fn test_reset() {
        let mut mfi = MoneyFlowIndex::new(3).unwrap();

        let bar1 = Bar::new().high(2).low(1).close(1.5).volume(1000.0);
        let bar2 = Bar::new().high(5).low(3).close(4).volume(2000.0);
        let bar3 = Bar::new().high(9).low(7).close(8).volume(3000.0);
        let bar4 = Bar::new().high(5).low(3).close(4).volume(4000.0);
        let bar5 = Bar::new().high(5).low(3).close(4).volume(5000.0);
        let bar6 = Bar::new().high(2).low(1).close(1.5).volume(6000.0);

        assert_eq!(mfi.next(&bar1), 50.0);
        assert_eq!(mfi.next(&bar2), 100.0);
        assert_eq!(mfi.next(&bar3), 100.0);
        assert_eq!(round(mfi.next(&bar4)), 66.667);
        assert_eq!(round(mfi.next(&bar5)), 73.333);
        assert_eq!(round(mfi.next(&bar6)), 44.444);

        mfi.reset();

        assert_eq!(mfi.next(&bar1), 50.0);
        assert_eq!(mfi.next(&bar2), 100.0);
        assert_eq!(mfi.next(&bar3), 100.0);
        assert_eq!(round(mfi.next(&bar4)), 66.667);
        assert_eq!(round(mfi.next(&bar5)), 73.333);
        assert_eq!(round(mfi.next(&bar6)), 44.444);
    }

    #[test]
    fn test_default() {
        MoneyFlowIndex::default();
    }

    #[test]
    fn test_display() {
        let mfi = MoneyFlowIndex::new(10).unwrap();
        assert_eq!(format!("{}", mfi), "MFI(10)");
    }

}
