use std::fmt;

use crate::errors::*;
use crate::indicators::{Maximum, Minimum};
use crate::{Close, High, Low, Next, Reset};

/// Fast stochastic oscillator.
///
/// The stochastic oscillator is a momentum indicator comparing the closing price
/// of a security to the range of its prices over a certain period of time.
///
/// # Formula
///
/// ![Fast stochastic oscillator formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/5a419041034a8044308c999f85661a08bcf91b1d)
///
/// Where:
///
/// * \%K<sub>t</sub> - value of fast stochastic oscillator
/// * C<sub>t</sub> - close price of the current period
/// * L<sub>n</sub> - lowest price for the last _n_ periods
/// * H<sub>n</sub> - highest price for the last _n_ periods
///
///
/// # Parameters
///
/// * _length_ - number of periods (integer greater than 0). Default is 14.
///
/// # Example
///
/// ```
/// use ta::indicators::FastStochastic;
/// use ta::Next;
///
/// let mut stoch = FastStochastic::new(5).unwrap();
/// assert_eq!(stoch.next(20.0), 50.0);
/// assert_eq!(stoch.next(30.0), 100.0);
/// assert_eq!(stoch.next(40.0), 100.0);
/// assert_eq!(stoch.next(35.0), 75.0);
/// assert_eq!(stoch.next(15.0), 0.0);
/// ```
#[derive(Debug, Clone)]
pub struct FastStochastic {
    length: u32,
    minimum: Minimum,
    maximum: Maximum,
}

impl FastStochastic {
    pub fn new(length: u32) -> Result<Self> {
        let indicator = Self {
            length: length,
            minimum: Minimum::new(length)?,
            maximum: Maximum::new(length)?,
        };
        Ok(indicator)
    }

    pub fn length(&self) -> u32 {
        self.length
    }
}

impl Next<f64> for FastStochastic {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let min = self.minimum.next(input);
        let max = self.maximum.next(input);

        if min == max {
            // When only 1 input was given, than min and max are the same,
            // therefore it makes sense to return 50
            50.0
        } else {
            (input - min) / (max - min) * 100.0
        }
    }
}

impl<'a, T: High + Low + Close> Next<&'a T> for FastStochastic {
    type Output = f64;

    fn next(&mut self, input: &'a T) -> Self::Output {
        let highest = self.maximum.next(input.high());
        let lowest = self.minimum.next(input.low());
        let close = input.close();

        if highest == lowest {
            // To avoid division by zero, return 50.0
            50.0
        } else {
            (close - lowest) / (highest - lowest) * 100.0
        }
    }
}

impl Reset for FastStochastic {
    fn reset(&mut self) {
        self.minimum.reset();
        self.maximum.reset();
    }
}

impl Default for FastStochastic {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for FastStochastic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FAST_STOCH({})", self.length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(FastStochastic);

    #[test]
    fn test_new() {
        assert!(FastStochastic::new(0).is_err());
        assert!(FastStochastic::new(1).is_ok());
    }

    #[test]
    fn test_next_with_f64() {
        let mut stoch = FastStochastic::new(3).unwrap();
        assert_eq!(stoch.next(0.0), 50.0);
        assert_eq!(stoch.next(200.0), 100.0);
        assert_eq!(stoch.next(100.0), 50.0);
        assert_eq!(stoch.next(120.0), 20.0);
        assert_eq!(stoch.next(115.0), 75.0);
    }

    #[test]
    fn test_next_with_bars() {
        let test_data = vec![
            // high, low , close, expected
            (20.0, 20.0, 20.0, 50.0), // min = 20, max = 20
            (30.0, 10.0, 25.0, 75.0), // min = 10, max = 30
            (40.0, 20.0, 16.0, 20.0), // min = 10, max = 40
            (35.0, 15.0, 19.0, 30.0), // min = 10, max = 40
            (30.0, 20.0, 25.0, 40.0), // min = 15, max = 40
            (35.0, 25.0, 30.0, 75.0), // min = 15, max = 35
        ];

        let mut stoch = FastStochastic::new(3).unwrap();

        for (high, low, close, expected) in test_data {
            let input_bar = Bar::new().high(high).low(low).close(close);
            assert_eq!(stoch.next(&input_bar), expected);
        }
    }

    #[test]
    fn test_reset() {
        let mut indicator = FastStochastic::new(10).unwrap();
        assert_eq!(indicator.next(10.0), 50.0);
        assert_eq!(indicator.next(210.0), 100.0);
        assert_eq!(indicator.next(10.0), 0.0);
        assert_eq!(indicator.next(60.0), 25.0);

        indicator.reset();
        assert_eq!(indicator.next(10.0), 50.0);
        assert_eq!(indicator.next(20.0), 100.0);
        assert_eq!(indicator.next(12.5), 25.0);
    }

    #[test]
    fn test_default() {
        FastStochastic::default();
    }

    #[test]
    fn test_display() {
        let indicator = FastStochastic::new(21).unwrap();
        assert_eq!(format!("{}", indicator), "FAST_STOCH(21)");
    }
}
