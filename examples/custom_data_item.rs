use ta::indicators::TrueRange;
use ta::{Close, High, Low, Next};

#[cfg(feature = "decimal")]
type Num = rust_decimal::Decimal;
#[cfg(not(feature = "decimal"))]
type Num = f64;

// You can create your own data items.
// You may want it for different purposes, e.g.:
// - you data source don't have volume or other fields.
// - you want to skip validation to avoid performance penalty.
struct Item {
    high: Num,
    low: Num,
    close: Num,
}

impl Low for Item {
    fn low(&self) -> Num {
        self.low
    }
}

impl High for Item {
    fn high(&self) -> Num {
        self.high
    }
}

impl Close for Item {
    fn close(&self) -> Num {
        self.close
    }
}

fn main() {
    let mut tr = TrueRange::default();
    let mut reader = csv::Reader::from_path("./examples/data/AMZN.csv").unwrap();

    for record in reader.deserialize() {
        let (date, _open, high, low, close, _volume): (String, Num, Num, Num, Num, Num) =
            record.unwrap();
        let item = Item { high, low, close };
        let val = tr.next(&item);
        println!("{date}: {tr} = {val:2.2}");
    }
}
