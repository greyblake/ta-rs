use std::fmt;

use crate::errors::{Result, TaError};
use crate::indicators::{RelativeStrengthIndex, ExponentialMovingAverage};
use crate::{Close, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Quantitative Qualitative Estimation (QQE).
/// 
/// An indicator similar to SuperTrend that uses a smoothed RSI as a base for 
/// two trailing (upper & lower) bands. The band width is derived from a true range of
/// the smoothed RSI base which is then doubly smoothed with a Wilder's Smoothing Function.
///
/// # Example
///
/// ```
/// use ta::indicators::QuantitativeQualitativeEstimation;
/// use ta::Next;
///
/// let mut qqe = QuantitativeQualitativeEstimation::new(5, 5, 3.0).unwrap();
///
/// assert_eq!(round(qqe.next(2.3).into()), (46.11, 23.18));
/// assert_eq!(round(qqe.next(1.4).into()), (43.9, 23.17));
/// assert_eq!(round(qqe.next(2.2).into()), (44.96, 24.54));
/// ```
///
/// # Links
///
/// * [Quantitative Qualitative Estimation, Tradingpedia](https://www.tradingpedia.com/forex-trading-indicators/quantitative-qualitative-estimation)
/// * [Pinescript Implementation, TradingView](https://www.tradingview.com/script/IYfA9R2k-QQE-MT4/)
///

#[doc(alias = "QQE")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct QuantitativeQualitativeEstimation {
    period: usize,
    wilders_multiplier: f64,
    last_smoothed_rsi: f64,
    last_lowerband: f64,
    last_upperband: f64,
    trend: bool,
    rsi: RelativeStrengthIndex,
    // This should really be an option between different moving averages,
    // but I'm unsure on the best way to implement that. MA marker trait maybe?
    rsi_smoother: ExponentialMovingAverage,
    rsi_tr_smoother: ExponentialMovingAverage,
    wilders_smoother: ExponentialMovingAverage,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuantitativeQualitativeEstimationOutput {
    pub rsi_ma: f64,
    pub qqe_combined: f64,
    pub qqe_upperband: f64,
    pub qqe_lowerband: f64,
}

impl From<QuantitativeQualitativeEstimationOutput> for (f64, f64, f64, f64) {
    fn from(qqe_out: QuantitativeQualitativeEstimationOutput) -> Self {
        (qqe_out.rsi_ma, qqe_out.qqe_combined, qqe_out.qqe_upperband, qqe_out.qqe_lowerband)
    }
}

impl From<QuantitativeQualitativeEstimationOutput> for (f64, f64) {
    fn from(qqe_out: QuantitativeQualitativeEstimationOutput) -> Self {
        (qqe_out.rsi_ma, qqe_out.qqe_combined)
    }
}

impl QuantitativeQualitativeEstimation {
    pub fn new(
        period: usize, 
        smooth_period: usize, 
        wilders_multiplier: f64,
    ) -> Result<Self> {
        if wilders_multiplier < 1.0 || period <= 0 {
            Err(TaError::InvalidParameter)
        } else {
            let wilders_period = 2 * period - 1;
            Ok(Self {
                period,
                wilders_multiplier,
                last_smoothed_rsi: 50.0,
                last_lowerband: 0.0,
                last_upperband: 0.0,
                trend: true,
                rsi: RelativeStrengthIndex::new(period)?,
                rsi_smoother: ExponentialMovingAverage::new(smooth_period)?,
                rsi_tr_smoother: ExponentialMovingAverage::new(wilders_period)?,
                wilders_smoother: ExponentialMovingAverage::new(wilders_period)?,
            })
        }
    }
}

impl Period for QuantitativeQualitativeEstimation {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for QuantitativeQualitativeEstimation {
    type Output = QuantitativeQualitativeEstimationOutput;

    fn next(&mut self, input: f64) -> Self::Output {
        // The central indicator
        let rsi_val = self.rsi.next(input);
        let smoothed_rsi = self.rsi_smoother.next(rsi_val);

        // Calculations for the gap width.
        let rsi_tr = (self.last_smoothed_rsi - smoothed_rsi).abs();
        let rsi_tr_smooth = self.rsi_tr_smoother.next(rsi_tr);
        let band_gap = self.wilders_smoother.next(rsi_tr_smooth) * self.wilders_multiplier;

        // Band calculations
        let new_upper = smoothed_rsi + band_gap;
        let upperband = if
                (self.last_smoothed_rsi > self.last_upperband) & 
                (smoothed_rsi > self.last_upperband) &
                (new_upper < self.last_upperband)
            { self.last_upperband } else { new_upper };
        
        let new_lower = smoothed_rsi - band_gap;
        let lowerband = if
                (self.last_smoothed_rsi < self.last_lowerband) & 
                (smoothed_rsi < self.last_lowerband) &
                (new_lower > self.last_lowerband)
            { self.last_lowerband } else { new_lower };

        // Calculate crossovers to determine if we need to update trend direction
        if ((smoothed_rsi > lowerband) & (self.last_smoothed_rsi < self.last_lowerband)) ||
           ((smoothed_rsi <= lowerband) & (self.last_smoothed_rsi >= self.last_lowerband)) 
            { self.trend = true; } // long
        else if ((smoothed_rsi > upperband) & (self.last_smoothed_rsi < self.last_upperband)) ||
                ((smoothed_rsi <= upperband) & (self.last_smoothed_rsi >= self.last_upperband)) 
            { self.trend = false; } // short

        let combined = if self.trend {
            upperband
        } else {
            lowerband
        };

        self.last_smoothed_rsi = smoothed_rsi;
        self.last_upperband = upperband;
        self.last_lowerband = lowerband;

        Self::Output {
            rsi_ma: smoothed_rsi,
            qqe_combined: combined,
            qqe_upperband: upperband,
            qqe_lowerband: lowerband,
        }
    }
}

impl<T: Close> Next<&T> for QuantitativeQualitativeEstimation {
    type Output = QuantitativeQualitativeEstimationOutput;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for QuantitativeQualitativeEstimation {
    fn reset(&mut self) {
        self.last_smoothed_rsi = 50.0;
        self.last_upperband = 0.0;
        self.last_lowerband = 0.0;
        self.trend = true;
        self.rsi.reset();
        self.rsi_smoother.reset();
        self.rsi_tr_smoother.reset();
        self.wilders_smoother.reset();
    }
}

impl Default for QuantitativeQualitativeEstimation {
    fn default() -> Self {
        Self::new(14, 5, 4.236).unwrap()
    }
}

impl fmt::Display for QuantitativeQualitativeEstimation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "QQE({}, {}, {})", 
            self.period, 
            self.rsi_smoother.period(), 
            self.wilders_multiplier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(QuantitativeQualitativeEstimation);

    fn round(nums: (f64, f64)) -> (f64, f64) {
        let n0 = (nums.0 * 100.0).round() / 100.0;
        let n1 = (nums.1 * 100.0).round() / 100.0;
        (n0, n1)
    }

    #[test]
    fn test_new() {
        assert!(QuantitativeQualitativeEstimation::new(0, 5, 4.236).is_err());
        assert!(QuantitativeQualitativeEstimation::new(14, 0, 4.236).is_err());
        assert!(QuantitativeQualitativeEstimation::new(14, 5, 0.0).is_err());
        assert!(QuantitativeQualitativeEstimation::new(14, 5, 0.5).is_err());
        assert!(QuantitativeQualitativeEstimation::new(14, 5, -4.236).is_err());
        assert!(QuantitativeQualitativeEstimation::new(3, 3, 2.0).is_ok());
        assert!(QuantitativeQualitativeEstimation::new(14, 3, 5.45).is_ok());
    }

    #[test]
    fn test_next() {
        let mut qqe = QuantitativeQualitativeEstimation::new(5, 5, 4.236).unwrap();

        let feed_vals = vec![3.0, 4.0, 6.0, 2.0, 3.0, 4.0, 5.6, 2.0, 1.0];
        // The implementation I'm testing against doesn't provide early values.
        // so I feed the first 8 values then test the rest
        for val in feed_vals {
            dbg!(qqe.next(val));
        }

        assert_eq!(round(qqe.next(2.3).into()), (46.11, 23.18));
        assert_eq!(round(qqe.next(1.4).into()), (43.9, 23.17));
        assert_eq!(round(qqe.next(2.2).into()), (44.96, 24.54));

        let bar1 = Bar::new().close(4.0);
        let bar2 = Bar::new().close(2.0);
        assert_eq!(round(qqe.next(&bar1).into()), (50.26, 30.66));
        assert_eq!(round(qqe.next(&bar2).into()), (48.40, 30.66));
    }

    #[test]
    fn test_reset() {
        let mut qqe = QuantitativeQualitativeEstimation::new(5, 5, 3.0).unwrap();

        assert_eq!(round(qqe.next(4.0).into()), (50.0, 50.0));
        qqe.next(10.0);
        qqe.next(15.0);
        qqe.next(20.0);
        assert_ne!(round(qqe.next(4.0).into()), (50.0, 50.0));

        qqe.reset();
        assert_eq!(round(qqe.next(4.0).into()), (50.0, 50.0));
    }

    #[test]
    fn test_default() {
        QuantitativeQualitativeEstimation::default();
    }

    #[test]
    fn test_display() {
        let qqe = QuantitativeQualitativeEstimation::new(7, 3, 5.33).unwrap();
        assert_eq!(format!("{}", qqe), "QQE(7, 3, 5.33)");
    }
}
