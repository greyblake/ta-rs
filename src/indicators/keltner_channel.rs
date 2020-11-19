use std::fmt;

use crate::errors::Result;
use crate::indicators::{AverageTrueRange, ExponentialMovingAverage};
use crate::{Close, High, Low, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Keltner Channel (KC).
///
/// A Keltner Channel is an indicator showing the Average True Range (ATR) of a
/// price surrounding a central moving average. The ATR bands are typically
/// shown 'k' times moved away from the moving average.
///
/// # Formula
///
/// See EMA, ATR documentation.
///
/// KC is composed as:
///
///  * _KC<sub>Middle Band</sub>_ - Exponential Moving Average (EMA).
///  * _KC<sub>Upper Band</sub>_ = EMA + ATR of observation * multipler (usually 2.0)
///  * _KC<sub>Lower Band</sub>_ = EMA - ATR of observation * multipler (usually 2.0)
///
/// # Example
///
///```
/// use ta::indicators::{KeltnerChannel, KeltnerChannelOutput};
/// use ta::Next;
///
/// let mut kc = KeltnerChannel::new(3, 2.0_f64).unwrap();
///
/// let out_0 = kc.next(2.0);
///
/// let out_1 = kc.next(5.0);
///
/// assert_eq!(out_0.average, 2.0);
/// assert_eq!(out_0.upper, 2.0);
/// assert_eq!(out_0.lower, 2.0);
///
/// assert_eq!(out_1.average, 3.5);
/// assert_eq!(out_1.upper, 6.5);
/// assert_eq!(out_1.lower, 0.5);
/// ```
///
/// # Links
///
/// * [Keltner channel, Wikipedia](https://en.wikipedia.org/wiki/Keltner_channel)
#[doc(alias = "KC")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct KeltnerChannel {
    period: usize,
    multiplier: f64,
    atr: AverageTrueRange,
    ema: ExponentialMovingAverage,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeltnerChannelOutput {
    pub average: f64,
    pub upper: f64,
    pub lower: f64,
}

impl KeltnerChannel {
    pub fn new(period: usize, multiplier: f64) -> Result<Self> {
        Ok(Self {
            period,
            multiplier,
            atr: AverageTrueRange::new(period)?,
            ema: ExponentialMovingAverage::new(period)?,
        })
    }

    pub fn multiplier(&self) -> f64 {
        self.multiplier
    }
}

impl Period for KeltnerChannel {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for KeltnerChannel {
    type Output = KeltnerChannelOutput;

    fn next(&mut self, input: f64) -> Self::Output {
        let atr = self.atr.next(input);
        let average = self.ema.next(input);

        Self::Output {
            average,
            upper: average + atr * self.multiplier,
            lower: average - atr * self.multiplier,
        }
    }
}

impl<T: Close + High + Low> Next<&T> for KeltnerChannel {
    type Output = KeltnerChannelOutput;

    fn next(&mut self, input: &T) -> Self::Output {
        let typical_price = (input.close() + input.high() + input.low()) / 3.0;

        let average = self.ema.next(typical_price);
        let atr = self.atr.next(input);

        Self::Output {
            average,
            upper: average + atr * self.multiplier,
            lower: average - atr * self.multiplier,
        }
    }
}

impl Reset for KeltnerChannel {
    fn reset(&mut self) {
        self.atr.reset();
        self.ema.reset();
    }
}

impl Default for KeltnerChannel {
    fn default() -> Self {
        Self::new(10, 2_f64).unwrap()
    }
}

impl fmt::Display for KeltnerChannel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KC({}, {})", self.period, self.multiplier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(KeltnerChannel);

    #[test]
    fn test_new() {
        assert!(KeltnerChannel::new(0, 2_f64).is_err());
        assert!(KeltnerChannel::new(1, 2_f64).is_ok());
        assert!(KeltnerChannel::new(2, 2_f64).is_ok());
    }

    #[test]
    fn test_next() {
        let mut kc = KeltnerChannel::new(3, 2.0_f64).unwrap();

        let a = kc.next(2.0);
        let b = kc.next(5.0);
        let c = kc.next(1.0);
        let d = kc.next(6.25);

        assert_eq!(round(a.average), 2.0);
        assert_eq!(round(b.average), 3.5);
        assert_eq!(round(c.average), 2.25);
        assert_eq!(round(d.average), 4.25);

        assert_eq!(round(a.upper), 2.0);
        assert_eq!(round(b.upper), 6.5);
        assert_eq!(round(c.upper), 7.75);
        assert_eq!(round(d.upper), 12.25);

        assert_eq!(round(a.lower), 2.0);
        assert_eq!(round(b.lower), 0.5);
        assert_eq!(round(c.lower), -3.25);
        assert_eq!(round(d.lower), -3.75);
    }

    #[test]
    fn test_next_with_data_item() {
        let mut kc = KeltnerChannel::new(3, 2.0_f64).unwrap();

        let dt1 = Bar::new().low(1.2).high(1.7).close(1.3); // typical_price = 1.4
        let o1 = kc.next(&dt1);
        assert_eq!(round(o1.average), 1.4);
        assert_eq!(round(o1.lower), 0.4);
        assert_eq!(round(o1.upper), 2.4);

        let dt2 = Bar::new().low(1.3).high(1.8).close(1.4); // typical_price = 1.5
        let o2 = kc.next(&dt2);
        assert_eq!(round(o2.average), 1.45);
        assert_eq!(round(o2.lower), 0.45);
        assert_eq!(round(o2.upper), 2.45);

        let dt3 = Bar::new().low(1.4).high(1.9).close(1.5); // typical_price = 1.6
        let o3 = kc.next(&dt3);
        assert_eq!(round(o3.average), 1.525);
        assert_eq!(round(o3.lower), 0.525);
        assert_eq!(round(o3.upper), 2.525);
    }

    #[test]
    fn test_reset() {
        let mut kc = KeltnerChannel::new(5, 2.0_f64).unwrap();

        let out = kc.next(3.0);

        assert_eq!(out.average, 3.0);
        assert_eq!(out.upper, 3.0);
        assert_eq!(out.lower, 3.0);

        kc.next(2.5);
        kc.next(3.5);
        kc.next(4.0);

        let out = kc.next(2.0);

        assert_eq!(round(out.average), 2.914);
        assert_eq!(round(out.upper), 4.864);
        assert_eq!(round(out.lower), 0.963);

        kc.reset();
        let out = kc.next(3.0);
        assert_eq!(out.average, 3.0);
        assert_eq!(out.lower, 3.0);
        assert_eq!(out.upper, 3.0);
    }

    #[test]
    fn test_default() {
        KeltnerChannel::default();
    }

    #[test]
    fn test_display() {
        let kc = KeltnerChannel::new(10, 3.0_f64).unwrap();
        assert_eq!(format!("{}", kc), "KC(10, 3)");
    }
}
