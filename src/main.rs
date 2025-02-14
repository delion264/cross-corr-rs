use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoint, PlotPoints};
use num_complex::{Complex64, ComplexFloat};
use processor::Processor;

pub mod processor;

fn main() -> eframe::Result {
    env_logger::init();

    let mut corr_proc = Processor::new(2048);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 800.0]),
        ..Default::default()
    };

    let mut x: Vec<f64> = Vec::new();
    for i in 1..corr_proc.fft_len {
        x.push(i as f64);
    }

    // signalx_pt_val denotes multi-dim vectors outlining point-value or index-value representation of complex samples
    let mut signal1_pt_val: Vec<[Complex64; 2]> = Vec::new();
    let mut signal2_pt_val: Vec<[Complex64; 2]> = Vec::new();

    let mut i: f64 = 0.;
    let pi = std::f64::consts::PI;
    let complex_1: Complex64 = Complex64::cis(pi / 2.);
    let step_size: f64 = pi / 100.;

    while i < corr_proc.fft_len as f64 {
        signal1_pt_val.push([step_size * i * complex_1, Complex64::cis(pi * i / 100.)]);
        signal2_pt_val.push([
            step_size * i * complex_1,
            Complex64::cis(pi * i / 100.0) + pi / 8.,
        ]);
        i = i + 1.;
    }

    let mut signal1: Vec<Complex64> = signal1_pt_val.iter().map(|p| p[1]).collect();
    let mut signal2: Vec<Complex64> = signal2_pt_val.iter().map(|p| p[1]).collect();
    let mut xcorr_coeffs: Vec<f64> = Vec::new();
    xcorr_coeffs = corr_proc.cross_corr(&mut signal1, &mut signal2);

    let mut xcorr: Vec<[f64; 2]> = Vec::new();
    let mut i: f64 = 0.;
    while i < xcorr_coeffs.len() as f64 {
        xcorr.push([i, xcorr_coeffs[i as usize]]);
        i = i + 1.;
    }

    // Map complex buffers to egui::PlotPoint vectors for plotting, separating Re and Im components
    // signal1 PlotPoint mapping
    let signal1_re_plot: Vec<PlotPoint> = signal1_pt_val
        .iter()
        .map(|p| PlotPoint::new(Complex64::re(p[0]), Complex64::re(p[1]) as f64))
        .collect();
    let signal1_im_plot: Vec<PlotPoint> = signal1_pt_val
        .iter()
        .map(|p| PlotPoint::new(Complex64::re(p[0]), Complex64::im(p[1])))
        .collect();

    // signal2 PlotPoint mapping
    let signal2_re_plot: Vec<PlotPoint> = signal2_pt_val
        .iter()
        .map(|p| PlotPoint::new(Complex64::re(p[0]), Complex64::re(p[1]) as f64))
        .collect();
    let signal2_im_plot: Vec<PlotPoint> = signal2_pt_val
        .iter()
        .map(|p| PlotPoint::new(Complex64::re(p[0]), Complex64::im(p[1])))
        .collect();

    let xcorr_plot: Vec<PlotPoint> = xcorr.iter().map(|p| PlotPoint::new(p[0], p[1])).collect();

    eframe::run_native(
        "Cross correlation test",
        options,
        Box::new(|_cc| {
            Ok(Box::new(MyApp {
                signal1_re_plot,
                signal1_im_plot,
                signal2_re_plot,
                signal2_im_plot,
                // fft_signal1_plot,
                // fft_signal2_plot,
                xcorr_plot,
            }))
        }),
    )
}

#[derive(Default)]
struct MyApp {
    signal1_re_plot: Vec<PlotPoint>,
    signal1_im_plot: Vec<PlotPoint>,
    signal2_re_plot: Vec<PlotPoint>,
    signal2_im_plot: Vec<PlotPoint>,
    // fft_signal1_plot: Vec<PlotPoint>,
    // fft_signal2_plot: Vec<PlotPoint>,
    xcorr_plot: Vec<PlotPoint>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                Plot::new("Source signal 1")
                    .legend(Legend::default())
                    .show(ui, |plot_ui| {
                        plot_ui.line(
                            Line::new(PlotPoints::Borrowed(&self.signal1_re_plot))
                                .name("signal 1 Re"),
                        );
                        plot_ui.line(
                            Line::new(PlotPoints::Borrowed(&self.signal1_im_plot))
                                .name("signal 1 Im"),
                        );
                    });
                // Plot::new("FFT signal 1")
                //     .legend(Legend::default())
                //     .show(ui, |plot_ui| {
                //         plot_ui.line(
                //             Line::new(PlotPoints::Borrowed(&self.fft_signal1_plot))
                //                 .name("FFT signal 1"),
                //         );
                //     });
                Plot::new("Source signal 2")
                    .legend(Legend::default())
                    .show(ui, |plot_ui| {
                        plot_ui.line(
                            Line::new(PlotPoints::Borrowed(&self.signal2_re_plot))
                                .name("signal 2 Re"),
                        );
                        plot_ui.line(
                            Line::new(PlotPoints::Borrowed(&self.signal2_im_plot))
                                .name("signal 2 Im"),
                        );
                    });
                // Plot::new("FFT signal 2")
                //     .legend(Legend::default())
                //     .show(ui, |plot_ui| {
                //         plot_ui.line(
                //             Line::new(PlotPoints::Borrowed(&self.fft_signal2_plot))
                //                 .name("FFT signal 2"),
                //         );
                //     });
                Plot::new("Cross correlation of signal 1 and signal 2")
                    .legend(Legend::default())
                    .show(ui, |plot_ui| {
                        plot_ui.line(
                            Line::new(PlotPoints::Borrowed(&self.xcorr_plot))
                                .name("Cross correlation of signal 1 and signal 2"),
                        );
                    });
            });
        });
    }
}
