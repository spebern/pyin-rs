mod core;
mod fft;
mod hmm;
mod pyin;
mod yin;

use std::ops::Range;

pub use pyin::Pyin;
pub use yin::Yin;

pub trait PitchDetector {
    /**
     * Find the most significant fundamental pitch in the specified audio buffer, optionally
     * within a given frequency range.
     * Return 0.0 if no pitch is detected.
     */
    fn pitch(&mut self, audio_buffer: &[f64], frequency_range: Option<Range<f64>>) -> f64;
}
