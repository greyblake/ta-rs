use std::fmt;

use crate::{High, Low, Next, Reset};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Rotation Factor
///
/// Measures the buy and sell pressure of the day.
///
/// # Formula
/// 
/// * At the start of each trading day, RF<sub>t</sub> = 0.
///
/// Let the High and Low at index <sub>t</sub> be denoted as H<sub>t</sub> and L<sub>t</sub> respectively.
/// 
/// * H<sub>t</sub> > H<sub>t-1</sub>, then RF<sub>t</sub> = RF<sub>t-1</sub> + 1
/// * L<sub>t</sub> > L<sub>t-1</sub>, then RF<sub>t</sub> = RF<sub>t-1</sub> + 1
/// * H<sub>t</sub> < H<sub>t-1</sub>, then RF<sub>t</sub> = RF<sub>t-1</sub> - 1
/// * L<sub>t</sub> < L<sub>t-1</sub>, then RF<sub>t</sub> = RF<sub>t-1</sub> - 1
/// 
/// # Example
///
/// ```
/// use ta::{Next, DataItem};
/// use ta::indicators::RotationFactor;
/// 
/// fn main() {
///    let data = vec![
///     // high, low, rf
///     (10.0, 9.0, 0.0), // 1st bar of day is baseline, RF is always 0
///     (10.4, 9.8, 2.0), // high > prev_high, low > prev_low = 1 + 1 = 2
///     (10.1, 9.8, 1.0), // high < prev_high, low == prev_low = -1 + 0 = (2 - 1) = 1
///     (9.1, 8.1, -1.0), // high < prev_high, low < prev_low = -1 - 1 = (1 - 2) = -1
///    ];
///   let mut indicator = RotationFactor::new();
/// 
///  for (high, low, rf) in data {
///    let di = DataItem::builder()
///     .high(high)
///     .low(low)
///     .build().unwrap();
///   assert_eq!(indicator.next(&di), rf);
///   }
/// }
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct RotationFactor {
    data: Vec<f64>,
    accumulation: f64,
    prev_high: Option<f64>,
    prev_low: Option<f64>,
}

impl RotationFactor {
    pub fn new() -> Self {
        RotationFactor {
            data: vec![],
            accumulation: 0.0,
            prev_high: None,
            prev_low: None,
        }
    }
}

fn calculate_rotation_factor(prev_high: Option<f64>, prev_low: Option<f64>, high: f64, low: f64) -> f64 {
    let mut rf = 0.0;

    if let Some(prev_high_val) = prev_high {
        if high > prev_high_val {
            rf += 1.0;
        } else if high < prev_high_val {
            rf -= 1.0;
        }
    }

    if let Some(prev_low_val) = prev_low {
        if low > prev_low_val {
            rf += 1.0;
        } else if low < prev_low_val {
            rf -= 1.0;
        }
    }

    if let Some(prev_high_val) = prev_high {
        if let Some(prev_low_val) = prev_low {
            if high == prev_high_val - 1.0 || low == prev_low_val - 1.0 {
                rf = 0.0;
            }
        }
    }

    rf
}

impl Reset for RotationFactor {
    fn reset(&mut self) {
        self.data.iter_mut().for_each(|x| *x = 0.0);
        self.accumulation = 0.0;
        self.prev_high = None;
        self.prev_low = None;
    }
}

impl Default for RotationFactor {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for RotationFactor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ROTATION_FACTOR({})", self.data.len())
    }
}

impl Next<f64> for RotationFactor {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let prev_high = self.prev_high;
        let prev_low = self.prev_low;
        let rf = calculate_rotation_factor(prev_high, prev_low, input, input);
        self.prev_high = Some(input);
        self.prev_low = Some(input);
        self.accumulation += rf;
        self.data.push(self.accumulation);
        self.data.remove(0);
        rf
    }
}

impl<'a, T: High + Low> Next<&'a T> for RotationFactor {
    type Output = f64;

    fn next(&mut self, input: &'a T) -> Self::Output {
        let high = input.high();
        let low = input.low();
        let rf = calculate_rotation_factor(self.prev_high, self.prev_low, high, low);
        self.prev_high = Some(high);
        self.prev_low = Some(low);
        self.accumulation += rf;
        self.data.push(self.accumulation);
        self.data.remove(0);
        rf
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(RotationFactor);

    #[test]
    fn test_new() {
        assert!(RotationFactor::new().data.is_empty());
    }

    #[test]
    fn test_next_with_bars() {
        let mut rf = RotationFactor::new();

        let bar1 = Bar::new().high(10.0).low(9.0);
        let bar2 = Bar::new().high(10.4).low(9.8);
        let bar3 = Bar::new().high(10.1).low(9.8);
        let bar4 = Bar::new().high(11.1).low(9.6);

        // First bar as a baseline of 0
        assert_eq!(rf.next(&bar1), 0.0);

        // Start incrementing and decrementing from the second bar onwards
        assert_eq!(rf.next(&bar2), 2.0); 
        assert_eq!(rf.next(&bar3), -1.0); 
        assert_eq!(rf.next(&bar4), 0.0);

    } 

    #[test]
    fn test_reset() {
        let mut rf = RotationFactor::new();

        rf.reset();
        assert!(rf.data.iter().all(|&x| x == 0.0));
        assert_eq!(rf.accumulation, 0.0);
        assert_eq!(rf.prev_high, None);
        assert_eq!(rf.prev_low, None);
    }

    #[test]
    fn test_default() {
        RotationFactor::default();
    }

    #[test]
    fn test_display() {
        let indicator = RotationFactor::new();
        assert_eq!(format!("{}", indicator), "ROTATION_FACTOR(0)");
    } 
}
