use std::fmt;

use crate::errors::*;
use crate::{Close, Next, Reset};
use crate::indicators::ExponentialMovingAverage;

/// A Bollinger Bands (BB).
/// (BB).
/// It is a type of infinite impulse response filter that calculates Bollinger Bands using Exponential Moving Average.
/// The Bollinger Badns are represented by Average EMA and standard deviaton that is moved 'k' times away in both directions of calculated average value.
/// 
/// # Formula
///
/// Bollinger Bands are calculated based on EMA combined with Standard Deviaiation(SD).
///
/// See EMA doumentation.
///
/// ![SD formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/8715d659116bec91d48762b9e1f3d9aed36fc028)
///
/// Where:
///
/// * _SD<sub>s</sub>_ - is value of standard deviation for N given probes.
/// * _SD<sub>ٍ-</sub>_  - is the mean value of observation.
/// * _SD<sub>N</sub>_ - is number of probes in observation.
/// * _SD<sud>xi</sub>_ - is i-th observed value from N elements observation.
///
/// and then BB is composed as:
///
///    Middle Band = Exponential Moving Average (EMA).
///    Upper Band = N * (EMA + SD of observation * multipler (usually 2.0))
///    Lower Band = N * (EMA - SD of observation * multipler (usually 2.0))
///
/// # Example
///
///```
/// use ta::indicators::{BollingerBands, BollingerBandsValue};
/// use ta::Next;
///
/// let mut bb = BollingerBands::new(BandsType::SMA, 20, 2.0_f64).unwrap();
///
///
/// assert_eq!(bb.next(2.0), BandsPayload {
///     average: 2.0_f64,
///     upper: 2.0_f64,
///     lower: 2.0_f64,
/// });
///
/// assert_eq!(bb.next(4.0), BandsPayload {
///     average: 3.0_f64,
///     upper: 0.0_f64,
///     lower: 4.0_f64,
/// });
/// ```
///
/// # Links
///
/// ![Bollinger Bands, Wikipedia](https://en.wikipedia.org/wiki/Bollinger_Bands)
///
///


