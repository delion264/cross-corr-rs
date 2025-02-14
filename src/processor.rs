use num_complex::Complex;
use rustfft::{Fft, FftPlanner};

pub struct Processor {
    fft_len: usize,
    fft_planner: FftPlanner<i16>,
}
enum Alignment {
    Left,
    Right,
}

impl Processor {
    fn create_fft_planner(&mut self) {
        self.fft_planner = FftPlanner::new();
    }

    fn new(&mut self, _fft_size: usize) -> &Processor {
        self.fft_len = 2048;
        self.create_fft_planner();
        self
    }

    fn pad(
        &self,
        signal: &mut Vec<Complex<i16>>,
        pad_alignment: Alignment,
        pad_value: Complex<i16>,
    ) {
        match pad_alignment {
            Alignment::Left => signal.resize(signal.len() * 2, pad_value),
            Alignment::Right => {
                for i in 0..signal.len() {
                    signal.insert(0, pad_value);
                }
            }
        }
    }

    pub fn cross_corr(
        &mut self,
        signal_a: &mut Vec<Complex<i16>>,
        signal_b: &mut Vec<Complex<i16>>,
    ) {
        self.create_fft_planner();
        let fft_fwd = self.fft_planner.plan_fft_forward(self.fft_len);
        let fft_inv = self.fft_planner.plan_fft_inverse(self.fft_len * 2);

        self.pad(signal_a, Alignment::Left, Complex { re: (0), im: (0) });
        self.pad(signal_b, Alignment::Right, Complex { re: (0), im: (0) });

        fft_fwd.process(signal_a);
        fft_fwd.process(signal_b);

        let mut xcorr_coeffs: Vec<Complex<i16>> = Vec::new();
        for i in 0..(self.fft_len * 2 - 1) {
            xcorr_coeffs.push(signal_a[i] * (signal_b[i].conj()));
        }

        fft_inv.process(&mut xcorr_coeffs);
    }
}
