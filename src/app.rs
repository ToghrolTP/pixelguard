use crate::ui::UiState;
use eframe::egui;

pub struct PixelGuardApp {
    ui_state: UiState,
}

impl PixelGuardApp {
    pub fn new() -> Self {
        Self {
            ui_state: UiState::new(),
        }
    }
}

impl eframe::App for PixelGuardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui_state.render(ctx);
    }
}
