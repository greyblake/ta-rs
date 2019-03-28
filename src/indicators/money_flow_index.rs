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
/// Raw Money Flow(RMF) = Typical Price x Volume
/// RMF is positive when currnt TP is greater that previous period TP and
/// negative when current TP is less than preivous TP
/// Positive money flow (PMF)- calculated by adding the money flow of all the days RMF is positive
/// Negative money flow (PMF)- calculated by adding the money flow of all the days RMF is negative
/// Money Flow Index(MFI) = 100 * (PMF / (PMF + NMF) )
///
/// Where:
///
/// _tp_ - typical price
/// _rmf_ - raw MF
/// _pmf_ - positive MF
/// _nmf_ - negative MF
///
/// # Parameters
///
/// * _n__ - number of periods, integer greater than 0
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
/// ![Moeny Flow Index, Wikipedia](https://en.wikipedia.org/wiki/Money_flow_index)
/// ![Money Flow Index, stockcharts] (https://stockcharts.com/school/doku.php?id=chart_school:technical_indicators:money_flow_index_mfi)

#[derive(Debug, Clone)]
pub struct MoneyFlowIndex {
    n: u32,
    mf_vec: VecDeque<f64>,
    prev_tp: f64,
    pmf: f64,
    sum_mf: f64, // Sum of absolute values of mf
}

impl MoneyFlowIndex {
    pub fn new(n: u32) -> Result<Self> {
        match n {
            0 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            _ => {
                let indicator = Self {
                    n: n,
                    mf_vec: VecDeque::with_capacity(n as usize + 1),
                    prev_tp: 0.0,
                    pmf: 0.0,
                    sum_mf: 0.0,
                };
                Ok(indicator)
            }
        }
    }
}

impl Next<f64> for MoneyFlowIndex {
    type Output = f64;

    fn next(&mut self, mf: f64) -> f64 {
        mf
    }
}

impl<'a, T: High + Low + Close + Volume> Next<&'a T> for MoneyFlowIndex {
    type Output = f64;

    fn next(&mut self, input: &'a T) -> f64 {
        let tp = (input.high() + input.low() + input.close()) / 3.0;
        let rmf = tp * input.volume();
        let mf = if tp >= self.prev_tp {
            self.pmf = self.pmf + rmf; // Adding to pmf as mf will be positive
            rmf
        } else {
            -rmf
        };

        let mut popped_mf = 0.0;
        if (self.mf_vec.len() as u32) == self.n {
            popped_mf = self.mf_vec.pop_front().unwrap();
        }

        if popped_mf > 0.0 {
            self.pmf = self.pmf - popped_mf;
        }
        self.sum_mf = self.sum_mf + mf.abs() - popped_mf.abs();
        self.mf_vec.push_back(mf);
        self.prev_tp = tp;

        ((self.pmf / self.sum_mf) * 100.0)
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
        self.mf_vec.clear();
        self.prev_tp = 0.0;
        self.pmf = 0.0;
        self.sum_mf = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(MoneyFlowIndex);

    #[test]
    fn test_new() {
        assert!(MoneyFlowIndex::new(0).is_err());
        assert!(MoneyFlowIndex::new(1).is_ok());
    }

    #[test]
    fn test_next_bar() {
        let mut mfi = MoneyFlowIndex::new(3).unwrap();

        let bar1 = Bar::new().high(2).low(1).close(1.5).volume(1000.0);
        let bar2 = Bar::new().high(5).low(3).close(4).volume(2000.0);
        let bar3 = Bar::new().high(9).low(7).close(8).volume(3000.0);
        let bar4 = Bar::new().high(5).low(3).close(4).volume(4000.0);
        let bar5 = Bar::new().high(5).low(3).close(4).volume(5000.0);
        let bar6 = Bar::new().high(2).low(1).close(1.5).volume(6000.0);

        assert_eq!(mfi.next(&bar1), 100.0);
        assert_eq!(mfi.next(&bar2), 100.0);
        assert_eq!(mfi.next(&bar3), 100.0);
        assert_eq!(round(mfi.next(&bar4)), 66.667);
        assert_eq!(round(mfi.next(&bar5)), 73.333);
        assert_eq!(round(mfi.next(&bar6)), 44.444);

        let mut mfi_1 = MoneyFlowIndex::new(4).unwrap();

        let bar7 = Bar::new().high(10).low(8).close(9).volume(1000.0);
        let bar8 = Bar::new().high(5).low(3).close(4).volume(2000.0);
        let bar9 = Bar::new().high(9).low(7).close(8).volume(3000.0);
        let bar10 = Bar::new().high(5).low(3).close(4).volume(4000.0);
        let bar11 = Bar::new().high(5).low(3).close(4).volume(5000.0);
        let bar12 = Bar::new().high(2).low(1).close(1.5).volume(6000.0);
        let bar13 = Bar::new().high(5).low(3).close(4).volume(2000.0);
        let bar14 = Bar::new().high(9).low(6).close(9).volume(1000.0);

        assert_eq!(mfi_1.next(&bar7), 100.0);
        assert_eq!(round(mfi_1.next(&bar8)), 52.941);
        assert_eq!(round(mfi_1.next(&bar9)), 80.488);
        assert_eq!(round(mfi_1.next(&bar10)), 57.895);
        assert_eq!(round(mfi_1.next(&bar11)), 64.706);
        assert_eq!(round(mfi_1.next(&bar12)), 63.768);
        assert_eq!(round(mfi_1.next(&bar13)), 52.83);
        assert_eq!(round(mfi_1.next(&bar14)), 80.0);

        let mut mfi_2 = MoneyFlowIndex::new(3).unwrap();

        let bar15 = Bar::new().high(10).low(8).close(9).volume(1000.0);
        let bar16 = Bar::new().high(9).low(7).close(8).volume(2000.0);
        let bar17 = Bar::new().high(8).low(6).close(7).volume(3000.0);
        let bar18 = Bar::new().high(7).low(5).close(6).volume(4000.0);
        let bar19 = Bar::new().high(6).low(4).close(5).volume(5000.0);
        let bar20 = Bar::new().high(7).low(5).close(6).volume(6000.0);
        let bar21 = Bar::new().high(8).low(6).close(7).volume(2000.0);

        assert_eq!(mfi_2.next(&bar15), 100.0);
        assert_eq!(round(mfi_2.next(&bar16)), 36.00);
        assert_eq!(round(mfi_2.next(&bar17)), 19.565);
        assert_eq!(round(mfi_2.next(&bar18)), 0.0);
        assert_eq!(round(mfi_2.next(&bar19)), 0.0);
        assert_eq!(round(mfi_2.next(&bar20)), 42.353);
        assert_eq!(round(mfi_2.next(&bar21)), 66.667);
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

        assert_eq!(mfi.next(&bar1), 100.0);
        assert_eq!(mfi.next(&bar2), 100.0);
        assert_eq!(mfi.next(&bar3), 100.0);
        assert_eq!(round(mfi.next(&bar4)), 66.667);
        assert_eq!(round(mfi.next(&bar5)), 73.333);
        assert_eq!(round(mfi.next(&bar6)), 44.444);

        mfi.reset();

        assert_eq!(mfi.next(&bar1), 100.0);
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
