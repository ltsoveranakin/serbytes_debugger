use crate::types::declared_type::DtRc;
use crate::types::generics::Generics;
use crate::types::registry::{Named, Registry};
use serbytes::prelude::ReadByteBufferRefMut;
use std::borrow::Cow;

pub(super) type Fields = Registry<Field>;

#[derive(Clone)]
pub(super) struct Field {
    pub(super) name: String,
    pub(super) field_ty: FieldType,
}

impl Field {
    fn new(name: impl Into<String>, field_ty: FieldType) -> Self {
        Self {
            name: name.into(),
            field_ty,
        }
    }

    pub(super) fn new_dt(name: impl Into<String>, dtrc: DtRc) -> Self {
        Self::new(name, FieldType::DtRc(dtrc))
    }

    pub(super) fn new_gen(name: impl Into<String>, index: usize) -> Self {
        Self::new(name, FieldType::Generic(index))
    }

    // pub(super) fn get_value_repr(
    //     &self,
    //     buf: &mut ReadByteBufferRefMut,
    //     deser_field_name_map: &FieldNameMap,
    // ) -> String {
    //     let field_val = deser_field_name_map.get(&self.name);
    //
    //     self.field_ty.borrow().get_value_repr(buf);
    //
    //     format!(
    //         "{}: {}",
    //         self.name,
    //         field_val.map_or_else(
    //             || { "<field not provided from deserializer>" },
    //             |value_str| { value_str }
    //         )
    //     )
    // }

    pub(super) fn deser_value(
        &self,
        buf: &mut ReadByteBufferRefMut,
        generics: &Generics,
    ) -> String {
        let ty_str = match &self.field_ty {
            FieldType::Generic(index) => generics.borrow().get(*index).map_or_else(
                || "<invalid generic index>".to_string(),
                |generic| generic.declared_type.borrow().deser_value(buf),
            ),

            FieldType::DtRc(dtrc) => dtrc.borrow().deser_value(buf),
        };

        format!("{}: {},\n", self.name, ty_str)
    }

    pub(super) fn to_type_string(&self, generics: &Generics) -> String {
        format!("{}: {}", self.name, self.field_ty.to_type_string(generics))
    }
}

#[derive(Clone)]
pub(super) enum FieldType {
    Generic(usize),
    DtRc(DtRc),
}

impl FieldType {
    pub(super) fn to_type_string(&self, generics: &Generics) -> String {
        match self {
            Self::Generic(index) => generics.borrow().get(*index).map_or_else(
                || "<invalid generic index>".to_string(),
                |generic| generic.name.clone(),
            ),

            Self::DtRc(dtrc) => dtrc.borrow().to_string(),
        }
    }
}

impl Named for Field {
    fn get_name(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }
}
