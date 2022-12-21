use std::fmt;

use crate::errors::{Result, TaError};
use crate::{lit, Close, High, Low, Next, NumberType, Period, Reset, Volume};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
/// * _period_ - number of periods, integer greater than 0
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

#[doc(alias = "MFI")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct MoneyFlowIndex {
    period: usize,
    index: usize,
    count: usize,
    previous_typical_price: NumberType,
    total_positive_money_flow: NumberType,
    total_negative_money_flow: NumberType,
    deque: Box<[NumberType]>,
}

impl MoneyFlowIndex {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                period,
                index: 0,
                count: 0,
                previous_typical_price: lit!(0.0),
                total_positive_money_flow: lit!(0.0),
                total_negative_money_flow: lit!(0.0),
                deque: vec![lit!(0.0); period].into_boxed_slice(),
            }),
        }
    }
}

impl Period for MoneyFlowIndex {
    fn period(&self) -> usize {
        self.period
    }
}

impl<T: High + Low + Close + Volume> Next<&T> for MoneyFlowIndex {
    type Output = NumberType;

    fn next(&mut self, input: &T) -> NumberType {
        let tp = (input.close() + input.high() + input.low()) / lit!(3.0);

        self.index = if self.index + 1 < self.period {
            self.index + 1
        } else {
            0
        };

        if self.count < self.period {
            self.count += 1;
            if self.count == 1 {
                self.previous_typical_price = tp;
                return lit!(50.0);
            }
        } else {
            let popped = self.deque[self.index];
            if popped.is_sign_positive() {
                self.total_positive_money_flow -= popped;
            } else {
                self.total_negative_money_flow += popped;
            }
        }

        if tp > self.previous_typical_price {
            let raw_money_flow = tp * input.volume();
            self.total_positive_money_flow += raw_money_flow;
            self.deque[self.index] = raw_money_flow;
        } else if tp < self.previous_typical_price {
            let raw_money_flow = tp * input.volume();
            self.total_negative_money_flow += raw_money_flow;
            self.deque[self.index] = -raw_money_flow;
        } else {
            self.deque[self.index] = lit!(0.0);
        }
        self.previous_typical_price = tp;

        self.total_positive_money_flow
            / (self.total_positive_money_flow + self.total_negative_money_flow)
            * lit!(100.0)
    }
}

impl Default for MoneyFlowIndex {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for MoneyFlowIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MFI({})", self.period)
    }
}

impl Reset for MoneyFlowIndex {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.previous_typical_price = lit!(0.0);
        self.total_positive_money_flow = lit!(0.0);
        self.total_negative_money_flow = lit!(0.0);
        for i in 0..self.period {
            self.deque[i] = lit!(0.0);
        }
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

        let bar1 = Bar::new().high(3).low(1).close(2).volume(500);
        assert_eq!(round(mfi.next(&bar1)), lit!(50.0));

        let bar2 = Bar::new()
            .high(lit!(2.3))
            .low(lit!(2.0))
            .close(lit!(2.3))
            .volume(1000);
        assert_eq!(round(mfi.next(&bar2)), lit!(100.0));

        let bar3 = Bar::new().high(9).low(7).close(8).volume(200);
        assert_eq!(round(mfi.next(&bar3)), lit!(100.0));

        let bar4 = Bar::new().high(5).low(3).close(4).volume(500);
        assert_eq!(round(mfi.next(&bar4)), lit!(65.517));

        let bar5 = Bar::new().high(4).low(2).close(3).volume(5000);
        assert_eq!(round(mfi.next(&bar5)), lit!(8.602));

        let bar6 = Bar::new().high(2).low(1).close(lit!(1.5)).volume(6000);
        assert_eq!(round(mfi.next(&bar6)), lit!(0.0));

        let bar7 = Bar::new().high(2).low(2).close(2).volume(7000);
        assert_eq!(round(mfi.next(&bar7)), lit!(36.842));

        let bar8 = Bar::new().high(2).low(2).close(2).volume(7000);
        assert_eq!(round(mfi.next(&bar8)), lit!(60.87));
    }

    #[test]
    fn test_reset() {
        let mut mfi = MoneyFlowIndex::new(3).unwrap();

        let bar1 = Bar::new().high(3).low(1).close(2).volume(500);
        let bar2 = Bar::new()
            .high(lit!(2.3))
            .low(lit!(2.0))
            .close(lit!(2.3))
            .volume(1000);

        assert_eq!(round(mfi.next(&bar1)), lit!(50.0));
        assert_eq!(round(mfi.next(&bar2)), lit!(100.0));

        mfi.reset();

        assert_eq!(round(mfi.next(&bar1)), lit!(50.0));
        assert_eq!(round(mfi.next(&bar2)), lit!(100.0));
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
