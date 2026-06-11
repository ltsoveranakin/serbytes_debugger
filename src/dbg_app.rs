use crate::serializer_panel::SerializerPanel;
use crate::types::TypePanel;
use eframe::egui::{CentralPanel, Panel, Ui};
use eframe::{App, Frame};

pub(super) struct DebuggerApp {
    type_panel: TypePanel,
    serializer_panel: SerializerPanel,
}

impl App for DebuggerApp {
    fn ui(&mut self, ui: &mut Ui, _: &mut Frame) {
        Panel::left("type-decl").show_inside(ui, |ui| {
            self.type_panel
                .render(ui, &mut self.serializer_panel.current_type);
        });

        CentralPanel::default().show_inside(ui, |ui| {
            self.serializer_panel.render(ui);
        });
    }
}

impl DebuggerApp {
    pub(super) fn new() -> Self {
        Self {
            type_panel: TypePanel::new(),
            serializer_panel: SerializerPanel::new(),
        }
    }
}
