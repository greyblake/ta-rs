use crate::{
    errors::Result,
    indicators::{
        AverageTrueRange, SmoothedNegativeDirectionalMovement, SmoothedPositiveDirectionalMovement,
    },
    High, Next, Period, Reset,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

/// Negative Directional Indicator (DI-).
///
/// A downtrend indicator, originally developed by J. Welles Wilder. The
/// negative directional indicator is an N-sample smoothed moving average of the
/// smoothed negative directional movement (SDM-) values normalized by the
/// average true range (ATR).
///
/// # Formula
///
/// DI- = SDM-<sub>t</sub> / ATR(period)<sub>t</sub>
///
/// Where:
///
/// * _SDM-(period)<sub>t</sub>_ – [Smoothed negative directional
///   movement](crate::indicators::SmoothedNegativeDirectionalMovement) over
///   _period_ at time _t_.
/// * _ATR(period)<sub>t</sub>_ – [Averag true
///   range](crate::indicators::AverageTrueRange) over _period_ at time _t_.
///
/// # Parameters
///
/// * `period` - Smoothing period (number of samples) of SDM- and ATR (positive
///   integer).
///
/// # Links
///
/// * [Average directional movement index, Wikipedia](https://en.wikipedia.org/wiki/Average_directional_movement_index)
#[doc(alias = "DI-")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct NegativeDirectionalIndicator {
    sndm: SmoothedNegativeDirectionalMovement,
    atr: AverageTrueRange,
}

impl NegativeDirectionalIndicator {
    pub fn new(period: usize) -> Result<Self> {
        Ok(Self {
            sndm: SmoothedNegativeDirectionalMovement::new(period)?,
            atr: AverageTrueRange::new(period)?,
        })
    }
}

impl Period for NegativeDirectionalIndicator {
    fn period(&self) -> usize {
        self.sndm.period()
    }
}

impl Next<f64> for NegativeDirectionalIndicator {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        100.0 * (self.sndm.next(input) / self.atr.next(input))
    }
}

impl<T: High> Next<&T> for NegativeDirectionalIndicator {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.high())
    }
}

impl Reset for NegativeDirectionalIndicator {
    fn reset(&mut self) {
        self.sndm.reset();
        self.atr.reset();
    }
}

impl Default for NegativeDirectionalIndicator {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for NegativeDirectionalIndicator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DI-({})", self.sndm.period())
    }
}

/// Positive Directional Indicator (DI+).
///
/// An uptrend indicator, originally developed by J. Welles
/// Wilder. The positive directional indicator is an N-sample smoothed moving
/// average of the smoothed positive directional movement (SDM+) values
/// normalized by the average true range (ATR).
///
/// # Formula
///
/// DI+(period)<sub>t</sub> = SDM+(period)<sub>t</sub> / ATR(period)<sub>t</sub>
///
/// Where:
///
/// * _SDM+(period)<sub>t</sub>_ – [Smoothed positive directional
///   movement](crate::indicators::SmoothedPositiveDirectionalMovement) over
///   _period_ at time _t_.
/// * _ATR(period)<sub>t</sub>_ – [Averag true
///   range](crate::indicators::AverageTrueRange) over _period_ at time _t_.
///
/// # Parameters
///
/// * `period` - Smoothing period (number of samples) of SDM+ and ATR (positive
///   integer).
///
/// # Links
///
/// * [Average directional movement index, Wikipedia](https://en.wikipedia.org/wiki/Average_directional_movement_index)
#[doc(alias = "DI+")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct PositiveDirectionalIndicator {
    spdm: SmoothedPositiveDirectionalMovement,
    atr: AverageTrueRange,
}

impl PositiveDirectionalIndicator {
    pub fn new(period: usize) -> Result<Self> {
        Ok(Self {
            spdm: SmoothedPositiveDirectionalMovement::new(period)?,
            atr: AverageTrueRange::new(period)?,
        })
    }
}

impl Period for PositiveDirectionalIndicator {
    fn period(&self) -> usize {
        self.spdm.period()
    }
}

impl Next<f64> for PositiveDirectionalIndicator {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        100.0 * (self.spdm.next(input) / self.atr.next(input))
    }
}

impl<T: High> Next<&T> for PositiveDirectionalIndicator {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.high())
    }
}

impl Reset for PositiveDirectionalIndicator {
    fn reset(&mut self) {
        self.spdm.reset();
        self.atr.reset();
    }
}

impl Default for PositiveDirectionalIndicator {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for PositiveDirectionalIndicator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DI+({})", self.spdm.period())
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(AverageTrueRange);

    #[test]
    fn test_new() {
        assert!(AverageTrueRange::new(0).is_err());
        assert!(AverageTrueRange::new(1).is_ok());
    }
    #[test]
    fn test_next() {
        let mut atr = AverageTrueRange::new(3).unwrap();

        let bar1 = Bar::new().high(10).low(7.5).close(9);
        let bar2 = Bar::new().high(11).low(9).close(9.5);
        let bar3 = Bar::new().high(9).low(5).close(8);

        assert_eq!(atr.next(&bar1), 2.5);
        assert_eq!(atr.next(&bar2), 2.25);
        assert_eq!(atr.next(&bar3), 3.375);
    }

    #[test]
    fn test_reset() {
        let mut atr = AverageTrueRange::new(9).unwrap();

        let bar1 = Bar::new().high(10).low(7.5).close(9);
        let bar2 = Bar::new().high(11).low(9).close(9.5);

        atr.next(&bar1);
        atr.next(&bar2);

        atr.reset();
        let bar3 = Bar::new().high(60).low(15).close(51);
        assert_eq!(atr.next(&bar3), 45.0);
    }

    #[test]
    fn test_default() {
        AverageTrueRange::default();
    }

    #[test]
    fn test_display() {
        let indicator = AverageTrueRange::new(8).unwrap();
        assert_eq!(format!("{}", indicator), "ATR(8)");
    }
}*/
