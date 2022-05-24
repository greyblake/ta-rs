use crate::errors::*;
use crate::traits::{Close, High, Low, NumberType, Open, Volume};

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
#[derive(Debug, Clone)]
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

pub struct DataItemBuilder {
    open: Option<NumberType>,
    high: Option<NumberType>,
    low: Option<NumberType>,
    close: Option<NumberType>,
    volume: Option<NumberType>,
}

impl DataItemBuilder {
    pub fn new() -> Self {
        Self {
            open: None,
            high: None,
            low: None,
            close: None,
            volume: None,
        }
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
                && volume >= 0.0
                && low >= 0.0
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
            assert!(result.is_err());
        }

        let valid_records = vec![
            // open, high, low , close, volume
            (20.0, 25.0, 15.0, 21.0, 7500.0),
            (10.0, 10.0, 10.0, 10.0, 10.0),
            (0.0, 0.0, 0.0, 0.0, 0.0),
        ];
        for record in valid_records {
            assert_valid(record)
        }

        let invalid_records = vec![
            // open, high, low , close, volume
            (-1.0, 25.0, 15.0, 21.0, 7500.0),
            (20.0, -1.0, 15.0, 21.0, 7500.0),
            (20.0, 25.0, -1.0, 21.0, 7500.0),
            (20.0, 25.0, 15.0, -1.0, 7500.0),
            (20.0, 25.0, 15.0, 21.0, -1.0),
            (14.9, 25.0, 15.0, 21.0, 7500.0),
            (25.1, 25.0, 15.0, 21.0, 7500.0),
            (20.0, 25.0, 15.0, 14.9, 7500.0),
            (20.0, 25.0, 15.0, 25.1, 7500.0),
            (20.0, 15.0, 25.0, 21.0, 7500.0),
        ];
        for record in invalid_records {
            assert_invalid(record)
        }
    }
}
