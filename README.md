# Technical Analysis (ta)

[![Build Status](https://travis-ci.org/greyblake/ta-rs.svg?branch=master)](https://travis-ci.org/greyblake/ta-rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/greyblake/ta-rs/master/LICENSE)
[![Documentation](https://docs.rs/ta/badge.svg)](https://docs.rs/ta)

Technical analysis library for Rust.

## Status

The library is in development.

## Goals
* Implement flexible and easy to use library in pure Rust
* Support most popular technical analysis indicators
* Have a good documentation for every indicator

## Get started

Add to you `Cargo.toml`:
```
[dependencies]

whatlang = "*"
```

Example:

```rust
use ta::indicators::ExponentialMovingAverage;
use ta::Next;

let mut ema = ExponentialMovingAverage::new(3).unwrap();
assert_eq!(ema.next(2.0), 2.0);
assert_eq!(ema.next(5.0), 3.5);
assert_eq!(ema.next(1.0), 2.25);
assert_eq!(ema.next(6.25), 4.25);
```

## License

[MIT](https://github.com/greyblake/ta-rs/blob/master/LICENSE) © [Sergey Potapov](http://greyblake.com/)


## Contributors

- [greyblake](https://github.com/greyblake) Potapov Sergey - creator, maintainer.
