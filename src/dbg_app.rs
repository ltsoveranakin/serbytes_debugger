use crate::binary_panel::BinaryPanel;
use crate::serializer_panel::SerializerPanel;
use crate::types::TypePanel;
use eframe::egui::{CentralPanel, Panel, Ui};
use eframe::{App, Frame};

#[derive(Default)]
pub(super) struct DebuggerApp {
    type_panel: TypePanel,
    binary_panel: BinaryPanel,
    serializer_panel: SerializerPanel,
}

impl App for DebuggerApp {
    fn ui(&mut self, ui: &mut Ui, _: &mut Frame) {
        Panel::left("type-panel").show_inside(ui, |ui| {
            self.type_panel
                .render(ui, &mut self.serializer_panel.current_type);
        });

        Panel::right("binary-panel").show_inside(ui, |ui| {
            self.binary_panel.render(ui);
        });

        CentralPanel::default().show_inside(ui, |ui| {
            self.serializer_panel
                .render(ui, &mut self.binary_panel.binary_file);
        });
    }
}
