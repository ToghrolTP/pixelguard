mod app;
mod ui;

use app::PixelGuardApp;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "PixelGuard - Lossless Image Compressor",
        options,
        Box::new(|_cc| Ok(Box::new(PixelGuardApp::new()))),
    )
}
