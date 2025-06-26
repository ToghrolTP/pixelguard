use eframe::egui;

pub struct CompressionPanel {
    compression_level: f32,
    format: CompressionFormat,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CompressionFormat {
    Png,
    WebP,
    Avif,
}

impl CompressionPanel {
    pub fn new() -> Self {
        Self {
            compression_level: 6.0,
            format: CompressionFormat::Png,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label("Compression Settings:");
            
            ui.horizontal(|ui| {
                ui.label("Format:");
                egui::ComboBox::from_id_source("format")
                    .selected_text(format!("{:?}", self.format))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.format, CompressionFormat::Png, "PNG");
                        ui.selectable_value(&mut self.format, CompressionFormat::WebP, "WebP");
                        ui.selectable_value(&mut self.format, CompressionFormat::Avif, "AVIF");
                    });
            });
            
            ui.horizontal(|ui| {
                ui.label("Level:");
                ui.add(egui::Slider::new(&mut self.compression_level, 1.0..=9.0));
            });
            
            if ui.button("Compress Images").clicked() {
                // Compression logic will be implemented later
            }
        });
    }
}
