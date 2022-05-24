// Indicator traits

#[cfg(not(feature = "rust_decimal"))]
pub(crate) type NumberType = f64;
#[cfg(feature = "rust_decimal")]
pub(crate) type NumberType = rust_decimal::Decimal;

/// Resets an indicator to the initial state.
pub trait Reset {
    fn reset(&mut self);
}

/// Return the period used by the indicator.
pub trait Period {
    fn period(&self) -> usize;
}

/// Consumes a data item of type `T` and returns `Output`.
///
/// Typically `T` can be `f64` or a struct similar to [DataItem](struct.DataItem.html), that implements
/// traits necessary to calculate value of a particular indicator.
///
/// In most cases `Output` is `f64`, but sometimes it can be different. For example for
/// [MACD](indicators/struct.MovingAverageConvergenceDivergence.html) it is `(f64, f64, f64)` since
/// MACD returns 3 values.
///
pub trait Next<T> {
    type Output;
    fn next(&mut self, input: T) -> Self::Output;
}

/// Open price of a particular period.
pub trait Open {
    fn open(&self) -> NumberType;
}

/// Close price of a particular period.
pub trait Close {
    fn close(&self) -> NumberType;
}

/// Lowest price of a particular period.
pub trait Low {
    fn low(&self) -> NumberType;
}

/// Highest price of a particular period.
pub trait High {
    fn high(&self) -> NumberType;
}

/// Trading volume of a particular trading period.
pub trait Volume {
    fn volume(&self) -> NumberType;
}
