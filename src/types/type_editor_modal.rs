use crate::types::declared_type::{DeclaredType, DtRc};
use crate::types::field::Field;
use eframe::egui::{Id, Modal, Ui};
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;

pub(super) struct TypeEditorModal {
    declared_type_to_change: DtRc,
    declared_type_editing: DeclaredType,
}

impl TypeEditorModal {
    pub(super) fn new(declared_type: DtRc) -> Self {
        let declared_type_editing = declared_type.borrow().clone();

        Self {
            declared_type_to_change: declared_type,
            declared_type_editing,
        }
    }

    pub(super) fn render(
        &mut self,
        ui: &mut Ui,
        types: &Vec<DtRc>,
        type_map: &HashMap<String, DtRc>,
    ) -> bool {
        let modal = Modal::new(Id::new("type-editor")).show(ui, |ui| {
            ui.heading("Edit Type");

            ui.separator();

            ui.label("Name:");
            ui.text_edit_singleline(&mut self.declared_type_editing.name);

            ui.separator();

            ui.label("Fields:");

            if ui.button("Add field").clicked() {
                self.declared_type_editing.fields.push(Field::new(
                    "field".to_string(),
                    Rc::clone(type_map.get("()").unwrap()),
                ))
            }

            let mut delete_index = None;

            for (i, field) in self.declared_type_editing.fields.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut field.name);
                    ui.label(":");

                    let new_type_opt = ui
                        .menu_button(&field.field_ty.borrow().name, |ui| {
                            let mut nt = None;

                            for declared_type in types {
                                if ui.button(&declared_type.borrow().name).clicked() {
                                    nt = Some(Rc::clone(declared_type));
                                }
                            }

                            nt
                        })
                        .inner;

                    if let Some(new_type_o) = new_type_opt
                        && let Some(new_type) = new_type_o
                    {
                        field.field_ty = new_type;
                    }

                    if ui.button("Delete").clicked() {
                        delete_index = Some(i)
                    }
                });
            }

            if let Some(i) = delete_index {
                self.declared_type_editing.fields.remove(i);
            }

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Ok").clicked() {
                    mem::swap(
                        &mut *self.declared_type_to_change.borrow_mut(),
                        &mut self.declared_type_editing,
                    );

                    ui.close();
                }

                if ui.button("Cancel").clicked() {
                    ui.close();
                }
            });
        });

        modal.response.should_close()
    }
}
