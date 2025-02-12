use num::{Complex, Float};
use num_complex::Complex32;
use rustfft;

enum alignment {
    left,
    right,
}
fn pad(signal: &mut Vec<Complex32>, pad_alignment: alignment, pad_value: Complex32) {
    match pad_alignment {
        alignment::left => signal.resize(signal.len() * 2, pad_value),
        alignment::right => {
            for i in 0..signal.len() {
                signal.insert(0, pad_value);
            }
        }
    }
}

fn cross_corr(signal_a: &mut Vec<Complex32>, signal_b: &mut Vec<Complex32>) -> Vec<f32> {
    // signal_b.align_left;
    pad(
        signal_a,
        alignment::left,
        Complex {
            re: (0.0),
            im: (0.0),
        },
    );

    signal_b.reverse();
    pad(
        signal_b,
        alignment::right,
        Complex {
            re: (0.0),
            im: (0.0),
        },
    );

    // 1. Set up FFT
    // 2. Compute FFT of each signal
    // 3. Conjugate multiplication of both FFTs
    // 4. Inverse FFT
    // 5. Integrate solution
}

fn main() {}
