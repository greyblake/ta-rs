use ta::indicators::TrueRange;
use ta::{Close, High, Low, Next};

// You can create your own data items.
// You may want it for different purposes, e.g.:
// - you data source don't have volume or other fields.
// - you want to skip validation to avoid performance penalty.
struct Item {
    high: f64,
    low: f64,
    close: f64,
}

impl Low for Item {
    fn low(&self) -> f64 {
        self.low
    }
}

impl High for Item {
    fn high(&self) -> f64 {
        self.high
    }
}

impl Close for Item {
    fn close(&self) -> f64 {
        self.close
    }
}

fn main() {
    let mut tr = TrueRange::default();
    let mut reader = csv::Reader::from_path("./examples/data/AMZN.csv").unwrap();

    for record in reader.deserialize() {
        let (date, _open, high, low, close, _volume): (String, f64, f64, f64, f64, f64) =
            record.unwrap();
        let item = Item { high, low, close };
        let val = tr.next(&item);
        println!("{date}: {tr} = {val:2.2}");
    }
}
