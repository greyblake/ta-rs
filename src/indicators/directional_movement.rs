use crate::{errors::Result, High, Low, Next, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

/// Negative Directional Movement (DM-).
///
/// A direction indicator that is commonly used as a component of the
/// [(Average)](crate::indicators::AverageDirectionalIndex)
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
/// [(Average)](crate::indicators::AverageDirectionalIndex)
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
#[doc(alias = "DM+")]
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
        write!(f, "DM+")
    }
}

#[cfg(test)]
mod tests_negative {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(NegativeDirectionalMovement);

    #[test]
    fn test_new() {
        assert!(NegativeDirectionalMovement::new().is_ok());
    }

    #[test]
    fn test_next() {
        let mut dmm = NegativeDirectionalMovement::new().unwrap();

        dmm.next(10.0);
        dmm.next(11.0);

        assert_eq!(dmm.next(9.0), 2.0);
    }

    #[test]
    fn test_reset() {
        let mut dmm = NegativeDirectionalMovement::new().unwrap();

        dmm.next(10.0);
        dmm.next(11.0);

        dmm.reset();

        assert_eq!(dmm.next(10.0), 10.0);
    }

    #[test]
    fn test_default() {
        NegativeDirectionalMovement::default();
    }

    #[test]
    fn test_display() {
        let indicator = NegativeDirectionalMovement::new().unwrap();
        assert_eq!(format!("{}", indicator), "DM-");
    }
}

#[cfg(test)]
mod tests_positive {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(PositiveDirectionalMovement);

    #[test]
    fn test_new() {
        assert!(PositiveDirectionalMovement::new().is_ok());
    }

    #[test]
    fn test_next() {
        let mut dmp = PositiveDirectionalMovement::new().unwrap();

        dmp.next(10.0);
        dmp.next(11.0);

        assert_eq!(dmp.next(9.0), -2.0);
    }

    #[test]
    fn test_reset() {
        let mut dmp = PositiveDirectionalMovement::new().unwrap();

        dmp.next(10.0);
        dmp.next(11.0);

        dmp.reset();

        assert_eq!(dmp.next(10.0), 10.0);
    }

    #[test]
    fn test_default() {
        PositiveDirectionalMovement::default();
    }

    #[test]
    fn test_display() {
        let indicator = PositiveDirectionalMovement::new().unwrap();
        assert_eq!(format!("{}", indicator), "DM+");
    }
}
