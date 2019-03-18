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
/// _mfr_ - MF ratio
///
/// # Parameters
///
/// * _n__ - number of periods, integer greater than 0
///
/// # Example
///
///```
/// use ta::indicators::MoneyFlowIndex;
/// use ta::Next;
///
/// let mut mfi  = MoneyflowIndex::new(3).unwrap();
///
/// let mfi_1 = mfi.next(100.0, 98.0, 99.0, 10101010)
///                     // high, low , close,volume
///
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

impl Next<(f64, f64, f64, f64)> for MoneyFlowIndex {
    type Output = f64;

    fn next(&mut self, input: (f64, f64, f64, f64)) -> f64 {
        let high = input.0;
        let low = input.1;
        let close = input.2;
        let volume = input.3;

        let tp = (high + low + close) / 3.0;
        let rmf = tp * volume;
        let mf = if tp >= self.prev_tp {
            rmf
        } else {
            (rmf * -1.0)
        };

        if (self.mf_vec.len() as u32) < self.n {
            self.mf_vec.push_back(mf);
        } else {
            self.mf_vec.pop_front();
            self.mf_vec.push_back(mf);
        }

        if self.mf_vec.len() as u32 == self.n {
            let pmf: f64 = self.mf_vec.iter().filter(|&i| *i >= 0.0).sum();
            let nmf: f64 = self.mf_vec.iter().filter(|&i| *i < 0.0).sum();
            (pmf / (pmf + nmf))
        } else {
            0.0
        }
    }
}

impl<'a, T: High + Low + Close + Volume> Next<&'a T> for MoneyFlowIndex {
    type Output = f64;

    fn next(&mut self, input: &'a T) -> Self::Output {
        self.next((input.high(), input.low(), input.close(), input.volume()))
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
        assert!(MoneyFlowIndex::new(0).is_ok());
    }

    #[test]
    fn test_next_f64() {
        let mut mfi = MoneyFlowIndex::new(3).unwrap();
    }

    #[test]
    fn test_next_bar() {
        fn bar(high: f64, low: f64, close: f64,volume:u32 ) -> Bar {
            Bar::new().close(close).high(high)
        }

        let mut mfi = MoneyFlowIndex::new(3).unwrap();

        let bar1 = Bar::new().high(10).low(7.5).close(9).volume(1000);
        let bar2 = Bar::new().high(11).low(9).close(9.5).volume(2000);
        let bar3 = Bar::new().high(9).low(5).close(8).volume(3000);

        assert_eq!(mfi.next(&bar1), 2.5);
        assert_eq!(mfi.next(&bar2), 2.0);
        assert_eq!(mfi.next(&bar3), 4.5);
    }

    #[test]
    fn test_reset() {
        let mut mfi = MoneyFlowIndex::new(3).unwrap();

        mfi.next((0.0, 0.0, 0.0, 0.0));
        mfi.next((0.0, 0.0, 0.0, 0.0));

        mfi.reset();
    }
}
