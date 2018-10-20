
//! Filter Module

pub trait NoiseFilter {
    fn next(&mut self, value: f64) -> f64;
}

#[derive(Debug, Clone)]
pub struct MovAvgFilter {
    values: [f64; 10],
    index: usize,
}

const FILTER_N: usize = 10;

impl MovAvgFilter {
    fn new() -> Self {
        MovAvgFilter {
            values: [0f64; FILTER_N],
            index: 0,
        }
    }
}

impl NoiseFilter for MovAvgFilter {
    fn next(&mut self, value: f64) -> f64 {
        if self.index == FILTER_N {
            self.index = 0;
        }

        self.values[self.index] = value;

        let mut sum = 0f64;
        for sum_index in 0..FILTER_N {
            sum += self.values[sum_index];
        }
        sum / FILTER_N as f64
    }
}

