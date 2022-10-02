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
/// let mut vwap = VolumeWeightedAveragePrice::new(1.0);
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
/// assert_eq!(vwap.next(&di1), (2.0, 2.0, 2.0));
/// assert_eq!(vwap.next(&di2), (1.8846153846153846, 1.6739528624980127, 2.0952779067327563));
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
    std_dev_multiplier: f64,
    cumulative_volume: f64,
    cumulative_traded: f64,
    cumulative_traded_squared: f64
}

impl VolumeWeightedAveragePrice {
    pub fn new(std_dev_multiplier: f64) -> Self {
        Self {
            std_dev_multiplier: std_dev_multiplier,
            cumulative_volume: 0.0,
            cumulative_traded: 0.0,
            cumulative_traded_squared: 0.0
        }
    }
}

impl<T: Close + Volume> Next<&T> for VolumeWeightedAveragePrice {
    type Output = (f64, f64, f64);

    /*
    computeVWAP(src, isNewPeriod, stDevMultiplier) =>
        var float cumulative_traded = na
        var float cumulative_volume = na
        var float cumulative_traded_squared = na

        cumulative_traded += src * volume
        cumulative_volume += volume
        cumulative_traded_squared += volume * pow(src, 2)

        _vwap = cumulative_traded / cumulative_volume
        variance = cumulative_traded_squared / cumulative_volume - pow(_vwap, 2)
        variance := variance < 0 ? 0 : variance
        stDev = sqrt(variance)

        lowerBand = _vwap - stDev * stDevMultiplier
        upperBand = _vwap + stDev * stDevMultiplier

        [_vwap, lowerBand, upperBand]
    */

    fn next(&mut self, input: &T) -> (f64, f64, f64) {
        let price = input.close();
        let volume = input.volume();
        self.cumulative_volume += volume;
        self.cumulative_traded += price * volume;
        self.cumulative_traded_squared += volume * price.powf(2.0);
        let vwap = self.cumulative_traded / self.cumulative_volume;
        let variance = self.cumulative_traded_squared / self.cumulative_volume - vwap.powf(2.0);
        let variance = if variance < 0.0 { 0.0 } else { variance };
        let std_dev = variance.sqrt();
        println!("variance = {} std_dev = {}", variance, std_dev);
        let lower_band = vwap - std_dev * self.std_dev_multiplier;
        let upper_band = vwap + std_dev * self.std_dev_multiplier;
        (vwap, lower_band, upper_band)
    }
}

impl Default for VolumeWeightedAveragePrice {
    fn default() -> Self {
        Self::new(1.0)
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
        self.cumulative_traded_squared = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    #[test]
    fn test_next_bar() {
        let mut vwap = VolumeWeightedAveragePrice::new(1.25);
        let bar1 = Bar::new().close(245.0504667).volume(103033.0);
        let bar2 = Bar::new().close(244.7635667).volume(21168.0);
        let bar3 = Bar::new().close(245.3166667).volume(36544.0);
        assert_eq!(vwap.next(&bar1), (245.0504667, 245.0504667, 245.0504667));
        assert_eq!(vwap.next(&bar2), (245.001569354568, 244.86672165111835, 245.13641705801766));
        assert_eq!(vwap.next(&bar3), (245.07320403926406, 244.86997872595126, 245.27642935257686));
    }

    #[test]
    fn test_reset() {
        let mut vwap = VolumeWeightedAveragePrice::new(1.25);
        let bar1 = Bar::new().close(245.0504667).volume(103033.0);
        let bar2 = Bar::new().close(244.7635667).volume(21168.0);
        let bar3 = Bar::new().close(245.3166667).volume(36544.0);
        assert_eq!(vwap.next(&bar1), (245.0504667, 245.0504667, 245.0504667));
        assert_eq!(vwap.next(&bar2), (245.001569354568, 244.86672165111835, 245.13641705801766));
        assert_eq!(vwap.next(&bar3), (245.07320403926406, 244.86997872595126, 245.27642935257686));
        vwap.reset();
        assert_eq!(vwap.next(&bar1), (245.0504667, 245.0504667, 245.0504667));
        assert_eq!(vwap.next(&bar2), (245.001569354568, 244.86672165111835, 245.13641705801766));
        assert_eq!(vwap.next(&bar3), (245.07320403926406, 244.86997872595126, 245.27642935257686));
    }

    #[test]
    fn test_default() {
        VolumeWeightedAveragePrice::default();
    }

    #[test]
    fn test_display() {
        let vwap = VolumeWeightedAveragePrice::new(1.0);
        assert_eq!(format!("{}", vwap), "VWAP");
    }
}
