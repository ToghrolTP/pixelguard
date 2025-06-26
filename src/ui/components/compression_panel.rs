use crate::compression::{CompressionSettings, OutputFormat, CompressionEngine, CompressionProgress, CompressionResult};
use crate::file::ImageFile;
use eframe::egui;
use std::sync::mpsc;

pub struct CompressionPanel {
    settings: CompressionSettings,
    progress_receiver: Option<mpsc::Receiver<CompressionProgress>>,
    result_sender: Option<mpsc::Sender<Vec<CompressionResult>>>,
    current_progress: f32,
    is_processing: bool,
    status_message: String,
}

impl CompressionPanel {
    pub fn new() -> Self {
        Self {
            settings: CompressionSettings::default(),
            progress_receiver: None,
            result_sender: None,
            current_progress: 0.0,
            is_processing: false,
            status_message: String::new(),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, files: &[ImageFile]) {
        self.update_progress();

        ui.group(|ui| {
            ui.label("Compression Settings:");
            
            self.render_format_selector(ui);
            self.render_quality_settings(ui);
            self.render_output_settings(ui);
            
            ui.separator();
            
            if self.is_processing {
                self.render_progress(ui);
            } else {
                self.render_compress_button(ui, files);
            }
        });
    }

    pub fn set_result_sender(&mut self, sender: mpsc::Sender<Vec<CompressionResult>>) {
        self.result_sender = Some(sender);
    }

    fn render_format_selector(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Format:");
            egui::ComboBox::from_id_source("format")
                .selected_text(format!("{:?}", self.settings.output_format))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.settings.output_format, OutputFormat::Png, "PNG (Lossless)");
                    ui.selectable_value(&mut self.settings.output_format, OutputFormat::WebP, "WebP");
                    ui.selectable_value(&mut self.settings.output_format, OutputFormat::Jpeg, "JPEG");
                });
        });
    }

    fn render_quality_settings(&mut self, ui: &mut egui::Ui) {
        match self.settings.output_format {
            OutputFormat::Png => {
                ui.horizontal(|ui| {
                    ui.label("Level:");
                    ui.add(egui::Slider::new(&mut self.settings.png_level, 1..=6));
                });
            }
            OutputFormat::WebP => {
                ui.horizontal(|ui| {
                    ui.label("Quality:");
                    ui.add(egui::Slider::new(&mut self.settings.webp_quality, 0.0..=100.0));
                });
            }
            OutputFormat::Jpeg => {
                ui.horizontal(|ui| {
                    ui.label("Quality:");
                    ui.add(egui::Slider::new(&mut self.settings.jpeg_quality, 1..=100));
                });
            }
        }
    }

    fn render_output_settings(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Output:");
            ui.text_edit_singleline(&mut self.settings.output_directory);
            if ui.button("Browse").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.settings.output_directory = path.to_string_lossy().to_string();
                }
            }
        });
        
        ui.checkbox(&mut self.settings.preserve_metadata, "Preserve metadata");
    }

    fn render_compress_button(&mut self, ui: &mut egui::Ui, files: &[ImageFile]) {
        let can_compress = !files.is_empty() && self.settings.validate().is_ok();
        
        ui.add_enabled_ui(can_compress, |ui| {
            if ui.button("Compress Images").clicked() {
                self.start_compression(files.to_vec());
            }
        });

        if !can_compress && !files.is_empty() {
            if let Err(error) = self.settings.validate() {
                ui.colored_label(egui::Color32::RED, error);
            }
        }
    }

    fn render_progress(&self, ui: &mut egui::Ui) {
        ui.label("Processing...");
        ui.add(egui::ProgressBar::new(self.current_progress).show_percentage());
        if !self.status_message.is_empty() {
            ui.label(&self.status_message);
        }
    }

    fn start_compression(&mut self, files: Vec<ImageFile>) {
        let (progress_sender, progress_receiver) = mpsc::channel();
        self.progress_receiver = Some(progress_receiver);
        self.is_processing = true;
        self.current_progress = 0.0;
        self.status_message = "Starting compression...".to_string();

        let settings = self.settings.clone();
        let result_sender = self.result_sender.clone();
        
        std::thread::spawn(move || {
            let results = CompressionEngine::compress_files(files, settings, progress_sender);
            if let Some(sender) = result_sender {
                let _ = sender.send(results);
            }
        });
    }

    fn update_progress(&mut self) {
        let mut should_clear_receiver = false;
        
        if let Some(receiver) = &self.progress_receiver {
            while let Ok(progress) = receiver.try_recv() {
                match progress {
                    CompressionProgress::Processing { current, total, filename } => {
                        self.status_message = format!("Processing {} ({}/{})", filename, current, total);
                    }
                    CompressionProgress::Progress(value) => {
                        self.current_progress = value;
                    }
                    CompressionProgress::Complete => {
                        self.is_processing = false;
                        should_clear_receiver = true;
                        self.status_message = "Compression completed!".to_string();
                    }
                    CompressionProgress::Error(error) => {
                        self.is_processing = false;
                        should_clear_receiver = true;
                        self.status_message = format!("Error: {}", error);
                    }
                }
            }
        }
        
        if should_clear_receiver {
            self.progress_receiver = None;
        }
    }
}
