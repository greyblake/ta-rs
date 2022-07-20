use crate::{errors::Result, High, Low, Next, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

/// Negative Directional Movement (DM-).
///
/// A direction indicator that is commonly used as a component of the
/// [(Average)](crate::indicators::AverageDirectionalMovementIndex)
/// [Directional Movement Index](crate::indicators::DirectionalMovementIndex)
/// (ADX/DX).
///
/// # Formula
///
/// DM-<sub>t</sub> = low<sub>t-1</sub> - low<sub>t</sub>
///
/// Where:
///
/// * _DM-<sub>t</sub>_ – [Negative Directional
///   Movement](crate::indicators::NegativeDirectionalMovement) at time _t_.
#[doc(alias = "DM-")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct NegativeDirectionalMovement {
    current: f64,
    is_new: bool,
}

impl NegativeDirectionalMovement {
    pub fn new() -> Result<Self> {
        Ok(Self {
            current: 0.0,
            is_new: true,
        })
    }
}

impl Next<f64> for NegativeDirectionalMovement {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        if self.is_new {
            self.is_new = false;
            self.current = input;
            self.current
        } else {
            let next = self.current - input;
            self.current = input;
            next
        }
    }
}

impl<T: Low> Next<&T> for NegativeDirectionalMovement {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.low())
    }
}

impl Reset for NegativeDirectionalMovement {
    fn reset(&mut self) {
        self.is_new = true;
    }
}

impl Default for NegativeDirectionalMovement {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

impl fmt::Display for NegativeDirectionalMovement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DM-")
    }
}

/// Positive Directional Movement (DM+).
///
/// A direction indicator that is commonly used as a component of the
/// [(Average)](crate::indicators::AverageDirectionalMovementIndex)
/// [Directional Movement Index](crate::indicators::DirectionalMovementIndex)
/// (ADX/DX).
///
/// # Formula
///
/// DM+<sub>t</sub> = high<sub>t</sub> - high<sub>t-1</sub>
///
/// Where:
///
/// * _DM+<sub>t</sub>_ – [Positive Directional
///   Movement](crate::indicators::PositiveDirectionalMovement) at time _t_.
#[doc(alias = "+DM")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct PositiveDirectionalMovement {
    current: f64,
    is_new: bool,
}

impl PositiveDirectionalMovement {
    pub fn new() -> Result<Self> {
        Ok(Self {
            current: 0.0,
            is_new: true,
        })
    }
}

impl Next<f64> for PositiveDirectionalMovement {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        if self.is_new {
            self.is_new = false;
            self.current = input;
            self.current
        } else {
            let next = input - self.current;
            self.current = input;
            next
        }
    }
}

impl<T: High> Next<&T> for PositiveDirectionalMovement {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.high())
    }
}

impl Reset for PositiveDirectionalMovement {
    fn reset(&mut self) {
        self.is_new = true;
    }
}

impl Default for PositiveDirectionalMovement {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

impl fmt::Display for PositiveDirectionalMovement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+DI")
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
}
*/
