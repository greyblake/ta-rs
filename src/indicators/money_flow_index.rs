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
                };
                Ok(indicator)
            }
        }
    }
}

impl Next<f64> for MoneyFlowIndex {
    type Output = f64;

    fn next(&mut self, mf: f64) -> f64 {
        if (self.mf_vec.len() as u32) < self.n {
            self.mf_vec.push_back(mf);
        } else {
            self.mf_vec.pop_front();
            self.mf_vec.push_back(mf);
        }

        if self.mf_vec.len() as u32 == self.n {
            let pmf: f64 = self.mf_vec.iter().filter(|&i| *i >= 0.0).sum();
            let nmf: f64 = self.mf_vec.iter().filter(|&i| *i < 0.0).sum();
            ((pmf / (pmf + nmf.abs())) * 100.0)
        } else {
            0.0
        }
    }
}

impl<'a, T: High + Low + Close + Volume> Next<&'a T> for MoneyFlowIndex {
    type Output = f64;

    fn next(&mut self, input: &'a T) -> f64 {
        // self.next((input.high(), input.low(), input.close(), input.volume()))

        let high = input.high();
        let low = input.low();
        let close = input.close();
        let volume = input.volume();

        let tp = (high + low + close) / 3.0;
        let rmf = tp * volume;
        let mf = if tp >= self.prev_tp {
            rmf
        } else {
            (rmf * -1.0)
        };

        self.prev_tp = tp;

        if (self.mf_vec.len() as u32) < self.n {
            self.mf_vec.push_back(mf);
        } else {
            self.mf_vec.pop_front();
            self.mf_vec.push_back(mf);
        }

        if self.mf_vec.len() as u32 == self.n {
            let pmf: f64 = self.mf_vec.iter().filter(|&i| *i >= 0.0).sum();
            let nmf: f64 = self.mf_vec.iter().filter(|&i| *i < 0.0).sum();

            ((pmf / (pmf + nmf.abs())) * 100.0)
        } else {
            0.0
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
        self.mf_vec.clear();
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
    fn test_next_f64() {
        let mut mfi = MoneyFlowIndex::new(4).unwrap();

        assert_eq!(mfi.next(1000.0), 0.0);
        assert_eq!(mfi.next(2000.0), 0.0);
        assert_eq!(mfi.next(-1000.0), 0.0);
        assert_eq!(round(mfi.next(3000.0)), 85.714);
        assert_eq!(round(mfi.next(2000.0)), 87.500);
        assert_eq!(round(mfi.next(-3000.0)), 55.556);
        assert_eq!(round(mfi.next(-2000.0)), 50.000);
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

        assert_eq!(mfi.next(&bar1), 0.0);
        assert_eq!(mfi.next(&bar2), 0.0);
        assert_eq!(mfi.next(&bar3), 100.0);
        assert_eq!(round(mfi.next(&bar4)), 66.667);
        assert_eq!(round(mfi.next(&bar5)), 73.333);
        assert_eq!(round(mfi.next(&bar6)), 44.444);
    }

    #[test]
    fn test_reset() {
        let mut mfi = MoneyFlowIndex::new(3).unwrap();

        let bar1 = Bar::new().high(2).low(1).close(1.5).volume(1000.0);
        let bar2 = Bar::new().high(5).low(3).close(4).volume(2000.0);
        let bar3 = Bar::new().high(9).low(7).close(8).volume(3000.0);
        let bar4 = Bar::new().high(5).low(3).close(4).volume(4000.0);

        assert_eq!(mfi.next(&bar1), 0.0);
        assert_eq!(mfi.next(&bar2), 0.0);
        assert_eq!(mfi.next(&bar3), 100.0);
        assert_eq!(round(mfi.next(&bar4)), 66.667);

        mfi.reset();

        assert_eq!(round(mfi.next(&bar4)), 0.0);
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
