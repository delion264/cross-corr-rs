use num_complex::Complex;
use rustfft::{Fft, FftPlanner};

pub struct Processor {
    fft_len: usize,
    fft_planner: FftPlanner<i16>,
}
enum alignment {
    left,
    right,
}

impl Processor {
    fn create_fft_planner(&mut self) {
        self.fft_planner = FftPlanner::new();
    }

    fn pad(
        &self,
        signal: &mut Vec<Complex<i16>>,
        pad_alignment: alignment,
        pad_value: Complex<i16>,
    ) {
        match pad_alignment {
            alignment::left => signal.resize(signal.len() * 2, pad_value),
            alignment::right => {
                for i in 0..signal.len() {
                    signal.insert(0, pad_value);
                }
            }
        }
    }

    fn cross_corr(&mut self, signal_a: &mut Vec<Complex<i16>>, signal_b: &mut Vec<Complex<i16>>) {
        self.create_fft_planner();
        let fft_fwd = self.fft_planner.plan_fft_forward(self.fft_len);
        let fft_inv = self.fft_planner.plan_fft_inverse(self.fft_len * 2);
        fft_fwd.process(signal_a);
        fft_fwd.process(signal_b);

        self.pad(signal_a, alignment::left, Complex { re: (0), im: (0) });
        self.pad(signal_b, alignment::right, Complex { re: (0), im: (0) });

        let mut xcorr_coeffs: Vec<Complex<i16>> = Vec::new();
        for i in 0..(self.fft_len * 2 - 1) {
            xcorr_coeffs.push(signal_a[i] * (signal_b[i].conj()));
        }

        fft_inv.process(&mut xcorr_coeffs);
    }
}

fn main() {}
