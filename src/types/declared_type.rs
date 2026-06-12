use crate::types::field::Field;
use crate::types::{render_code, GetValueRepr};
use eframe::egui::Ui;
use serbytes::prelude::{BBReadResult, ReadByteBufferRefMut};
use std::cell::RefCell;
use std::fmt::Write;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub(crate) type DtRc = Rc<RefCell<DeclaredType>>;

#[derive(Clone)]
pub(crate) struct DeclaredType {
    pub(crate) name: String,
    pub(super) fields: Vec<Field>,
    pub(super) deser_fn: Rc<dyn Fn(&mut ReadByteBufferRefMut) -> BBReadResult<Box<dyn ToString>>>,
    pub(super) is_struct: bool,
}

impl Default for DeclaredType {
    fn default() -> Self {
        Self::new("TypeName", Vec::new(), |_| Ok(Box::new("")), true)
    }
}

impl DeclaredType {
    pub(super) fn new(
        name: impl Into<String>,
        fields: Vec<Field>,
        deser_fn: impl Fn(&mut ReadByteBufferRefMut) -> BBReadResult<Box<dyn ToString>> + 'static,
        is_struct: bool,
    ) -> Self {
        Self {
            name: name.into(),
            fields,
            deser_fn: Rc::new(deser_fn),
            is_struct,
        }
    }

    pub(crate) fn render(&self, ui: &mut Ui) {
        render_code(ui, &self.get_code());
    }

    pub(super) fn get_code(&self) -> String {
        if self.is_struct {
            format!("struct {}", self)
        } else {
            self.to_string()
        }
    }
}

impl GetValueRepr for DeclaredType {
    fn get_value_repr(&self, buf: &mut ReadByteBufferRefMut) -> String {
        if self.is_struct {
            let mut s = format!("{} {{\n", self.name);

            for field in &self.fields {
                write!(s, "{},\n", field.get_value_repr(buf)).unwrap();
            }

            s += "}";

            s
        } else {
            (self.deser_fn)(buf).map_or_else(|_| "<error>".to_string(), |to_str| to_str.to_string())
        }
    }
}

impl Display for DeclaredType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_struct {
            write!(f, "{} {{\n", self.name)?;

            for field in &self.fields {
                write!(f, "{},\n", field)?;
            }

            write!(f, "}}")?;
        } else {
            f.write_str(&self.name)?;
        }

        Ok(())
    }
}
