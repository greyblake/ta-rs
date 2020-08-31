#[macro_use]
extern crate bencher;
extern crate ta;

use bencher::Bencher;
use rand::Rng;
use ta::indicators::{
    BollingerBands, EfficiencyRatio, ExponentialMovingAverage, FastStochastic, KeltnerChannel,
    Maximum, Minimum, MoneyFlowIndex, MovingAverageConvergenceDivergence, OnBalanceVolume,
    RateOfChange, RelativeStrengthIndex, SimpleMovingAverage, SlowStochastic, StandardDeviation,
    TrueRange,
};
use ta::DataItem;
use ta::Next;

const ITEMS_COUNT: usize = 5_000;

fn rand_data_item() -> DataItem {
    let mut rng = rand::thread_rng();

    let low = rng.gen_range(0.0, 500.0);
    let high = rng.gen_range(500.0, 1000.0);
    let open = rng.gen_range(low, high);
    let close = rng.gen_range(low, high);
    let volume = rng.gen_range(0.0, 10_000.0);

    DataItem::builder()
        .open(open)
        .high(high)
        .low(low)
        .close(close)
        .volume(volume)
        .build()
        .unwrap()
}

macro_rules! bench_indicators {
    ($($indicator:ident), *) => {
        $(
            fn $indicator(bench: &mut Bencher) {
                let items: Vec<DataItem> = (0..ITEMS_COUNT).map( |_| rand_data_item() ).collect();
                let mut indicator = $indicator::default();

                bench.iter(|| {
                    for item in items.iter() {
                        indicator.next(item);
                    }
                })
            }
        )*

        benchmark_group!(benches, $($indicator,)*);
        benchmark_main!(benches);
    }
}

bench_indicators!(
    SimpleMovingAverage,
    ExponentialMovingAverage,
    StandardDeviation,
    BollingerBands,
    KeltnerChannel,
    EfficiencyRatio,
    FastStochastic,
    Maximum,
    Minimum,
    MovingAverageConvergenceDivergence,
    RateOfChange,
    RelativeStrengthIndex,
    SlowStochastic,
    TrueRange,
    MoneyFlowIndex,
    OnBalanceVolume
);
