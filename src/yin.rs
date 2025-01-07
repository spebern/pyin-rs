use std::ops::Range;

use crate::{core::YinCore, PitchDetector};

/** YIN pitch detection: http://audition.ens.fr/adc/pdf/2002_JASA_YIN.pdf */
pub struct Yin {
    core: YinCore,
    threshold: f64,
}

impl Yin {
    pub fn new(input_size: usize, sample_rate: usize, threshold: f64) -> Self {
        Self {
            core: YinCore::new(input_size, sample_rate),
            threshold,
        }
    }
}

impl PitchDetector for Yin {
    fn pitch(&mut self, audio_buffer: &[f64], frequency_range: Option<Range<f64>>) -> f64 {
        self.core.preprocess(audio_buffer);

        let tau_range = self.core.calculate_tau_range(frequency_range);
        let tau_estimate = self.core.threshold(self.threshold, &tau_range);

        let result = if tau_estimate >= 0 {
            let x = self.core.parabolic_interpolation(tau_estimate as usize);
            self.core.sample_rate as f64 / x
        } else {
            0.0
        };

        self.core.fft.clear();
        result
    }
}
