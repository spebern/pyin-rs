use std::ops::Range;

use pyin_rs::{PitchDetector, Pyin, Yin};

pub fn load_data(data: &str) -> Vec<f32> {
    data.lines()
        .map(|line| line.parse::<f32>().unwrap())
        .collect()
}

pub fn run_yin(
    data: &str,
    sample_rate: usize,
    threshold: f32,
    frequency_range: Option<Range<f64>>,
) -> f32 {
    let samples = load_data(data);
    let pot_size = 2usize.pow(samples.len().ilog2());
    let samples = samples.into_iter().take(pot_size).collect::<Vec<_>>();

    let mut yin = Yin::new(pot_size, sample_rate, threshold);
    yin.pitch(&samples, frequency_range)
}

pub fn run_pyin(data: &str, sample_rate: usize, frequency_range: Option<Range<f64>>) -> f32 {
    let samples = load_data(data);
    let pot_size = 2usize.pow(samples.len().ilog2());
    let samples = samples.into_iter().take(pot_size).collect::<Vec<_>>();

    let mut pyin = Pyin::new(pot_size, sample_rate);
    pyin.pitch(&samples, frequency_range)
}
