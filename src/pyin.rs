use std::{collections::HashMap, ops::Range};

use crate::{
    core::YinCore,
    hmm::{PitchCandidate, PitchHmm},
    PitchDetector,
};

const PYIN_N_THRESHOLDS: usize = 100;
const PYIN_MIN_THRESHOLD: f64 = 0.025;

// Beta distribution with mean=0.1, alpha=1 and beta=18
const BETA_DISTRIBUTION_0: [f64; 100] = [
    0.000000, 0.029069, 0.048836, 0.061422, 0.068542, 0.071571, 0.071607, 0.069516, 0.065976,
    0.061512, 0.056523, 0.051309, 0.046089, 0.041021, 0.036211, 0.031727, 0.027608, 0.023871,
    0.020517, 0.017534, 0.014903, 0.012601, 0.010601, 0.008875, 0.007393, 0.006130, 0.005059,
    0.004155, 0.003397, 0.002765, 0.002239, 0.001806, 0.001449, 0.001157, 0.000920, 0.000727,
    0.000572, 0.000448, 0.000349, 0.000271, 0.000209, 0.000160, 0.000122, 0.000092, 0.000070,
    0.000052, 0.000039, 0.000029, 0.000021, 0.000015, 0.000011, 0.000008, 0.000006, 0.000004,
    0.000003, 0.000002, 0.000001, 0.000001, 0.000001, 0.000000, 0.000000, 0.000000, 0.000000,
    0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000,
    0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000,
    0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000,
    0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000,
    0.000000,
];

/** Probabilistic YIN pitch detection: https://www.eecs.qmul.ac.uk/~simond/pub/2014/MauchDixon-PYIN-ICASSP2014.pdf */
pub struct Pyin {
    core: YinCore,
    hmm: PitchHmm,
}

impl Pyin {
    pub fn new(input_size: usize, sample_rate: usize) -> Self {
        let hmm = PitchHmm::new(0.5, None);
        let core = YinCore::new(input_size, sample_rate);

        Self { core, hmm }
    }

    fn probabilistic_threshold(&self, frequency_range: Option<Range<f64>>) -> Vec<PitchCandidate> {
        let tau_range = self.core.calculate_tau_range(frequency_range);

        // probability distribution of tau
        let mut tau_prob_dist = HashMap::new();

        for n in 0..PYIN_N_THRESHOLDS {
            let threshold = (n + 1) as f64 * PYIN_MIN_THRESHOLD;
            let tau = self.core.threshold(threshold, &tau_range);

            if tau >= 0 {
                *tau_prob_dist.entry(tau as usize).or_insert(0.0) += BETA_DISTRIBUTION_0[n];
            }
        }

        let mut pitch_candidates = Vec::new();
        for (tau, probability) in tau_prob_dist {
            let f0 = self.core.sample_rate as f64 / self.core.parabolic_interpolation(tau);

            if f0 != -0.0 {
                pitch_candidates.push(PitchCandidate::new(f0, probability));
            }
        }

        pitch_candidates
    }
}

impl PitchDetector for Pyin {
    fn pitch(&mut self, audio_buffer: &[f64], frequency_range: Option<Range<f64>>) -> f64 {
        self.core.preprocess(audio_buffer);
        let f0_estimates = self.probabilistic_threshold(frequency_range);

        self.core.fft.clear();
        self.hmm.inference(&f0_estimates)
    }
}
