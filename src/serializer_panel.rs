use crate::binary_panel::BinaryFile;
use crate::types::declared_type::DtRc;
use crate::types::render_code;
use eframe::egui::Ui;

#[derive(Default)]
pub(super) struct SerializerPanel {
    pub(super) current_type: Option<DtRc>,
}

impl SerializerPanel {
    pub(super) fn render(&self, ui: &mut Ui, binary_file: &Option<BinaryFile>) {
        let current_type = if let Some(ct) = &self.current_type {
            ct
        } else {
            ui.label("No type input");
            return;
        };

        let ct = current_type.borrow();

        ui.columns(2, |cols| {
            cols[0].group(|ui| {
                ct.render(ui);
            });

            if let Some(binary_file) = binary_file
                && let Ok(rbb) = binary_file.get_buffer()
            {
                cols[1].group(|ui| {
                    render_code(ui, &ct.deser_value(&mut rbb.peek().rbb_ref_mut()));
                });
            }
        });
    }
}
