use crate::types::declared_type::DtRc;
use std::cell::RefCell;
use std::rc::Rc;

pub(super) type Generics = Rc<RefCell<Vec<Generic>>>;

#[derive(Clone)]
pub(super) struct Generic {
    pub(super) name: String,
    pub(super) declared_type: DtRc,
}

impl Generic {
    pub(super) fn new(name: impl Into<String>, dtrc: DtRc) -> Self {
        Self {
            name: name.into(),
            declared_type: dtrc,
        }
    }
}
