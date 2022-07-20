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

#[doc(alias = "S+DM")]
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
                    let mut window = VecDeque::with_capacity(period);
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
        // Remove front of window from sum.
        self.sum -= if self.window.len() < self.period {
            *self.window.front().unwrap()
        } else {
            self.window.pop_front().unwrap()
        };
        // Calculate current DM.
        let dm = self.pdm.next(input);
        // Add to window.
        self.window.push_back(dm);
        // Update sum of values in window
        self.sum += dm;

        self.sum - self.sum / self.period as f64 - dm
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
    }
}

impl Default for SmoothedPositiveDirectionalMovement {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for SmoothedPositiveDirectionalMovement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "S+DM({})", self.period)
    }
}

#[doc(alias = "S+DM")]
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
    }
}

impl Default for SmoothedNegativeDirectionalMovement {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for SmoothedNegativeDirectionalMovement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "S+DM({})", self.period)
    }
}
