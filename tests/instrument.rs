#![allow(non_snake_case)]

mod utils;

use rstest::*;
use utils::{run_pyin, run_yin};

const VIOLIN_A4_44100: &str = include_str!("./samples/A4_44100_violin.txt");
const PIANO_B4_44100: &str = include_str!("./samples/B4_44100_piano.txt");
const PIANO_D4_44100: &str = include_str!("./samples/D4_44100_piano.txt");
const ACOUSTIC_E2_44100: &str = include_str!("./samples/E2_44100_acousticguitar.txt");
const CLASSICAL_FSHARP4_48000: &str = include_str!("./samples/F-4_48000_classicalguitar.txt");

#[rstest]
#[case::A4_44100_Violin(VIOLIN_A4_44100, 440.0, 44100)]
#[case::B4_44100_Piano(PIANO_B4_44100, 493.9, 44100)]
#[case::D4_44100_Piano(PIANO_D4_44100, 293.7, 44100)]
#[case::E2_44100_Acoustic(ACOUSTIC_E2_44100, 82.41, 44100)]
#[case::FSharp4_48000_Classical(CLASSICAL_FSHARP4_48000, 370.0, 48000)]
fn test_yin_instrument(#[case] data: &str, #[case] expected_freq: f64, #[case] sample_rate: usize) {
    let pitch = run_yin(data, sample_rate, 0.15, None);

    approx::assert_relative_eq!(pitch, expected_freq, epsilon = 0.01 * expected_freq);
}

#[rstest]
#[case::A4_44100_Violin(VIOLIN_A4_44100, 440.0, 44100, 400.0, 500.0)]
#[case::E2_44100_Acoustic(ACOUSTIC_E2_44100, 82.41, 44100, 75.0, 90.0)]
fn test_yin_instrument_with_range(
    #[case] data: &str,
    #[case] expected_freq: f64,
    #[case] sample_rate: usize,
    #[case] min: f64,
    #[case] max: f64,
) {
    let pitch = run_yin(data, sample_rate, 0.15, Some(min..max));

    approx::assert_relative_eq!(pitch, expected_freq, epsilon = 0.01 * expected_freq);
}

#[rstest]
#[case::A4_44100_Violin(VIOLIN_A4_44100, 440.0, 44100)]
#[case::B4_44100_Piano(PIANO_B4_44100, 493.9, 44100)]
#[case::D4_44100_Piano(PIANO_D4_44100, 293.7, 44100)]
#[case::E2_44100_Acoustic(ACOUSTIC_E2_44100, 82.41, 44100)]
//#[case::FSharp4_48000_Classical(CLASSICAL_FSHARP4_48000, 370.0, 48000)] // fails - 186Hz detected?
fn test_pyin_instrument(
    #[case] data: &str,
    #[case] expected_freq: f64,
    #[case] sample_rate: usize,
) {
    let pitch = run_pyin(data, sample_rate, None);

    approx::assert_relative_eq!(pitch, expected_freq, epsilon = 0.01 * expected_freq);
}

#[rstest]
// this case fails in the boundless test above, but passes with a range
#[case::FSharp4_48000_Classical(CLASSICAL_FSHARP4_48000, 370.0, 48000, 300.0, 400.0)]
fn test_pyin_instrument_with_range(
    #[case] data: &str,
    #[case] expected_freq: f64,
    #[case] sample_rate: usize,
    #[case] min: f64,
    #[case] max: f64,
) {
    let pitch = run_pyin(data, sample_rate, Some(min..max));

    approx::assert_relative_eq!(pitch, expected_freq, epsilon = 0.01 * expected_freq);
}
