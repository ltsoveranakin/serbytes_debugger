use crate::types::declared_type::{DeclaredType, DtRc, FieldTypeVariant, TypeOf};
use crate::types::field::{Field, Fields};
use crate::types::generics::Generic;
use crate::types::registry::Registry;
use serbytes::prelude::SerBytes;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

type TypeRegistry = Registry<DtRc>;

#[derive(SerBytes)]
struct UnitTy;

impl Display for UnitTy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "()")
    }
}

pub(crate) fn get_predeclared_types() -> Vec<DtRc> {
    let mut r = TypeRegistry::default();

    r.reg_mul(vec![
        DeclaredType::new_prim::<UnitTy>("()"),
        DeclaredType::new_prim::<i8>("i8"),
        DeclaredType::new_prim::<u8>("u8"),
        DeclaredType::new_prim::<u16>("u16"),
    ]);

    r.reg_with_map(|r| {
        let dtrc_unit = Rc::clone(r.get_by_name("()").unwrap());
        let dtrc_u16 = Rc::clone(r.get_by_name("u16").unwrap());
        let dtrc_u16_clos = Rc::clone(&dtrc_u16);

        DeclaredType::new(
            "Vec",
            TypeOf::FieldsType {
                deser_fn: Rc::new(move |buf, b, c, d, generics| {
                    let mut peek = buf.peek();
                    let mut rbb = peek.rbb_ref_mut();

                    let len = u16::from_buf(&mut rbb).unwrap_or_default();
                    let mut contents = Vec::with_capacity(len as usize);

                    for _ in 0..len {
                        contents.push(
                            generics.borrow()[0]
                                .declared_type
                                .borrow()
                                .deser_value(&mut rbb),
                        );
                    }

                    let mut s = format!("{{\n{}\n", dtrc_u16_clos.borrow().deser_value(buf));

                    s.push_str(&format!("contents: [{}]\n}}", contents.join(", ")));

                    Ok(s)
                }),
                fields: Fields::from_vec(vec![
                    Field::new_dt("len", dtrc_u16),
                    Field::new_gen("contents", 0),
                ]),
                variant: FieldTypeVariant::Struct,
                generics: Rc::new(RefCell::new(vec![Generic::new("S", dtrc_unit)])),
            },
        )
    });

    r.into_values()
}
