use std::{any::TypeId, hash::{Hash, Hasher}};

/*
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct ComponentType(usize);

impl ComponentType {
    pub fn of<T: 'static>() -> Self {
        let type_id = TypeId::of::<T>();

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        type_id.hash(&mut hasher);

        ComponentType(hasher.finish() as usize)
    }
}

impl From<usize> for ComponentType {
    fn from(value: usize) -> Self {
        ComponentType(value)
    }
}

impl From<ComponentType> for usize {
    fn from(val: ComponentType) -> Self {
        val.0
    }
}
*/