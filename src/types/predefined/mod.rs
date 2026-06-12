use crate::types::declared_type::{DeclaredType, DtRc};
use serbytes::prelude::SerBytes;
use std::cell::RefCell;

pub(crate) fn get_predeclared_types() -> Vec<DtRc> {
    vec![
        DeclaredType {
            name: "()".to_string(),
            ..Default::default()
        },
        DeclaredType::new(
            "i8",
            Vec::new(),
            |rbb_ref_mut| i8::from_buf(rbb_ref_mut).map(|int| Box::new(int) as Box<dyn ToString>),
            false,
        ),
    ]
    .into_iter()
    .map(|declared_type| DtRc::new(RefCell::new(declared_type)))
    .collect()
}
