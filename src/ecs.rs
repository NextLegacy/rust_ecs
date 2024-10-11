pub mod component;
pub mod system;
pub mod entity;
pub mod archtype;
pub mod query;

use std::alloc::Layout;
use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};

use archtype::{ArchType, ArchTypeUUID, Component};
use entity::EntityUUID;

use crate::data_structures::sparse_set::SparseSet;
use crate::data_structures::bit_set::BitSet;


pub struct ECSStorage
{
    storage: SparseSet<ArchTypeUUID, SparseSet<EntityUUID, Box<dyn Any>, 1000>, 1000>,
    component_bitsets: SparseSet<ArchTypeUUID, BitSet, 1000>,
    archtype_constructors: SparseSet<ArchTypeUUID, Box<dyn Fn() -> Box<dyn Any>>, 1000>,
    entity_components: SparseSet<EntityUUID, BitSet, 1000>,

    changes: SparseSet<EntityUUID, HashMap<ArchTypeUUID, bool>, 1000>,
}

pub struct ECS
{
    pub storage: ECSStorage,
}

impl ECSStorage
{
    pub fn new() -> Self
    {
        Self
        {
            storage: SparseSet::new(),
            component_bitsets: SparseSet::new(),
            archtype_constructors: SparseSet::new(),
            entity_components: SparseSet::new(),
            changes: SparseSet::new(),
        }
    }

    pub fn get_component_bitset<T: 'static + Component>(&mut self) -> &BitSet
    {
        let component_type_id = TypeId::of::<T>();
        self.component_bitsets.get_or_insert_with(component_type_id.into(), BitSet::new)
    }

    pub fn get_archtype_bitset<T: 'static + for<'a> ArchType<'a>>(&mut self) -> &BitSet
    {
        let archtype_uuid = ArchTypeUUID::of::<T>(self);
        self.component_bitsets.get_or_insert_with(archtype_uuid, BitSet::new)
    }

    pub fn archtype_bitset_get_or_insert_with<T: 'static + for<'a> ArchType<'a>>(&mut self, default: impl FnOnce() -> BitSet) -> &BitSet
    {
        let archtype_uuid = ArchTypeUUID::of::<T>(self);
        self.component_bitsets.get_or_insert_with(archtype_uuid, default)
    }

    pub fn create_entity(&mut self) -> EntityUUID
    {
        rand::random::<usize>().into()
    }

    pub fn register<T: for<'a> ArchType<'a> + Any + 'static>(&mut self, default: impl Fn() -> T + 'static)
    {
        let archtype_uuid = ArchTypeUUID::of::<T>(self);

        let archtype_constructor = Box::new(move || Box::new(default()) as Box<dyn Any>);

        self.archtype_constructors.set(archtype_uuid, archtype_constructor);
    }   

    pub fn add_component<T: 'static + for<'a> ArchType<'a>>(&mut self, entity: EntityUUID)
    {
        let archtype_uuid = ArchTypeUUID::of::<T>(self);
        self.changes.get_or_insert(entity, HashMap::new()).insert(archtype_uuid, true);
    }

    pub fn remove_component<T: 'static + for<'a> ArchType<'a>>(&mut self, entity: EntityUUID)
    {
        let archtype_uuid = ArchTypeUUID::of::<T>(self);
        self.changes.get_or_insert(entity, HashMap::new()).insert(archtype_uuid, false);
    }

    pub fn update_archtypes(&mut self)
    {
        for (entity, changes) in self.changes.iter()
        {
            for (archtype_uuid, add) in changes.iter()
            {
                let archtype = self.storage.get_or_insert_with(*archtype_uuid, SparseSet::new);

                // move components of entity to new archtype
                if *add
                {
                    let entity_components = self.entity_components.get_or_insert_with(entity, BitSet::new);
                    entity_components.union(self.component_bitsets.get_or_insert_with(*archtype_uuid, BitSet::new));

                    let archtype_constructor = self.archtype_constructors.get(*archtype_uuid).unwrap();
                    let component = archtype_constructor();

                    archtype.set(entity, component);
                }
                else
                {
                    let entity_components = self.entity_components.get_mut(entity).unwrap();
                    entity_components.difference(self.component_bitsets.get_or_insert_with(*archtype_uuid, BitSet::new));

                    archtype.remove(entity);
                }
            }
        }
    }

    pub fn iter<T: 'static + for<'a> ArchType<'a>>(&mut self) -> impl Iterator<Item = (EntityUUID, &T)>
    {
        let archtype_uuid = ArchTypeUUID::of::<T>(self);
        self.storage.get_or_insert_with(archtype_uuid, SparseSet::new).iter().map(|(entity, component)| (entity, component.downcast_ref::<T>().unwrap()))
    }
}

impl ECS
{
    pub fn new() -> Self
    {
        Self
        {
            storage: ECSStorage::new(),
        }
    }
}