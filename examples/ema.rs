extern crate csv;
extern crate ta;

use ta::indicators::ExponentialMovingAverage as Ema;
use ta::DataItem;
use ta::Next;

fn main() {
    let mut ema = Ema::new(9).unwrap();
    let mut reader = csv::Reader::from_file("./examples/data/AMZN.csv").unwrap();

    for record in reader.decode() {
        let (date, open, high, low, close, volume): (String, f64, f64, f64, f64, f64) =
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
