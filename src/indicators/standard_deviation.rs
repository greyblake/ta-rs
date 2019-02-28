use std::fmt;

use crate::error::*;
use crate::{Close, Next, Reset};

/// Standard Deviation (SD).
///
/// # Formula
///
/// ![SD](https://wikimedia.org/api/rest_v1/media/math/render/svg/9f2b2061a0bda785c7d246d21ec20a17c79185e0)
///
/// 
/// TODO: continue with documentation
///

#[derive(Debug, Clone)]
pub struct StandardDeviation {
    n: u32,
    index: usize,
    count: u32,
    mean: f64,
    vec: Vec<f64>,
}

impl StandardDeviation {
    pub fn new(n: u32) -> Reseult<Self> {
        match n {
            0 => Err(Error::from_kind(Error::InvalidParameter)),
            _ => {
                let indicator = Self {
                    n: n,
                    index: 0,
                    count: 0,
                    sum: 0.0,
                    vec: vec![0.0; n as usize],
                };
                Ok(indicator)
            },
        }
    }
}

impl Next<f64> for SimpleMovingAverage {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.index = (self.index + 1) % (self.n as usize);

        let old_val = self.vec[self.index];
        self.vec[self.index] = input;
        if self.count < self.n {
            self.count += 1;
        }
        self.sum = self.sum - old_val + input;
        let mean = self.sum / (self.count as f64);
        let mut mean_item_sum_pow = 0_f64;
        for item in self.vec.iter() {
            mean_item_sum_pow += (item - mean).powi(2);
        }
        (mean_item_sum_pow / (self.count as f64)).sqrt()
}

