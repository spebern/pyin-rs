// hidden markov model

use bio::stats::hmm::{discrete_emission::Model as HMM, viterbi};
use std::collections::HashMap;

use ndarray::{Array1, Array2};

const DEFAULT_A4_FREQUENCY: f64 = 440.0;
const A4_SEMITONES: i32 = 57; // number of semitones from C0
const N_BINS: usize = 12 * 9; // C0 to B8

const TRANSITION_WIDTH: f64 = 13.0;
const SELF_TRANS: f64 = 0.99;

pub(crate) struct PitchCandidate {
    pub frequency: f64,
    pub probability: f64,
}

impl PitchCandidate {
    pub fn new(frequency: f64, probability: f64) -> Self {
        Self {
            frequency,
            probability,
        }
    }
}

pub(crate) struct PitchHmm {
    hmm: HMM,
    pitch_bins: [f64; N_BINS],
    yin_trust: f64,
}

impl PitchHmm {
    pub fn new(yin_trust: f64, a4_frequency: Option<f64>) -> Self {
        let hmm = Self::build_hmm();
        let pitch_bins = Self::build_bins(a4_frequency.unwrap_or(DEFAULT_A4_FREQUENCY));

        Self {
            hmm,
            pitch_bins,
            yin_trust,
        }
    }

    fn build_bins(a4_frequency: f64) -> [f64; N_BINS] {
        let mut bins = [0.0; N_BINS];
        let a: f64 = 2.0f64.powf(1.0 / 12.0);

        for i in 0..N_BINS {
            let freq = a4_frequency * a.powi(i as i32 - A4_SEMITONES);
            bins[i] = freq;
        }

        bins
    }

    fn build_hmm() -> HMM {
        let hmm_size = 2 * N_BINS;

        // Initial state probabilities
        let initial: Array1<f64> = Array1::from_elem(hmm_size, 1.0 / hmm_size as f64);

        // Transition matrix
        let mut transition = Array2::<f64>::zeros((hmm_size, hmm_size));

        for i in 0..N_BINS {
            let half_transition = (TRANSITION_WIDTH / 2.0) as usize;
            let theoretical_min_next_pitch = i as isize - half_transition as isize;
            let min_next_pitch = if i > half_transition {
                i - half_transition
            } else {
                0
            };
            let max_next_pitch = if i < N_BINS - half_transition {
                i + half_transition
            } else {
                N_BINS - 1
            };

            let mut weight_sum = 0.0;
            let mut weights = Vec::new();

            for j in min_next_pitch..=max_next_pitch {
                let weight = if j <= i {
                    j as isize - theoretical_min_next_pitch + 1
                } else {
                    (i * 2) as isize - theoretical_min_next_pitch + 1 - j as isize
                } as f64;
                weights.push(weight);
                weight_sum += weight;
            }

            for j in min_next_pitch..=max_next_pitch {
                let weight = weights[j - min_next_pitch] / weight_sum;
                transition[[i, j]] = weight * SELF_TRANS;
                transition[[i, j + N_BINS]] = weight * (1.0 - SELF_TRANS);
                transition[[i + N_BINS, j + N_BINS]] = weight * SELF_TRANS;
                transition[[i + N_BINS, j]] = weight * (1.0 - SELF_TRANS);
            }
        }

        // the only valid emissions are exact notes
        // i.e. an identity matrix of emissions
        let observation: Array2<f64> = Array2::eye(hmm_size);

        HMM::with_float(&transition, &observation, &initial).unwrap()
    }

    fn bin_pitches(&self, candidates: &[PitchCandidate]) -> (Vec<usize>, [f64; N_BINS]) {
        let mut real_pitches = [0.0; N_BINS];

        let mut pitch_probs = vec![0.0; 2 * N_BINS + 1];
        let mut possible_bins = Vec::new();

        let mut prob_pitched = 0.0;

        for PitchCandidate {
            frequency,
            probability,
        } in candidates
        {
            let mut prev_delta = f64::MAX;
            for i in 0..N_BINS {
                let delta = (frequency - self.pitch_bins[i]).abs();
                if prev_delta < delta {
                    pitch_probs[i - 1] = *probability;
                    prob_pitched += pitch_probs[i - 1];
                    real_pitches[i - 1] = *frequency;
                    break;
                }
                prev_delta = delta;
            }
        }

        let prob_really_pitched = self.yin_trust * prob_pitched;

        for i in 0..N_BINS {
            if prob_pitched > 0.0 {
                pitch_probs[i] *= prob_really_pitched / prob_pitched;
            }
            pitch_probs[i + N_BINS] = (1.0 - prob_really_pitched) / N_BINS as f64;
        }

        for (i, pitch_probability) in pitch_probs.iter().enumerate() {
            for _ in 0..(100.0 * pitch_probability).round() as usize {
                possible_bins.push(i);
            }
        }

        (possible_bins, real_pitches)
    }

    pub fn inference(&self, candidates: &[PitchCandidate]) -> f64 {
        if candidates.is_empty() {
            return -1.0;
        }

        let (possible_bins, real_pitches) = self.bin_pitches(candidates);

        if possible_bins.is_empty() {
            return -1.0;
        }

        let (states, _) = viterbi(&self.hmm, &possible_bins);

        let mut counts = HashMap::new();
        for state in states.iter() {
            let bin_index = state.0;
            if bin_index < N_BINS {
                *counts.entry(bin_index).or_insert(0) += 1;
            }
        }

        let mut most_frequent = 0;
        let mut max = 0;

        for (state, count) in counts.iter() {
            if *count > max {
                most_frequent = *state;
                max = *count;
            }
        }

        real_pitches[most_frequent]
    }
}
