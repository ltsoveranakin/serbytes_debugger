use crate::types::declared_type::DtRc;
use eframe::egui::Ui;

pub(super) struct SerializerPanel {
    pub(super) current_type: Option<DtRc>,
}

impl SerializerPanel {
    pub(super) fn new() -> Self {
        Self { current_type: None }
    }

    pub(super) fn render(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let current_type = if let Some(ct) = &self.current_type {
                ct
            } else {
                ui.label("No type input");
                return;
            };

            current_type.borrow().render(ui);
        });
    }
}
