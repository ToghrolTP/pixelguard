use crate::ui::components::*;
use eframe::egui;

pub struct UiState {
    header: Header,
    file_input: FileInput,
    compression_panel: CompressionPanel,
    output_panel: OutputPanel,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            header: Header::new(),
            file_input: FileInput::new(),
            compression_panel: CompressionPanel::new(),
            output_panel: OutputPanel::new(),
        }
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.header.render(ui);
            ui.separator();
            
            ui.vertical_centered(|ui| {
                self.file_input.render(ui);
                ui.add_space(20.0);
                
                self.compression_panel.render(ui);
                ui.add_space(20.0);
                
                self.output_panel.render(ui);
            });
        });
    }
}
