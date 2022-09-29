use std::fmt;

use crate::{Close, Next, Volume, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Volume weight average price (VWAP).
///
/// In finance, volume-weighted average price (VWAP) is the ratio of the value of a security or
/// financial asset traded to the total volume of transactions during a trading session.
/// It is a measure of the average trading price for the period.
///
/// # Formula
///
/// ![VWAP](https://wikimedia.org/api/rest_v1/media/math/render/svg/6c0a822a0a9e58a127105e818a07061a02851685)
///
/// Where:
///
/// vwap - volume weight average price
///
/// # Example
///
/// ```
/// use ta::indicators::VolumeWeightedAveragePrice;
/// use ta::{Next, DataItem};
///
/// let mut vwap = VolumeWeightedAveragePrice::new();
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
/// assert_eq!(vwap.next(&di1), 2.0);
/// assert_eq!(vwap.next(&di2), 1.8846153846153846);
/// ```
///
/// # Links
///
/// * [Volume weight average price, Wikipedia](https://en.wikipedia.org/wiki/Volume-weighted_average_price)
///
#[doc(alias = "VWAP")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct VolumeWeightedAveragePrice {
    cumulative_volume: f64,
    cumulative_traded: f64,
    vwap: f64
}

impl VolumeWeightedAveragePrice {
    pub fn new() -> Self {
        Self {
            cumulative_volume: 0.0,
            cumulative_traded: 0.0,
            vwap: 0.0
        }
    }
}

impl<T: Close + Volume> Next<&T> for VolumeWeightedAveragePrice {
    type Output = f64;

    fn next(&mut self, input: &T) -> f64 {
        let price = input.close();
        let volume = input.volume();
        self.cumulative_volume += volume;
        self.cumulative_traded += price * volume;
        self.vwap = self.cumulative_traded / self.cumulative_volume;
        self.vwap
    }
}

impl Default for VolumeWeightedAveragePrice {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for VolumeWeightedAveragePrice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VWAP")
    }
}

impl Reset for VolumeWeightedAveragePrice {
    fn reset(&mut self) {
        self.cumulative_volume = 0.0;
        self.cumulative_traded = 0.0;
        self.vwap = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    #[test]
    fn test_next_bar() {
        let mut vwap = VolumeWeightedAveragePrice::new();
        let bar1 = Bar::new().close(245.0504667).volume(103033.0);
        let bar2 = Bar::new().close(244.7635667).volume(21168.0);
        let bar3 = Bar::new().close(245.3166667).volume(36544.0);
        assert_eq!(vwap.next(&bar1), 245.0504667);
        assert_eq!(vwap.next(&bar2), 245.001569354568);
        assert_eq!(vwap.next(&bar3), 245.07320403926406);
    }

    #[test]
    fn test_reset() {
        let mut vwap = VolumeWeightedAveragePrice::new();

        let bar1 = Bar::new().close(245.0504667).volume(103033.0);
        let bar2 = Bar::new().close(244.7635667).volume(21168.0);
        let bar3 = Bar::new().close(245.3166667).volume(36544.0);
        assert_eq!(vwap.next(&bar1), 245.0504667);
        assert_eq!(vwap.next(&bar2), 245.001569354568);
        assert_eq!(vwap.next(&bar3), 245.07320403926406);
        vwap.reset();
        assert_eq!(vwap.next(&bar1), 245.0504667);
        assert_eq!(vwap.next(&bar2), 245.001569354568);
        assert_eq!(vwap.next(&bar3), 245.07320403926406);
    }

    #[test]
    fn test_default() {
        VolumeWeightedAveragePrice::default();
    }

    #[test]
    fn test_display() {
        let vwap = VolumeWeightedAveragePrice::new();
        assert_eq!(format!("{}", vwap), "VWAP");
    }
}
