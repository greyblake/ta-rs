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

/// Directional Movement Index (DX/DMI)
///
/// A direction indicator, originally developed by J. Welles Wilder. The
/// directional movement index is an N-sample moving average of the
/// a combination of positive & negative directional indicator (DI) values .
///
/// # Parameters
///
/// * `period` - Smoothing period (samples) of SDM and ATR (nonzero integer)
///   used in the DIs.
///
/// # Links
///
/// * [Averager directional moviement index, Wikipedia](https://en.wikipedia.org/wiki/Average_directional_movement_index)
/// * [Directional movement index, Wikipedia (French)](https://fr.wikipedia.org/wiki/Directional_Movement_Index)
#[doc(alias = "DX")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct DirectionalMovementIndex {
    sndm: SmoothedNegativeDirectionalMovement,
    spdm: SmoothedPositiveDirectionalMovement,
    atr: AverageTrueRange,
}

impl DirectionalMovementIndex {
    pub fn new(period: usize) -> Result<Self> {
        Ok(Self {
            sndm: SmoothedNegativeDirectionalMovement::new(period)?,
            spdm: SmoothedPositiveDirectionalMovement::new(period)?,
            atr: AverageTrueRange::new(period)?,
        })
    }
}

impl Period for DirectionalMovementIndex {
    fn period(&self) -> usize {
        self.atr.period()
    }
}

impl Next<f64> for DirectionalMovementIndex {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let atr = self.atr.next(input);
        let ndi = self.sndm.next(input) / atr;
        let pdi = self.spdm.next(input) / atr;

        100.0 * ((pdi - ndi).abs() / (pdi + ndi).abs())
    }
}

impl<T: High> Next<&T> for DirectionalMovementIndex {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.high())
    }
}

impl Reset for DirectionalMovementIndex {
    fn reset(&mut self) {
        self.sndm.reset();
        self.spdm.reset();
        self.atr.reset()
    }
}

impl Default for DirectionalMovementIndex {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for DirectionalMovementIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DX({})", self.period())
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(ExponentialMovingAverage);

    #[test]
    fn test_new() {
        assert!(ExponentialMovingAverage::new(0).is_err());
        assert!(ExponentialMovingAverage::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut ema = ExponentialMovingAverage::new(3).unwrap();

        assert_eq!(ema.next(2.0), 2.0);
        assert_eq!(ema.next(5.0), 3.5);
        assert_eq!(ema.next(1.0), 2.25);
        assert_eq!(ema.next(6.25), 4.25);

        let mut ema = ExponentialMovingAverage::new(3).unwrap();
        let bar1 = Bar::new().close(2);
        let bar2 = Bar::new().close(5);
        assert_eq!(ema.next(&bar1), 2.0);
        assert_eq!(ema.next(&bar2), 3.5);
    }

    #[test]
    fn test_reset() {
        let mut ema = ExponentialMovingAverage::new(5).unwrap();

        assert_eq!(ema.next(4.0), 4.0);
        ema.next(10.0);
        ema.next(15.0);
        ema.next(20.0);
        assert_ne!(ema.next(4.0), 4.0);

        ema.reset();
        assert_eq!(ema.next(4.0), 4.0);
    }

    #[test]
    fn test_default() {
        ExponentialMovingAverage::default();
    }

    #[test]
    fn test_display() {
        let ema = ExponentialMovingAverage::new(7).unwrap();
        assert_eq!(format!("{}", ema), "EMA(7)");
    }
}
*/
