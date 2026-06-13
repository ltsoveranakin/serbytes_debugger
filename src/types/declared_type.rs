use crate::types::field::{Field, Fields};
use crate::types::generics::Generics;
use crate::types::registry::Named;
use crate::types::render_code;
use eframe::egui::Ui;
use serbytes::prelude::{BBReadResult, ReadByteBufferRefMut, SerBytes};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Write;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub(crate) type DtRc = Rc<RefCell<DeclaredType>>;
pub(super) type FieldNameMap = HashMap<String, String>;

#[derive(Clone)]
pub(crate) struct DeclaredType {
    pub(crate) name: String,
    pub(super) type_of: TypeOf,
}

impl Default for DeclaredType {
    fn default() -> Self {
        Self::new_fields_value_cb("TypeName", Vec::new())
    }
}

type NewFieldsTy<'a> = Vec<(
    Field,
    &'static dyn Fn(&mut ReadByteBufferRefMut, &Field, &Generics) -> String,
)>;

impl DeclaredType {
    pub(super) fn new(name: impl Into<String>, type_of: TypeOf) -> Self {
        Self {
            name: name.into(),
            type_of,
        }
    }

    pub(super) fn new_prim<T>(name: impl Into<String>) -> Self
    where
        T: SerBytes + ToString + 'static,
    {
        Self::new(
            name,
            TypeOf::Primitive {
                deser_fn: Rc::new(|rbb| T::from_buf(rbb).map(|t| t.to_string())),
            },
        )
    }

    pub(super) fn new_fields_value_cb(name: impl Into<String>, fields: NewFieldsTy) -> Self {
        Self::new_fields_gen(name, fields, Rc::new(RefCell::new(Vec::new())))
    }

    pub(super) fn new_fields_gen(
        name: impl Into<String>,
        fields: NewFieldsTy,
        generics: Generics,
    ) -> Self {
        let mut fields_raw = Vec::with_capacity(fields.len());
        let mut fields_cb = Vec::with_capacity(fields.len());

        for (field, cb) in fields {
            fields_raw.push(field);

            fields_cb.push(cb);
        }

        let fields = Fields::from_vec(fields_raw);
        let generics_fn = Rc::clone(&generics);
        // let generics = Rc::clone(&generics);

        Self::new(
            name,
            TypeOf::FieldsType {
                deser_fn: Rc::new(move |buf, fields, name, _, _| {
                    let mut s = format!("{} {{\n", name);

                    for field in fields.iter() {
                        s.push_str(&field.deser_value(buf, &generics_fn));
                    }

                    s += "}";

                    Ok(s)
                }),
                fields,
                variant: FieldTypeVariant::Struct,
                generics,
            },
        )
    }

    pub(crate) fn render(&self, ui: &mut Ui) {
        render_code(ui, &self.get_code());
    }

    pub(super) fn get_code(&self) -> String {
        match &self.type_of {
            TypeOf::FieldsType { variant, .. } => match variant {
                FieldTypeVariant::Struct => {
                    format!("struct {}", self)
                }

                FieldTypeVariant::Enum => {
                    format!("enum {}", self)
                }
            },

            TypeOf::Primitive { .. } => self.to_string(),
        }
    }

    pub(crate) fn deser_value(&self, buf: &mut ReadByteBufferRefMut) -> String {
        let res = match &self.type_of {
            TypeOf::Primitive { deser_fn, .. } => deser_fn(buf),

            TypeOf::FieldsType {
                deser_fn,
                fields,
                variant,
                generics,
                ..
            } => deser_fn(buf, fields, &self.name, variant, generics),
        };

        res.unwrap_or_else(|_| "<error>".to_string())
    }
}

impl Display for DeclaredType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.type_of {
            TypeOf::FieldsType {
                fields, generics, ..
            } => {
                write!(f, "{} {{\n", self.name)?;

                for field in fields.iter() {
                    write!(f, "{},\n", field.to_type_string(generics))?;
                }

                write!(f, "}}")?;
            }

            TypeOf::Primitive { .. } => {
                f.write_str(&self.name)?;
            }
        }

        Ok(())
    }
}

#[derive(Clone)]
pub(super) enum TypeOf {
    Primitive {
        deser_fn: Rc<dyn Fn(&mut ReadByteBufferRefMut) -> BBReadResult<String>>,
    },
    FieldsType {
        deser_fn: Rc<
            dyn Fn(
                &mut ReadByteBufferRefMut,
                &Fields,
                &String,
                &FieldTypeVariant,
                &Generics,
            ) -> BBReadResult<String>,
        >,
        fields: Fields,
        variant: FieldTypeVariant,
        generics: Generics,
    },
}

#[derive(Clone)]
pub(super) enum FieldTypeVariant {
    Struct,
    Enum,
}

impl Named for DeclaredType {
    fn get_name(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }
}

impl Named for DtRc {
    fn get_name(&self) -> Cow<str> {
        Cow::Owned(self.borrow().get_name().to_string())
    }
}

impl From<DeclaredType> for DtRc {
    fn from(value: DeclaredType) -> Self {
        Self::new(RefCell::new(value))
    }
}
