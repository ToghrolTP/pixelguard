use crate::compression::CompressionResult;
use crate::ui::components::*;
use eframe::egui;
use std::sync::mpsc;

pub struct UiState {
    header: Header,
    file_input: FileInput,
    compression_panel: CompressionPanel,
    output_panel: OutputPanel,
    result_receiver: Option<mpsc::Receiver<Vec<CompressionResult>>>,
}

impl UiState {
    pub fn new() -> Self {
        let (result_sender, result_receiver) = mpsc::channel();
        let mut compression_panel = CompressionPanel::new();
        compression_panel.set_result_sender(result_sender);

        Self {
            header: Header::new(),
            file_input: FileInput::new(),
            compression_panel,
            output_panel: OutputPanel::new(),
            result_receiver: Some(result_receiver),
        }
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        self.update_results();

        egui::CentralPanel::default().show(ctx, |ui| {
            self.header.render(ui);
            ui.separator();

            ui.vertical_centered(|ui| {
                self.file_input.render(ui);
                ui.add_space(20.0);

                let files = self.file_input.get_selected_files();
                self.compression_panel.render(ui, files);
                ui.add_space(20.0);

                self.output_panel.render(ui);
            });
        });
    }

    fn update_results(&mut self) {
        if let Some(receiver) = &self.result_receiver {
            if let Ok(results) = receiver.try_recv() {
                self.output_panel.add_results(results);
            }
        }
    }
}
