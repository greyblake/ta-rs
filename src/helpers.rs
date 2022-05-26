#[cfg(not(feature = "rust_decimal"))]
mod generics {
    pub(crate) type NumberType = f64;

    #[macro_export]
    macro_rules! lit {
        ($e:expr) => {
            $e
        };
    }

    #[macro_export]
    macro_rules! int {
        ($e:expr) => {
            $e as f64
        };
    }

    pub use std::f64::INFINITY;
}

#[cfg(feature = "rust_decimal")]
mod generics {
    pub(crate) type NumberType = rust_decimal::Decimal;

    #[macro_export]
    macro_rules! lit {
        ($e:expr) => {
            ::rust_decimal::Decimal::from_str_exact(stringify!($e)).unwrap()
        };
    }

    #[macro_export]
    macro_rules! int {
        ($e:expr) => {
            ::rust_decimal::Decimal::new($e.try_into().unwrap(), 0)
        };
    }

    use rust_decimal::Decimal;
    pub const INFINITY: Decimal = Decimal::MAX;
}

pub(crate) use generics::*;

/// Returns the largest of 3 given numbers.
pub fn max3(a: f64, b: f64, c: f64) -> f64 {
    a.max(b).max(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max3() {
        assert_eq!(max3(3.0, 2.0, 1.0), 3.0);
        assert_eq!(max3(2.0, 3.0, 1.0), 3.0);
        assert_eq!(max3(2.0, 1.0, 3.0), 3.0);
    }
}
