use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;

pub struct TypeRegistry<T>(HashMap<TypeId, Box<T>>);

impl<T> Default for TypeRegistry<T>
where
    T: Default,
{
    fn default() -> Self {
        Self(HashMap::default())
    }
}

impl<T> fmt::Debug for TypeRegistry<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T> TypeRegistry<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert<K>(&mut self, spec: Box<T>)
    where
        K: Any,
    {
        let key = TypeId::of::<K>();
        self.0.insert(key, spec);
    }

    pub fn get<K>(&self) -> Option<&T>
    where
        K: Any,
    {
        let key = TypeId::of::<K>();
        self.0.get(&key).map(Box::as_ref)
    }
}
