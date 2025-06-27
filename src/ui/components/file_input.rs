use crate::file::{FileManager, ImageFile};
use eframe::egui;
use std::sync::mpsc;

pub struct FileInput {
    selected_files: Vec<ImageFile>,
    file_receiver: Option<mpsc::Receiver<Vec<ImageFile>>>,
    is_loading: bool,
}

impl FileInput {
    pub fn new() -> Self {
        Self {
            selected_files: Vec::new(),
            file_receiver: None,
            is_loading: false,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        // Check for completed file loading
        if let Some(receiver) = &self.file_receiver {
            if let Ok(files) = receiver.try_recv() {
                self.selected_files = files;
                self.file_receiver = None;
                self.is_loading = false;
            }
        }

        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Select Images:");

                if ui.button("Browse Files").clicked() && !self.is_loading {
                    self.load_files_async();
                }

                if !self.selected_files.is_empty() {
                    if ui.button("Clear").clicked() {
                        self.selected_files.clear();
                    }
                }
            });

            if self.is_loading {
                ui.spinner();
                ui.label("Loading files...");
            } else if !self.selected_files.is_empty() {
                ui.separator();
                ui.label(format!("Selected files: {}", self.selected_files.len()));

                let mut to_remove = None;

                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .show(ui, |ui| {
                        for (index, file) in self.selected_files.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}.", index + 1));
                                ui.label(&file.name);
                                ui.label(format!("{:?}", file.format));
                                ui.label(FileManager::format_file_size(file.size));

                                if ui.small_button("×").clicked() {
                                    to_remove = Some(index);
                                }
                            });
                        }
                    });

                if let Some(index) = to_remove {
                    self.selected_files.remove(index);
                }

                let total_size: u64 = self.selected_files.iter().map(|f| f.size).sum();
                ui.label(format!(
                    "Total: {}",
                    FileManager::format_file_size(total_size)
                ));
            } else {
                ui.label("No files selected");
            }
        });
    }

    fn load_files_async(&mut self) {
        let (sender, receiver) = mpsc::channel();
        self.file_receiver = Some(receiver);
        self.is_loading = true;

        std::thread::spawn(move || {
            if let Some(paths) = FileManager::open_file_dialog() {
                let files: Vec<ImageFile> = paths
                    .into_iter()
                    .filter_map(FileManager::analyze_file)
                    .collect();
                let _ = sender.send(files);
            } else {
                let _ = sender.send(Vec::new());
            }
        });
    }

    pub fn get_selected_files(&self) -> &[ImageFile] {
        &self.selected_files
    }
}
