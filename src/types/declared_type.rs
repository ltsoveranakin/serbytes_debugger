use crate::types::field::Field;
use eframe::egui::Ui;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub(crate) type DtRc = Rc<RefCell<DeclaredType>>;

#[derive(Clone)]
pub(crate) struct DeclaredType {
    pub(crate) name: String,
    pub(super) fields: Vec<Field>,
}

impl Default for DeclaredType {
    fn default() -> Self {
        Self::new("TypeName", Vec::new())
    }
}

impl DeclaredType {
    pub(super) fn new(name: impl Into<String>, fields: Vec<Field>) -> Self {
        Self {
            name: name.into(),
            fields,
        }
    }

    pub(crate) fn render(&self, ui: &mut Ui) {
        let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
        egui_extras::syntax_highlighting::code_view_ui(ui, &theme, &self.get_code(), "rs");
    }

    pub(super) fn get_code(&self) -> String {
        format!("struct {}", self)
    }
}

impl Display for DeclaredType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {{\n", self.name)?;

        for field in &self.fields {
            write!(f, "{},\n", field)?;
        }

        write!(f, "}}")?;

        Ok(())
    }
}
