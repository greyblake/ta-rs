//! ta is a Rust library for technical analysis. It provides number of technical
//! indicators that can be used to build trading strategies for stock markets,
//! futures, forex, cryptocurrencies, etc.
//!
//! Every indicator is implemented as a data structure with fields, that define
//! parameters and state.
//!
//! Every indicator implements [`Next`](traits::Next) and
//! [`Reset`](traits::Reset) traits, which are the core concept of the library.
//!
//! Since `Next` is a generic trait, most of the indicators can work with both
//! input types: [`f64`] and more complex structures like [`DataItem`].
//!
//! # Example
//! ```
//! use ta::indicators::ExponentialMovingAverage;
//! use ta::Next;
//!
//! // it can return an error, when an invalid period is passed (e.g. 0)
//! let mut ema = ExponentialMovingAverage::new(3).unwrap();
//!
//! assert_eq!(ema.next(2.0), 2.0);
//! assert_eq!(ema.next(5.0), 3.5);
//! assert_eq!(ema.next(1.0), 2.25);
//! assert_eq!(ema.next(6.25), 4.25);
//! ```
//!
//! # List of Indicators
//!
//! * Trends
//!   * [Exponential Moving Average (EMA)](indicators::ExponentialMovingAverage)
//!   * [Hull Moving Average (HMA)](indicators::HullMovingAverage)
//!   * [Weighted Moving Average (WMA)](indicators::WeightedMovingAverage)
//!   * [Simple Moving Average (SMA)](indicators::SimpleMovingAverage)
//! * Oscillators
//!   * [Relative Strength Index (RSI)](indicators::RelativeStrengthIndex)
//!   * [Fast Stochastic](indicators::FastStochastic)
//!   * [Slow Stochastic](indicators::SlowStochastic)
//!   * [Moving Average Convergence Divergence
//!     (MACD)](indicators::MovingAverageConvergenceDivergence)
//!   * [Percentage Price Oscillator
//!     (PPO)](indicators::PercentagePriceOscillator)
//!   * [Commodity Channel Index (CCI)](indicators::CommodityChannelIndex)
//!   * [Money Flow Index (MFI)](indicators::MoneyFlowIndex)
//! * Others
//!   * [Standard Deviation (SD)](indicators::StandardDeviation)
//!   * [Mean Absolute Deviation (MAD)](indicators::MeanAbsoluteDeviation)
//!   * [Bollinger Bands (BB)](indicators::BollingerBands)
//!   * [Chandelier Exit (CE)](indicators::ChandelierExit)
//!   * [Keltner Channel (KC)](indicators::KeltnerChannel)
//!   * [Maximum](indicators::Maximum)
//!   * [Minimum](indicators::Minimum)
//!   * [True Range (TR)](indicators::TrueRange)
//!   * [Average True Range (ATR)](indicators::AverageTrueRange)
//!   * [Efficiency Ratio (ER)](indicators::EfficiencyRatio)
//!   * [Rate of Change (ROC)](indicators::RateOfChange)
//!   * [On Balance Volume (OBV)](indicators::OnBalanceVolume)
//!   * [Quantitative Qualitative Estimation
//!     (QQE)](indicators::QuantitativeQualitativeEstimation)
#[cfg(test)]
#[macro_use]
mod test_helper;

mod helpers;

pub mod errors;
pub mod indicators;

mod traits;
pub use traits::*;

mod data_item;
pub use data_item::{DataItem, DataItemBuilder};
