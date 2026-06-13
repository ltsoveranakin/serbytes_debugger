use std::borrow::Cow;
use std::collections::HashMap;
use std::slice::{Iter, IterMut};

#[derive(Clone)]
pub(super) struct Registry<T> {
    vec: Vec<T>,
    name_map: HashMap<String, usize>,
}

impl<T> Default for Registry<T> {
    fn default() -> Self {
        Self {
            vec: Vec::default(),
            name_map: HashMap::default(),
        }
    }
}

impl<T> Registry<T>
where
    T: Named,
{
    pub(super) fn from_vec(v: Vec<T>) -> Self {
        let mut s = Self::default();

        s.reg_mul(v);

        s
    }

    pub(super) fn reg<I>(&mut self, value: I)
    where
        I: Into<T>,
    {
        self.reg_mul(vec![value])
    }

    pub(super) fn reg_mul<I>(&mut self, values: Vec<I>)
    where
        I: Into<T>,
    {
        self.name_map.reserve(values.len());

        for intoable in values {
            let value = intoable.into();

            self.name_map
                .insert(value.get_name().to_string(), self.vec.len());
            self.vec.push(value)
        }
    }

    pub(super) fn reg_with_map<I>(&mut self, f: impl Fn(&Registry<T>) -> I)
    where
        I: Into<T>,
    {
        let declared_type = f(self);

        self.reg(declared_type);
    }

    pub(super) fn get_by_name(&self, name: &str) -> Option<&T> {
        self.name_map.get(name).map(|&index| &self.vec[index])
    }

    pub(super) fn remove(&mut self, index: usize) {
        let field = self.vec.remove(index);

        self.name_map.remove(field.get_name().as_ref());
    }

    pub(super) fn iter(&self) -> Iter<'_, T> {
        self.vec.iter()
    }

    pub(super) fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.vec.iter_mut()
    }

    pub(super) fn into_values(self) -> Vec<T> {
        self.vec
    }
}

pub(super) trait Named {
    fn get_name(&self) -> Cow<str>;
}
