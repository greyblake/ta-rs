extern crate ta;
extern crate csv;

//use ta::DataItem;
//use ta::indicators::SimpleMovingAverage as Sma;

#[test]
fn test_sma() {
    //let quotes = load_quotes();
    //println!("quotes = {:?}", quotes);

    //let nums = load_nums();
    //println!("nums = {:?}", nums);

    //let points = load_points();
    //println!("points = {:?}", points);
}

//fn load_quotes() -> Vec<DataItem> {
//    let mut output: Vec<DataItem> = vec![];

//    let mut reader = csv::Reader::from_file("./tests/AMZN.csv").unwrap();

//    for record in reader.decode() {
//        let (_date, open, high, low, close, volume): (String, f64, f64, f64, f64, f64) = record.unwrap();

//        let dt = DataItem::builder().
//            open(open).
//            high(high).
//            low(low).
//            close(close).
//            volume(volume).
//            build().
//            unwrap();

//        //println!("dt = {:?}", dt);
//        output.push(dt);
//    }
//    output
//}
