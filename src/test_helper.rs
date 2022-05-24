use super::{Close, High, Low, NumberType, Open, Volume};

#[derive(Debug, PartialEq)]
pub struct Bar {
    open: NumberType,
    high: NumberType,
    low: NumberType,
    close: NumberType,
    volume: NumberType,
}

impl Bar {
    pub fn new() -> Self {
        Self {
            open: 0.0,
            close: 0.0,
            low: 0.0,
            high: 0.0,
            volume: 0.0,
        }
    }

    //pub fn open<T: Into<NumberType>>(mut self, val :T ) -> Self {
    //    self.open = val.into();
    //    self
    //}

    pub fn high<T: Into<NumberType>>(mut self, val: T) -> Self {
        self.high = val.into();
        self
    }

    pub fn low<T: Into<NumberType>>(mut self, val: T) -> Self {
        self.low = val.into();
        self
    }

    pub fn close<T: Into<NumberType>>(mut self, val: T) -> Self {
        self.close = val.into();
        self
    }

    pub fn volume<T: Into<NumberType>>(mut self, val: T) -> Self {
        self.volume = val.into();
        self
    }
}

impl Open for Bar {
    fn open(&self) -> NumberType {
        self.open
    }
}

impl Close for Bar {
    fn close(&self) -> NumberType {
        self.close
    }
}

impl Low for Bar {
    fn low(&self) -> NumberType {
        self.low
    }
}

impl High for Bar {
    fn high(&self) -> NumberType {
        self.high
    }
}

impl Volume for Bar {
    fn volume(&self) -> NumberType {
        self.volume
    }
}

#[cfg(not(feature = "rust_decimal"))]
pub fn round(num: NumberType) -> NumberType {
    (num * 1000.0).round() / 1000.00
}

#[cfg(feature = "rust_decimal")]
pub fn round(num: NumberType) -> NumberType {
    num.round_dp(3)
}

macro_rules! test_indicator {
    ($i:tt) => {
        #[test]
        fn test_indicator() {
            let bar = Bar::new();

            // ensure Default trait is implemented
            let mut indicator = $i::default();

            // ensure Next<NumberType> is implemented
            let first_output = indicator.next(12.3);

            // ensure next accepts &DataItem as well
            indicator.next(&bar);

            // ensure Reset is implemented and works correctly
            indicator.reset();
            assert_eq!(indicator.next(12.3), first_output);

            // ensure Display is implemented
            format!("{}", indicator);
        }
    };
}
