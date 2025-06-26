use eframe::egui;

pub struct OutputPanel {
    output_directory: String,
    compression_progress: f32,
    is_processing: bool,
    results: Vec<CompressionResult>,
}

#[derive(Clone)]
pub struct CompressionResult {
    pub filename: String,
    pub original_size: u64,
    pub compressed_size: u64,
    pub compression_ratio: f32,
}

impl OutputPanel {
    pub fn new() -> Self {
        Self {
            output_directory: "output/".to_string(),
            compression_progress: 0.0,
            is_processing: false,
            results: Vec::new(),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label("Output Settings:");
            
            ui.horizontal(|ui| {
                ui.label("Directory:");
                ui.text_edit_singleline(&mut self.output_directory);
                if ui.button("Browse").clicked() {
                    // Directory dialog will be implemented later
                }
            });
            
            if self.is_processing {
                ui.label("Processing...");
                ui.add(egui::ProgressBar::new(self.compression_progress));
            }
            
            if !self.results.is_empty() {
                ui.separator();
                ui.label("Results:");
                for result in &self.results {
                    ui.horizontal(|ui| {
                        ui.label(&result.filename);
                        ui.label(format!("{:.1}% reduction", result.compression_ratio * 100.0));
                    });
                }
            }
        });
    }
}
