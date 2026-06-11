use crate::types::declared_type::{DeclaredType, DtRc};
use std::cell::RefCell;

pub(crate) fn get_predeclared_types() -> Vec<DtRc> {
    vec![DeclaredType::new("()", Vec::new())]
        .into_iter()
        .map(|declared_type| DtRc::new(RefCell::new(declared_type)))
        .collect()
}
