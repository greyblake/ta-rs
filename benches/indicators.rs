use bencher::{benchmark_group, benchmark_main, black_box, Bencher};
use rand::Rng;
use ta::indicators::{
    AverageTrueRange, BollingerBands, ChandelierExit, CommodityChannelIndex, EfficiencyRatio,
    ExponentialMovingAverage, FastStochastic, KeltnerChannel, Maximum, MeanAbsoluteDeviation,
    Minimum, MoneyFlowIndex, MovingAverageConvergenceDivergence, OnBalanceVolume,
    PercentagePriceOscillator, RateOfChange, RelativeStrengthIndex, SimpleMovingAverage,
    SlowStochastic, StandardDeviation, TrueRange, WeightedMovingAverage,
};
use ta::{lit, DataItem, Next};

const ITEMS_COUNT: usize = 5_000;

fn rand_data_item() -> DataItem {
    let mut rng = rand::thread_rng();

    let low = rng.gen_range(lit!(0.0)..=lit!(500.0));
    let high = rng.gen_range(lit!(500.0)..=lit!(1000.0));
    let open = rng.gen_range(low..=high);
    let close = rng.gen_range(low..=high);
    let volume = rng.gen_range(lit!(0.0)..=lit!(10_000.0));

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
            #[allow(non_snake_case)]
            fn $indicator(bench: &mut Bencher) {
                let items: Vec<DataItem> = (0..ITEMS_COUNT).map( |_| rand_data_item() ).collect();
                let mut indicator = $indicator::default();

                bench.iter(|| {
                    for item in items.iter() {
                        black_box(indicator.next(item));
                    }
                })
            }
        )*

        benchmark_group!(benches, $($indicator,)*);
        benchmark_main!(benches);
    }
}

bench_indicators!(
    AverageTrueRange,
    ExponentialMovingAverage,
    MeanAbsoluteDeviation,
    BollingerBands,
    ChandelierExit,
    EfficiencyRatio,
    FastStochastic,
    KeltnerChannel,
    Maximum,
    Minimum,
    MoneyFlowIndex,
    MovingAverageConvergenceDivergence,
    OnBalanceVolume,
    PercentagePriceOscillator,
    CommodityChannelIndex,
    RateOfChange,
    RelativeStrengthIndex,
    SimpleMovingAverage,
    SlowStochastic,
    StandardDeviation,
    TrueRange,
    WeightedMovingAverage
);
