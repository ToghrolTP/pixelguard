use crate::compression::{CompressionResult, CompressionStatus};
use crate::file::FileManager;
use eframe::egui;

pub struct OutputPanel {
    results: Vec<CompressionResult>,
    show_details: bool,
}

impl OutputPanel {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            show_details: false,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Results:");
                if !self.results.is_empty() {
                    ui.checkbox(&mut self.show_details, "Show details");
                    if ui.button("Clear").clicked() {
                        self.results.clear();
                    }
                    if ui.button("Open folder").clicked() {
                        self.open_output_folder();
                    }
                }
            });
            
            if self.results.is_empty() {
                ui.label("No compression results yet");
            } else {
                self.render_summary(ui);
                if self.show_details {
                    self.render_detailed_results(ui);
                }
            }
        });
    }

    fn render_summary(&self, ui: &mut egui::Ui) {
        let total_original: u64 = self.results.iter().map(|r| r.original_size).sum();
        let total_compressed: u64 = self.results.iter().map(|r| r.compressed_size).sum();
        let total_saved = if total_original > total_compressed { total_original - total_compressed } else { 0 };
        let avg_ratio = if total_original > 0 { 1.0 - (total_compressed as f32 / total_original as f32) } else { 0.0 };

        ui.separator();
        ui.columns(4, |cols| {
            cols[0].label(format!("Files: {}", self.results.len()));
            cols[1].label(format!("Original: {}", FileManager::format_file_size(total_original)));
            cols[2].label(format!("Compressed: {}", FileManager::format_file_size(total_compressed)));
            cols[3].label(format!("Saved: {} ({:.1}%)", 
                FileManager::format_file_size(total_saved), 
                avg_ratio * 100.0
            ));
        });
    }

    fn render_detailed_results(&self, ui: &mut egui::Ui) {
        ui.separator();
        egui::ScrollArea::vertical()
            .max_height(300.0)
            .show(ui, |ui| {
                for result in &self.results {
                    self.render_result_row(ui, result);
                }
            });
    }

    fn render_result_row(&self, ui: &mut egui::Ui, result: &CompressionResult) {
        let filename = result.input_path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        ui.horizontal(|ui| {
            match &result.status {
                CompressionStatus::Success => {
                    ui.colored_label(egui::Color32::GREEN, "✓");
                }
                CompressionStatus::Failed(_) => {
                    ui.colored_label(egui::Color32::RED, "✗");
                }
                CompressionStatus::Skipped(_) => {
                    ui.colored_label(egui::Color32::YELLOW, "⚠");
                }
            }
            
            ui.label(&filename);
            ui.label(FileManager::format_file_size(result.original_size));
            ui.label("→");
            ui.label(FileManager::format_file_size(result.compressed_size));
            ui.colored_label(
                if result.compression_ratio > 0.0 { egui::Color32::GREEN } else { egui::Color32::RED },
                format!("{:.1}%", result.compression_ratio * 100.0)
            );
            ui.label(format!("{:.1}s", result.processing_time.as_secs_f32()));
        });

        if let CompressionStatus::Failed(error) = &result.status {
            ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
        }
    }

    fn open_output_folder(&self) {
        if let Some(result) = self.results.first() {
            if let Some(parent) = result.output_path.parent() {
                let _ = opener::open(parent);
            }
        }
    }

    pub fn add_results(&mut self, results: Vec<CompressionResult>) {
        self.results.extend(results);
    }
}
