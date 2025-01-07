use rstest::*;
use utils::{run_pyin, run_yin};

mod utils;

const FREQUENCIES: [f64; 12] = [
    77.0, 83.0, 100.0, 150.0, 233.0, 250.0, 298.0, 350.0, 1337.0, 1583.0, 3398.0, 4200.0,
];

const DATUM: [&str; 12] = [
    include_str!("./samples/sine_77_0.txt"),
    include_str!("./samples/sine_83_0.txt"),
    include_str!("./samples/sine_100_0.txt"),
    include_str!("./samples/sine_150_0.txt"),
    include_str!("./samples/sine_233_0.txt"),
    include_str!("./samples/sine_250_0.txt"),
    include_str!("./samples/sine_298_0.txt"),
    include_str!("./samples/sine_350_0.txt"),
    include_str!("./samples/sine_1337_0.txt"),
    include_str!("./samples/sine_1583_0.txt"),
    include_str!("./samples/sine_3398_0.txt"),
    include_str!("./samples/sine_4200_0.txt"),
];

#[rstest]
#[case(0)]
#[case(1)]
#[case(2)]
//#[case(3)]  // fails
#[case(4)]
#[case(5)]
#[case(6)]
#[case(7)]
#[case(8)]
#[case(9)]
#[case(10)]
#[case(11)]
fn test_yin_sine_waves(#[case] index: usize) {
    let frequency = FREQUENCIES[index];
    let data = DATUM[index];
    let pitch = run_yin(data, 48000, 0.15, None);

    let tolerance = if frequency <= 100.0 { 0.05 } else { 0.01 };

    approx::assert_relative_eq!(pitch, frequency, epsilon = tolerance * frequency);
}

#[rstest]
#[case(0)]
#[case(1)]
#[case(2)]
#[case(3)]
#[case(4)]
#[case(5)]
#[case(6)]
#[case(7)]
#[case(8)]
#[case(9)]
#[case(10)]
#[case(11)]
fn test_pyin_sine_waves(#[case] index: usize) {
    let frequency = FREQUENCIES[index];
    let data = DATUM[index];

    let pitch = run_pyin(data, 48000, None);

    let tolerance = if frequency <= 100.0 { 0.05 } else { 0.01 };

    approx::assert_relative_eq!(pitch, frequency, epsilon = tolerance * frequency);
}
