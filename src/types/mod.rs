pub(crate) mod declared_type;
mod field;
mod predefined;
mod type_editor_modal;

use crate::types::declared_type::{DeclaredType, DtRc};
use crate::types::predefined::get_predeclared_types;
use crate::types::type_editor_modal::TypeEditorModal;
use eframe::egui::Ui;
use serbytes::prelude::ReadByteBufferRefMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(super) struct TypePanel {
    types: Vec<DtRc>,
    type_map: HashMap<String, DtRc>,
    type_editor_modal: Option<TypeEditorModal>,
}

impl TypePanel {
    pub(super) fn render(&mut self, ui: &mut Ui, serializer_type: &mut Option<DtRc>) {
        if let Some(type_editor_modal) = &mut self.type_editor_modal {
            if type_editor_modal.render(ui, &mut self.types, &self.type_map) {
                self.type_editor_modal = None;
            }
        }

        if ui.button("Create Type").clicked() {
            self.types
                .push(Rc::new(RefCell::new(DeclaredType::default())));
        }

        ui.separator();

        for declared_type in &self.types {
            ui.menu_button(&declared_type.borrow().name, |ui| {
                if ui.button("Add to serializer").clicked() {
                    *serializer_type = Some(Rc::clone(declared_type));
                }

                if ui.button("Edit").clicked() {
                    self.type_editor_modal = Some(TypeEditorModal::new(Rc::clone(declared_type)))
                }
            });
        }
    }
}

impl Default for TypePanel {
    fn default() -> Self {
        let types = get_predeclared_types();
        let mut type_map = HashMap::with_capacity(types.len());

        for declared_type in types.clone() {
            let name = declared_type.borrow().name.clone();

            type_map.insert(name, declared_type);
        }

        Self {
            types,
            type_map,
            type_editor_modal: None,
        }
    }
}

pub(super) trait GetValueRepr {
    fn get_value_repr(&self, buf: &mut ReadByteBufferRefMut) -> String;
}

pub(super) fn render_code(ui: &mut Ui, code: &str) {
    let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
    egui_extras::syntax_highlighting::code_view_ui(ui, &theme, code, "rs");
}
