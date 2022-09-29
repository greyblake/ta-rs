use std::fmt;

use crate::errors::{Result, TaError};
use crate::{Close, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Volume weight adjusted price (VWAP).
///
/// # Formula
///
/// ![VWAP](https://wikimedia.org/api/rest_v1/media/math/render/svg/6c0a822a0a9e58a127105e818a07061a02851685)
///
/// Where:
///
/// TODO
///
/// # Parameters
///
/// TODO
///
/// # Example
///
/// ```
/// use ta::indicators::VolumeWeightAdjustedPrice;
/// use ta::Next;
///
/// let mut vwap = VolumeWeightAdjustedPrice::new().unwrap();
///
/// let di1 = DataItem::builder()
///             .high(3.0)
///             .low(1.0)
///             .close(2.0)
///             .open(1.5)
///             .volume(1000.0)
///             .build().unwrap();
///
/// let di2 = DataItem::builder()
///             .high(3.0)
///             .low(1.0)
///             .close(1.5)
///             .open(1.5)
///             .volume(300.0)
///             .build().unwrap();
///
/// assert_eq!(vwap.next(&di1), 1000.0);
/// assert_eq!(vwap.next(&di2), 700.0);
/// ```
///
/// # Links
///
/// * [Volume weight adjusted price, Wikipedia](https://en.wikipedia.org/wiki/Volume-weighted_average_price)
///
#[doc(alias = "VWAP")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct VolumeWeightAdjustedPrice {
    vwap: f64
}

impl VolumeWeightAdjustedPrice {
    pub fn new() -> Result<Self> {
        Self {
            vwap: 0.0
        }
    }
}

impl<T: Close + Volume> Next<&T> for VolumeWeightAdjustedPrice {
    type Output = f64;

    fn next(&mut self, input: &T) -> f64 {
        // TODO
    }
}

impl Default for VolumeWeightAdjustedPrice {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for VolumeWeightAdjustedPrice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VWAP")
    }
}

impl Reset for VolumeWeightAdjustedPrice {
    fn reset(&mut self) {
        self.vwap = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    #[test]
    fn test_next_bar() {
        let mut vwap = VolumeWeightAdjustedPrice::new();

        let bar1 = Bar::new().close(1.5).volume(1000.0);
        let bar2 = Bar::new().close(5).volume(5000.0);
        let bar3 = Bar::new().close(4).volume(9000.0);
        let bar4 = Bar::new().close(4).volume(4000.0);

        assert_eq!(vwap.next(&bar1), 1000.0);

        //close > prev_close
        assert_eq!(vwap.next(&bar2), 6000.0);

        // close < prev_close
        assert_eq!(vwap.next(&bar3), -3000.0);

        // close == prev_close
        assert_eq!(vwap.next(&bar4), -3000.0);
    }

    #[test]
    fn test_reset() {
        let mut vwap = VolumeWeightAdjustedPrice::new();

        let bar1 = Bar::new().close(1.5).volume(1000.0);
        let bar2 = Bar::new().close(4).volume(2000.0);
        let bar3 = Bar::new().close(8).volume(3000.0);

        assert_eq!(vwap.next(&bar1), 1000.0);
        assert_eq!(vwap.next(&bar2), 3000.0);
        assert_eq!(vwap.next(&bar3), 6000.0);

        vwap.reset();

        assert_eq!(vwap.next(&bar1), 1000.0);
        assert_eq!(vwap.next(&bar2), 3000.0);
        assert_eq!(vwap.next(&bar3), 6000.0);
    }

    #[test]
    fn test_default() {
        VolumeWeightAdjustedPrice::default();
    }

    #[test]
    fn test_display() {
        let vwap = VolumeWeightAdjustedPrice::new();
        assert_eq!(format!("{}", vwap), "VWAP");
    }
}
