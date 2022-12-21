use ta::indicators::ExponentialMovingAverage as Ema;
use ta::DataItem;
use ta::Next;

#[cfg(feature = "decimal")]
type Num = rust_decimal::Decimal;
#[cfg(not(feature = "decimal"))]
type Num = f64;

fn main() {
    let mut ema = Ema::new(9).unwrap();
    let mut reader = csv::Reader::from_path("./examples/data/AMZN.csv").unwrap();

    for record in reader.deserialize() {
        let (date, open, high, low, close, volume): (String, Num, Num, Num, Num, Num) =
            record.unwrap();
        let dt = DataItem::builder()
            .open(open)
            .high(high)
            .low(low)
            .close(close)
            .volume(volume)
            .build()
            .unwrap();
        let ema_val = ema.next(&dt);
        println!("{}: {} = {:2.2}", date, ema, ema_val);
    }
}
