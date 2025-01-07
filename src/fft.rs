use std::sync::Arc;

use realfft::{num_complex::Complex64, ComplexToReal, RealFftPlanner, RealToComplex};

pub(crate) struct FftContext {
    fft_forward: Arc<dyn RealToComplex<f64>>,
    fft_inverse: Arc<dyn ComplexToReal<f64>>,
    buffer_in: Vec<f64>,
    buffer_out: Vec<Complex64>,
    scratch: Vec<Complex64>,
}

impl FftContext {
    pub fn new(size: usize) -> Self {
        if size == 0 || size & (size - 1) != 0 {
            panic!("power-of-two input size required. Got {}", size);
        }

        let mut real_planner = RealFftPlanner::<f64>::new();

        let fft_forward = real_planner.plan_fft_forward(size);
        let fft_inverse = real_planner.plan_fft_inverse(size);

        let buffer_in = fft_forward.make_input_vec();
        let buffer_out = fft_forward.make_output_vec();
        let scratch = fft_forward.make_scratch_vec();

        Self {
            fft_forward,
            fft_inverse,
            buffer_in,
            buffer_out,
            scratch,
        }
    }

    pub fn input_size(&self) -> usize {
        self.buffer_in.len()
    }

    pub fn buffer_in(&self) -> &[f64] {
        &self.buffer_in
    }

    pub fn buffer_in_mut(&mut self) -> &mut [f64] {
        &mut self.buffer_in
    }

    pub fn buffer_out_mut(&mut self) -> &mut [Complex64] {
        &mut self.buffer_out
    }

    pub fn clear(&mut self) {
        self.buffer_out
            .iter_mut()
            .for_each(|fft| *fft = Complex64::new(0.0, 0.0));
    }

    pub fn forward(&mut self) {
        self.fft_forward
            .process_with_scratch(&mut self.buffer_in, &mut self.buffer_out, &mut self.scratch)
            .unwrap();
    }

    pub fn inverse(&mut self) {
        self.fft_inverse
            .process_with_scratch(&mut self.buffer_out, &mut self.buffer_in, &mut self.scratch)
            .unwrap();
    }
}
