# Technical Analysis for Rust (ta)

[![Build Status](https://travis-ci.org/greyblake/ta-rs.svg?branch=master)](https://travis-ci.org/greyblake/ta-rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/greyblake/ta-rs/master/LICENSE)
[![Documentation](https://docs.rs/ta/badge.svg)](https://docs.rs/ta)

Technical analysis library for Rust.

* [Getting started](#getting-started)
* [Basic ideas](#basic-ideas)
* [List of indicators](#list-of-indicators)
* [Running benchmarks](#running-benchmarks)
* [License](#license)
* [Contributors](#contributors)

## Getting started

Add to you `Cargo.toml`:
```
[dependencies]

ta = "0.1.5"
```

Example:

```rust
use ta::indicators::ExponentialMovingAverage;
use ta::Next;

// it can return an error, when an invalid length is passed (e.g. 0)
let mut ema = ExponentialMovingAverage::new(3).unwrap();

assert_eq!(ema.next(2.0), 2.0);
assert_eq!(ema.next(5.0), 3.5);
assert_eq!(ema.next(1.0), 2.25);
assert_eq!(ema.next(6.25), 4.25);
```

See more in the examples [here](https://github.com/greyblake/ta-rs/tree/master/examples).
Check also the [documentation](https://docs.rs/ta).

## Basic ideas

A data item which represent a stock quote may implement the following traits:

* `Open`
* `High`
* `Low`
* `Close`
* `Volume`

It's not necessary to implement all of them, but it must be enough to fulfill requirements for a particular indicator.
You probably should prefer using `DataItem` unless you have reasons to implement your own structure.

Indicators typically implement the following traits:

* `Next<T>` (often `Next<f64>` and `Next<&DataItem>`) - to feed and get the next value
* `Reset` - to reset an indicator
* `Debug`
* `Display`
* `Default`
* `Clone`

## List of indicators

So far there are the following indicators available.

* Trend
  * Exponential Moving Average (EMA)
  * Simple Moving Average (SMA)
* Oscillators
  * Relative Strength Index (RSI)
  * Fast Stochastic
  * Slow Stochastic
  * Moving Average Convergence Divergence (MACD)
  * Money Flow Index (MFI)
* Other
  * Minimum
  * Maximum
  * True Range
  * Average True Range (AR)
  * Efficiency Ratio (ER)
  * Bollinger Bands (BB)
  * Keltner Change (KC)
  * Rate of Change (ROC)
  * On Balance Volume (OBV)

## Running benchmarks

```
cargo bench
```

## License

[MIT](https://github.com/greyblake/ta-rs/blob/master/LICENSE) © [Sergey Potapov](http://greyblake.com/)


## Contributors

- [greyblake](https://github.com/greyblake) Potapov Sergey - creator, maintainer.
- [Bartoshko](https://github.com/Bartoshko) - BollingerBands
- [shreyasdeotare](https://github.com/shreyasdeotare) Shreyas Deotare - MoneyFlowIndex, OnBalanceVolume
- [edwardycl](https://github.com/edwardycl) - StandardDeviation Implementation & More Efficient BollingerBands
- [rideron89](https://github.com/rideron89) Ron Rider - Keltner Channel
