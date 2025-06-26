use eframe::egui;

pub struct FileInput {
    selected_files: Vec<String>,
}

impl FileInput {
    pub fn new() -> Self {
        Self {
            selected_files: Vec::new(),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label("Select Images:");
            
            if ui.button("Browse Files").clicked() {
                // File dialog will be implemented later
                self.selected_files.push("placeholder.jpg".to_string());
            }
            
            if !self.selected_files.is_empty() {
                ui.label(format!("Selected files: {}", self.selected_files.len()));
                for file in &self.selected_files {
                    ui.label(format!("• {}", file));
                }
            }
        });
    }
}
