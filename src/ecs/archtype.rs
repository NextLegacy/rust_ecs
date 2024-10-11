use std::{any::TypeId, hash::{Hash, Hasher}};

use crate::data_structures::bit_set::{self, BitSet};

use super::ECSStorage;

pub trait Component: {}

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct ArchTypeUUID(usize);

impl ArchTypeUUID {
    pub fn of<T: for<'a> ArchType<'a>>(storage: &mut ECSStorage) -> Self
    {
        let bitset = T::get_bitset(storage);
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        bitset.hash(&mut hasher);
        ArchTypeUUID(hasher.finish() as usize)
    }
}

impl Hash for ArchTypeUUID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<usize> for ArchTypeUUID {
    fn from(value: usize) -> Self {
        ArchTypeUUID(value)
    }
}

impl From<ArchTypeUUID> for usize {
    fn from(val: ArchTypeUUID) -> Self {
        val.0
    }
}

impl From<TypeId> for ArchTypeUUID {
    fn from(val: TypeId) -> Self {
        {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            val.hash(&mut hasher);
            ArchTypeUUID(hasher.finish() as usize)
        }
    }
}

pub trait ArchType<'a> {
    fn get_bitset(storage: &'a mut ECSStorage) -> &BitSet;
}

impl<'a, T: for<'b> Component + 'static> ArchType<'a> for T {
    fn get_bitset(storage: &mut ECSStorage) -> &BitSet {
        storage.get_component_bitset::<T>()
    }
}

impl<'a, T1: for<'b> Component + 'static, T2: for<'b> Component + 'static> ArchType<'a> for (T1, T2)
where
    T1: 'static,
    T2: 'static,
{
    fn get_bitset(storage: &'a mut ECSStorage) -> &BitSet {
        let mut bitset1 = storage.get_component_bitset::<T1>().clone();
        bitset1.union(storage.get_component_bitset::<T2>());

        storage.archtype_bitset_get_or_insert_with::<(T1, T2)>(|| bitset1)
    }
}

impl<'a, T1: for<'b> Component + 'static, T2: for<'b> Component + 'static, T3: for<'b> Component + 'static> ArchType<'a> for (T1, T2, T3)
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
{
    fn get_bitset(storage: &'a mut ECSStorage) -> &BitSet {
        let mut bitset1 = storage.get_component_bitset::<T1>().clone();
        bitset1.union(storage.get_component_bitset::<T2>());
        bitset1.union(storage.get_component_bitset::<T3>());

        storage.archtype_bitset_get_or_insert_with::<(T1, T2, T3)>(|| bitset1)
    }
}

impl<'a, T1: for<'b> Component + 'static, T2: for<'b> Component + 'static, T3: for<'b> Component + 'static, T4: for<'b> Component + 'static> ArchType<'a> for (T1, T2, T3, T4)
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
{
    fn get_bitset(storage: &'a mut ECSStorage) -> &BitSet {
        let mut bitset1 = storage.get_component_bitset::<T1>().clone();
        bitset1.union(storage.get_component_bitset::<T2>());
        bitset1.union(storage.get_component_bitset::<T3>());
        bitset1.union(storage.get_component_bitset::<T4>());

        storage.archtype_bitset_get_or_insert_with::<(T1, T2, T3, T4)>(|| bitset1)
    }
}