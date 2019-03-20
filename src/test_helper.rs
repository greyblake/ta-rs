use super::{Close, High, Low, Open, Volume};

#[derive(Debug, PartialEq)]
pub struct Bar {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
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

    //pub fn open<T: Into<f64>>(mut self, val :T ) -> Self {
    //    self.open = val.into();
    //    self
    //}

    pub fn high<T: Into<f64>>(mut self, val: T) -> Self {
        self.high = val.into();
        self
    }

    pub fn low<T: Into<f64>>(mut self, val: T) -> Self {
        self.low = val.into();
        self
    }

    pub fn close<T: Into<f64>>(mut self, val: T) -> Self {
        self.close = val.into();
        self
    }

    pub fn volume(mut self, val: f64) -> Self {
        self.volume = val;
        self
    }
}

impl Open for Bar {
    fn open(&self) -> f64 {
        self.open
    }
}

impl Close for Bar {
    fn close(&self) -> f64 {
        self.close
    }
}

impl Low for Bar {
    fn low(&self) -> f64 {
        self.low
    }
}

impl High for Bar {
    fn high(&self) -> f64 {
        self.high
    }
}

impl Volume for Bar {
    fn volume(&self) -> f64 {
        self.volume
    }
}

pub fn round(num: f64) -> f64 {
    (num * 1000.0).round() / 1000.00
}

macro_rules! test_indicator {
    ($i:tt) => {
        #[test]
        fn test_indicator() {
            let bar = Bar::new();

            // ensure Default trait is implemented
            let mut indicator = $i::default();

            // ensure Next<f64> is implemented
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
