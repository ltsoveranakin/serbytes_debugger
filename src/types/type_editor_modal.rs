use crate::types::declared_type::{DeclaredType, DtRc, TypeOf};
use crate::types::field::{Field, FieldType};
use crate::types::generics::Generic;
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

            if let TypeOf::FieldsType {
                fields, generics, ..
            } = &mut self.declared_type_editing.type_of
            {
                ui.separator();

                ui.label("Generics:");

                if ui.button("Add generic").clicked() {
                    generics
                        .borrow_mut()
                        .push(Generic::new("T", Rc::clone(type_map.get("()").unwrap())))
                }

                for generic in generics.borrow_mut().iter_mut() {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut generic.name);
                        if ui.button("Delete").clicked() {}
                    });
                }

                ui.separator();

                ui.label("Fields:");

                if ui.button("Add field").clicked() {
                    fields.reg(Field::new_dt(
                        "field",
                        Rc::clone(type_map.get("()").unwrap()),
                    ))
                }

                let mut delete_index = None;

                for (i, field) in fields.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        let field_type_text = match field.field_ty {
                            FieldType::Generic(_) => "generic",
                            FieldType::DtRc(_) => "concrete type",
                        };

                        ui.menu_button(format!("Field type: {}", field_type_text), |ui| {
                            if ui.button("Concrete").clicked() {
                                field.field_ty =
                                    FieldType::DtRc(Rc::clone(type_map.get("()").unwrap()));
                            }

                            if ui.button("Generic").clicked() {
                                field.field_ty = FieldType::Generic(0);
                            }
                        });
                        ui.text_edit_singleline(&mut field.name);
                        ui.label(":");

                        match &field.field_ty {
                            FieldType::Generic(index) => {
                                let generics = generics.borrow();
                                let generic_at_index_name = generics
                                    .get(*index)
                                    .map_or_else(|| "<invalid generic index>", |gener| &gener.name);

                                let new_gen_index_opt = ui
                                    .menu_button(generic_at_index_name, |ui| {
                                        let mut ngi = None;

                                        for (i, generic) in generics.iter().enumerate() {
                                            if ui.button(&generic.name).clicked() {
                                                ngi = Some(i);
                                            }
                                        }

                                        ngi
                                    })
                                    .inner;

                                if let Some(new_index_o) = new_gen_index_opt
                                    && let Some(new_index) = new_index_o
                                {
                                    field.field_ty = FieldType::Generic(new_index);
                                }
                            }

                            FieldType::DtRc(dtrc) => {
                                let new_type_opt = ui
                                    .menu_button(&dtrc.borrow().name, |ui| {
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
                                    field.field_ty = FieldType::DtRc(new_type);
                                }
                            }
                        };

                        if ui.button("Delete").clicked() {
                            delete_index = Some(i)
                        }
                    });
                }

                if let Some(i) = delete_index {
                    fields.remove(i);
                }
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
