use eframe::egui;

pub struct Header;

impl Header {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        ui.heading("PixelGuard");
        ui.label("Lossless Image Compression Tool");
    }
}
