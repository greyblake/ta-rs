use crate::errors::*;
use crate::NumberType;
use crate::{lit, Close, High, Low, Open, Volume};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Data item is used as an input for indicators.
///
/// # Example
///
/// ```
/// use ta::DataItem;
/// use ta::{Open, High, Low, Close, Volume};
///
/// let item = DataItem::builder()
///     .open(20.0)
///     .high(25.0)
///     .low(15.0)
///     .close(21.0)
///     .volume(7500.0)
///     .build()
///     .unwrap();
///
/// assert_eq!(item.open(), 20.0);
/// assert_eq!(item.high(), 25.0);
/// assert_eq!(item.low(), 15.0);
/// assert_eq!(item.close(), 21.0);
/// assert_eq!(item.volume(), 7500.0);
/// ```
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct DataItem {
    open: NumberType,
    high: NumberType,
    low: NumberType,
    close: NumberType,
    volume: NumberType,
}

impl DataItem {
    pub fn builder() -> DataItemBuilder {
        DataItemBuilder::new()
    }
}

impl Open for DataItem {
    fn open(&self) -> NumberType {
        self.open
    }
}

impl High for DataItem {
    fn high(&self) -> NumberType {
        self.high
    }
}

impl Low for DataItem {
    fn low(&self) -> NumberType {
        self.low
    }
}

impl Close for DataItem {
    fn close(&self) -> NumberType {
        self.close
    }
}

impl Volume for DataItem {
    fn volume(&self) -> NumberType {
        self.volume
    }
}

#[derive(Default)]
pub struct DataItemBuilder {
    open: Option<NumberType>,
    high: Option<NumberType>,
    low: Option<NumberType>,
    close: Option<NumberType>,
    volume: Option<NumberType>,
}

impl DataItemBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(mut self, val: NumberType) -> Self {
        self.open = Some(val);
        self
    }

    pub fn high(mut self, val: NumberType) -> Self {
        self.high = Some(val);
        self
    }

    pub fn low(mut self, val: NumberType) -> Self {
        self.low = Some(val);
        self
    }

    pub fn close(mut self, val: NumberType) -> Self {
        self.close = Some(val);
        self
    }

    pub fn volume(mut self, val: NumberType) -> Self {
        self.volume = Some(val);
        self
    }

    pub fn build(self) -> Result<DataItem> {
        if let (Some(open), Some(high), Some(low), Some(close), Some(volume)) =
            (self.open, self.high, self.low, self.close, self.volume)
        {
            // validate
            if low <= open
                && low <= close
                && low <= high
                && high >= open
                && high >= close
                && volume >= lit!(0.0)
            {
                let item = DataItem {
                    open,
                    high,
                    low,
                    close,
                    volume,
                };
                Ok(item)
            } else {
                Err(TaError::DataItemInvalid)
            }
        } else {
            Err(TaError::DataItemIncomplete)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        fn assert_valid(
            (open, high, low, close, volume): (
                NumberType,
                NumberType,
                NumberType,
                NumberType,
                NumberType,
            ),
        ) {
            let result = DataItem::builder()
                .open(open)
                .high(high)
                .low(low)
                .close(close)
                .volume(volume)
                .build();
            assert!(result.is_ok());
        }

        fn assert_invalid(
            (open, high, low, close, volume): (
                NumberType,
                NumberType,
                NumberType,
                NumberType,
                NumberType,
            ),
        ) {
            let result = DataItem::builder()
                .open(open)
                .high(high)
                .low(low)
                .close(close)
                .volume(volume)
                .build();
            assert_eq!(result, Err(TaError::DataItemInvalid));
        }

        let valid_records = vec![
            // open, high, low , close, volume
            (lit!(20.0), lit!(25.0), lit!(15.0), lit!(21.0), lit!(7500.0)),
            (lit!(10.0), lit!(10.0), lit!(10.0), lit!(10.0), lit!(10.0)),
            (lit!(0.0), lit!(0.0), lit!(0.0), lit!(0.0), lit!(0.0)),
        ];
        for record in valid_records {
            assert_valid(record)
        }

        let invalid_records = vec![
            // open, high, low , close, volume
            (lit!(-1.0), lit!(25.0), lit!(15.0), lit!(21.0), lit!(7500.0)),
            (lit!(20.0), lit!(-1.0), lit!(15.0), lit!(21.0), lit!(7500.0)),
            (lit!(20.0), lit!(25.0), lit!(15.0), lit!(-1.0), lit!(7500.0)),
            (lit!(20.0), lit!(25.0), lit!(15.0), lit!(21.0), lit!(-1.0)),
            (lit!(14.9), lit!(25.0), lit!(15.0), lit!(21.0), lit!(7500.0)),
            (lit!(25.1), lit!(25.0), lit!(15.0), lit!(21.0), lit!(7500.0)),
            (lit!(20.0), lit!(25.0), lit!(15.0), lit!(14.9), lit!(7500.0)),
            (lit!(20.0), lit!(25.0), lit!(15.0), lit!(25.1), lit!(7500.0)),
            (lit!(20.0), lit!(15.0), lit!(25.0), lit!(21.0), lit!(7500.0)),
        ];
        for record in invalid_records {
            assert_invalid(record)
        }
    }
}
