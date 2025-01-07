use std::ops::Range;

use crate::fft::FftContext;

pub struct YinCore {
    pub(crate) input_size: usize,
    pub(crate) sample_rate: usize,
    pub(crate) buffer: Vec<f64>,
    pub(crate) fft: FftContext,
}

impl YinCore {
    pub fn new(input_size: usize, sample_rate: usize) -> Self {
        let buffer_size = input_size / 2;

        let fft = FftContext::new(input_size);

        Self {
            input_size,
            sample_rate,
            buffer: vec![0.0; buffer_size],
            fft,
        }
    }

    fn auto_correlate(&mut self, audio_buffer: &[f64]) {
        self.fft.buffer_in_mut().copy_from_slice(audio_buffer);
        self.fft.forward();

        let scale = 1.0 / self.fft.input_size() as f64;

        self.fft
            .buffer_out_mut()
            .iter_mut()
            .for_each(|c| *c *= c.conj() * scale);

        self.fft.inverse();
    }

    fn difference(&mut self, audio_buffer: &[f64]) {
        self.auto_correlate(audio_buffer);

        let fft = self.fft.buffer_in();
        for tau in 0..self.buffer.len() {
            self.buffer[tau] = fft[0] + fft[1] - 2.0 * fft[tau];
        }
    }

    /** Cumulative mean normalized difference. */
    fn cmnd(&mut self) {
        let mut running_sum = 0.0f64;

        self.buffer[0] = 1.0;

        for tau in 1..self.buffer.len() {
            running_sum += self.buffer[tau];
            self.buffer[tau] *= tau as f64 / running_sum;
        }
    }

    /** Take an input buffer, apply difference and CMND on it. */
    pub(crate) fn preprocess(&mut self, audio_buffer: &[f64]) {
        if audio_buffer.len() != self.input_size {
            panic!("audio buffer size mismatch");
        }

        self.difference(audio_buffer);
        self.cmnd();
    }

    /**
     * Calculate the range of tau, optionally based on a range of frequency.
     * The resulting tau range is always at least 2, and will not be larger than the buffer size.
     */
    pub(crate) fn calculate_tau_range(&self, frequency_range: Option<Range<f64>>) -> Range<usize> {
        let buffer = &self.buffer;
        let size = buffer.len();

        let (min, max) = if let Some(range) = frequency_range {
            let sample_rate = self.sample_rate as f64;
            (
                ((sample_rate / range.end).floor() as usize).max(2),
                ((sample_rate / range.start).ceil() as usize).min(size),
            )
        } else {
            (2, size)
        };

        min..max
    }

    /**
     * Apply a threshold to the buffer, find the first local minimum tau that is lower than
     * the threshold.
     * Return -1 if no such tau is found.
     */
    pub(crate) fn threshold(&self, threshold: f64, tau_range: &Range<usize>) -> isize {
        let buffer = &self.buffer;

        let mut tau = tau_range.start;
        let max = tau_range.end;

        while tau < max {
            if buffer[tau] < threshold {
                while tau + 1 < max && buffer[tau + 1] < buffer[tau] {
                    tau += 1;
                }
                break;
            }
            tau += 1;
        }
        if tau == max || buffer[tau] >= threshold {
            -1
        } else {
            tau as isize
        }
    }

    pub(crate) fn parabolic_interpolation(&self, t: usize) -> f64 {
        let b = &self.buffer;
        if t < 1 {
            if b[t] <= b[t + 1] {
                t as f64
            } else {
                (t + 1) as f64
            }
        } else if t >= b.len() - 1 {
            (t - 1) as f64
        } else {
            let den = b[t + 1] + b[t - 1] - 2.0 * b[t];
            let delta = b[t - 1] - b[t + 1];
            if den == 0.0 {
                t as f64
            } else {
                // value is b[t] - delta * delta / (8.0 * den)
                t as f64 + delta / (2.0 * den)
            }
        }
    }
}
