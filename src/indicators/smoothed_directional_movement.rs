use crate::{
    errors::{Result, TaError},
    indicators::{NegativeDirectionalMovement, PositiveDirectionalMovement},
    High, Next, Period, Reset,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, fmt};

/// Weighted moving average (WMA).
///
/// A moving average that assigns weights that decrease in arithmetical
/// progression. In an _n_-day WMA the latest day has weight _n_, the second
/// latest _n−1_, etc., down to one.
///
/// # Formula
///
/// ![WMA formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/7780333af18da7e27a1186a3d566e28da21b2840)
///
/// Where:
///
/// * _WMA<sub>M</sub>_ - is the value of the WMA at time _m_
/// * _n_ - is the period.
/// * _p<sub>M</sub>_ - is the input value at a time period t.
///
/// # Example
///
/// ```
/// use ta::indicators::WeightedMovingAverage;
/// use ta::Next;
///
/// let mut wma = WeightedMovingAverage::new(3).unwrap();
/// assert_eq!(wma.next(10.0), 10.0);
/// assert_eq!(wma.next(13.0), 12.0);
/// assert_eq!(wma.next(16.0), 14.0);
/// assert_eq!(wma.next(14.0), 14.5);
/// ```
///
/// # Links
///
/// * [Weighted moving average, Wikipedia](https://en.wikipedia.org/wiki/Moving_average#Weighted_moving_average)
#[doc(alias = "SDM-")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct SmoothedNegativeDirectionalMovement {
    period: usize,
    sum: f64,
    window: VecDeque<f64>,
    ndm: NegativeDirectionalMovement,
}

impl SmoothedNegativeDirectionalMovement {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                period,
                sum: 0.0,
                window: {
                    let mut window = VecDeque::with_capacity(period);
                    window.push_back(0.0);
                    window
                },
                ndm: NegativeDirectionalMovement::new().unwrap(),
            }),
        }
    }
}

impl Period for SmoothedNegativeDirectionalMovement {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for SmoothedNegativeDirectionalMovement {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        // Remove front of window from sum.
        self.sum -= if self.window.len() < self.period {
            *self.window.front().unwrap()
        } else {
            self.window.pop_front().unwrap()
        };
        // Calculate current DM.
        let dm = self.ndm.next(input);
        // Add to window.
        self.window.push_back(dm);
        // Update sum of values in window
        self.sum += dm;

        self.sum - self.sum / self.period as f64 - dm
    }
}

impl<T: High> Next<&T> for SmoothedNegativeDirectionalMovement {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.high())
    }
}

impl Reset for SmoothedNegativeDirectionalMovement {
    fn reset(&mut self) {
        self.sum = 0.0;
        self.window.clear();
        self.window.push_back(0.0);
        self.ndm.reset();
    }
}

impl Default for SmoothedNegativeDirectionalMovement {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for SmoothedNegativeDirectionalMovement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SDM-({})", self.period)
    }
}

#[cfg(test)]
mod tests_negative {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(SmoothedNegativeDirectionalMovement);

    #[test]
    fn test_new() {
        assert!(SmoothedNegativeDirectionalMovement::new(0).is_err());
        assert!(SmoothedNegativeDirectionalMovement::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut sndm = SmoothedNegativeDirectionalMovement::new(3).unwrap();

        sndm.next(10.);
        sndm.next(11.);
        sndm.next(9.);
        sndm.next(12.);

        assert_eq!(sndm.next(11.), -1.0);
    }

    #[test]
    fn test_reset() {}

    #[test]
    fn test_default() {
        SmoothedNegativeDirectionalMovement::default();
    }

    #[test]
    fn test_display() {
        let indicator = SmoothedNegativeDirectionalMovement::new(8).unwrap();
        assert_eq!(format!("{}", indicator), "SDM-(8)");
    }
}

#[doc(alias = "SDM+")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct SmoothedPositiveDirectionalMovement {
    period: usize,
    sum: f64,
    window: VecDeque<f64>,
    pdm: PositiveDirectionalMovement,
}

impl SmoothedPositiveDirectionalMovement {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                period,
                sum: 0.0,
                window: {
                    let mut window = VecDeque::with_capacity(period + 1);
                    window.push_back(0.0);
                    window
                },
                pdm: PositiveDirectionalMovement::new().unwrap(),
            }),
        }
    }
}

impl Period for SmoothedPositiveDirectionalMovement {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for SmoothedPositiveDirectionalMovement {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        if self.period < self.window.len() {
            // Remove front of window from sum.
            self.sum -= self.window.pop_front().unwrap();
        }
        // Calculate current DM.
        let dm = self.pdm.next(input);
        // Add to window.
        self.window.push_back(dm);
        // Update sum of values in window
        self.sum += dm;

        self.sum - self.sum / self.period as f64 + dm
    }
}

impl<T: High> Next<&T> for SmoothedPositiveDirectionalMovement {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.high())
    }
}

impl Reset for SmoothedPositiveDirectionalMovement {
    fn reset(&mut self) {
        self.sum = 0.0;
        self.window.clear();
        self.window.push_back(0.0);
        self.pdm.reset();
    }
}

impl Default for SmoothedPositiveDirectionalMovement {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for SmoothedPositiveDirectionalMovement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SDM+({})", self.period)
    }
}

#[cfg(test)]
mod tests_positive {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(SmoothedPositiveDirectionalMovement);

    #[test]
    fn test_new() {
        assert!(SmoothedPositiveDirectionalMovement::new(0).is_err());
        assert!(SmoothedPositiveDirectionalMovement::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut sdmp = SmoothedPositiveDirectionalMovement::new(3).unwrap();

        sdmp.next(10.);
        sdmp.next(11.);
        sdmp.next(9.);

        assert_eq!(sdmp.next(12.), 11.0);
    }

    #[test]
    fn test_reset() {}

    #[test]
    fn test_default() {
        SmoothedPositiveDirectionalMovement::default();
    }

    #[test]
    fn test_display() {
        let indicator = SmoothedPositiveDirectionalMovement::new(8).unwrap();
        assert_eq!(format!("{}", indicator), "SDM+(8)");
    }
}
