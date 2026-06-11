use crate::types::declared_type::DtRc;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub(super) struct Field {
    pub(super) name: String,
    pub(super) field_ty: DtRc,
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.field_ty.borrow())
    }
}

impl Field {
    pub(super) fn new(name: String, field_ty: DtRc) -> Self {
        Self { name, field_ty }
    }
}
