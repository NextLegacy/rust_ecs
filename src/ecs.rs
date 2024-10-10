use std::{any::{Any, TypeId}, collections::HashMap, iter, ops::Deref};

pub mod component;
pub mod system;
pub mod entity;
pub mod query;

use component::{Component, ComponentTypeUUID, ComponentUUID};
use entity::EntityUUID;
use query::{ComponentQuery, ComponentQueryMut};
use system::System;

use crate::data_structures::sparse_set::SparseSet;
use crate::data_structures::bit_set::BitSet;

pub struct ECSStorage
{
    components: HashMap<ComponentTypeUUID, SparseSet<1000>>,
    component_type_id_to_uuid: HashMap<TypeId, ComponentTypeUUID>,
    entity_components_bitset: HashMap<EntityUUID, BitSet>,
    deleted_entities: Vec<EntityUUID>,
    entity_uuid_counter: EntityUUID,
    component_uuid_counter: ComponentUUID,
}

pub struct ECS
{
    storage: ECSStorage,
    dynamic_systems: HashMap<TypeId, Box<dyn System>>,
}

impl ECSStorage
{
    pub fn new() -> Self
    {
        Self
        {
            components: HashMap::new(),
            component_type_id_to_uuid: HashMap::new(),
            entity_components_bitset: HashMap::new(),
            deleted_entities: Vec::new(),
            entity_uuid_counter: 0,
            component_uuid_counter: 0,
        }
    }

    pub fn create_entity(&mut self) -> EntityUUID
    {
        let uuid = if self.deleted_entities.is_empty()
        {
            self.entity_uuid_counter += 1;
            self.entity_uuid_counter
        }
        else
        {
            self.deleted_entities.pop().unwrap()
        };

        self.entity_components_bitset.insert(uuid, BitSet::new());

        uuid
    }

    pub fn remove_entity(&mut self, uuid: EntityUUID) -> bool
    {
        if let Some(bit_set) = self.entity_components_bitset.remove(&uuid) {
            let mut component_type_uuid = 0;
            for bits in bit_set.data() {
                let mut bit = 0;
                while bit < usize::BITS as usize {
                    if bits & (1 << bit) != 0 {
                        let component_type_id = self.component_type_id_to_uuid.iter().find(|(_, &uuid)| uuid == component_type_uuid).unwrap().0;
                        let component_type_uuid = self.component_type_id_to_uuid.get(component_type_id).unwrap();
                        let components = self.components.get_mut(component_type_uuid).unwrap();
                        components.remove(uuid);
                    }
                    bit += 1;
                    component_type_uuid += 1;
                }
            }
            self.deleted_entities.push(uuid);
            true
        } else {
            false
        }
    }

    pub fn has_entity(&self, uuid: EntityUUID) -> bool
    {
        self.entity_components_bitset.contains_key(&uuid)
    }

    pub fn add_component<T>(&mut self, uuid: EntityUUID) where T: 'static
    {
        let component_type_id = TypeId::of::<T>();
        let component_type_uuid = *self.component_type_id_to_uuid.entry(component_type_id).or_insert_with(|| 
        {
            self.component_uuid_counter += 1;
            self.component_uuid_counter
        });

        let components = self.components.entry(component_type_uuid).or_insert_with(|| SparseSet::<1000>::new::<T>());
        
        components.emplace(uuid);

        if let Some(bitset) = self.entity_components_bitset.get_mut(&uuid) {
            bitset.set(component_type_uuid);
        }
    }

    pub fn remove_component<T>(&mut self, uuid: EntityUUID) where T: 'static
    {
        let component_type_id = TypeId::of::<T>();
        let component_type_uuid = *self.component_type_id_to_uuid.get(&component_type_id).unwrap();

        if let Some(components) = self.components.get_mut(&component_type_uuid)
        {
            components.remove(uuid);
        }
    }

    pub fn get_component<T>(&self, uuid: EntityUUID) -> Option<&T> where T: 'static
    {
        let component_type_id = TypeId::of::<T>();
        let component_type_uuid = match self.component_type_id_to_uuid.get(&component_type_id) {
            Some(&uuid) => uuid,
            None => return None,
        };

        if let Some(components) = self.components.get(&component_type_uuid)
        {
            components.get::<T>(uuid)
        }
        else
        {
            None
        }
    }
    pub fn iter_components<T: 'static>(&self) -> Box<dyn Iterator<Item = (usize, &T)> + '_>
    {
        let component_type_id = TypeId::of::<T>();
        let component_type_uuid = match self.component_type_id_to_uuid.get(&component_type_id) {
            Some(&uuid) => uuid,
            None => return Box::new(iter::empty()),
        };

        match self.components.get(&component_type_uuid) {
            Some(components) => Box::new(components.iter::<T>()),
            None => Box::new(iter::empty()),
        }
    }

    pub fn iter_components_mut<T: 'static>(&mut self) -> Box<dyn Iterator<Item = (usize, &mut T)> + '_>
    {
        let component_type_id = TypeId::of::<T>();
        let component_type_uuid = match self.component_type_id_to_uuid.get(&component_type_id) {
            Some(&uuid) => uuid,
            None => return Box::new(iter::empty()),
        };

        match self.components.get_mut(&component_type_uuid) {
            Some(components) => Box::new(components.iter_mut::<T>()),
            None => Box::new(iter::empty()),
        }
    }

    pub fn query<'a, T: ComponentQuery<'a>>(&'a self) -> T::Iter {
        T::query(self)
    }

    pub fn query_mut<'a, T: ComponentQueryMut<'a>>(&'a mut self) -> T::Iter {
        T::query_mut(self)
    }
}

impl ECS
{
    pub fn new() -> Self
    {
        Self
        {
            storage: ECSStorage::new(),
            dynamic_systems: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> EntityUUID
    {
        self.storage.create_entity()
    }

    pub fn remove_entity(&mut self, uuid: EntityUUID) -> bool
    {
        self.storage.remove_entity(uuid)
    }

    pub fn has_entity(&self, uuid: EntityUUID) -> bool
    {
        self.storage.has_entity(uuid)
    }

    pub fn add_component<T>(&mut self, uuid: EntityUUID) where T: 'static
    {
        self.storage.add_component::<T>(uuid);
    }

    pub fn remove_component<T>(&mut self, uuid: EntityUUID) where T: 'static
    {
        self.storage.remove_component::<T>(uuid);
    }

    pub fn get_component<T>(&self, uuid: EntityUUID) -> Option<&T> where T: 'static
    {
        self.storage.get_component::<T>(uuid)
    }

    pub fn iter_components<T: 'static>(&self) -> impl Iterator<Item = (usize, &T)>
    {
        self.storage.iter_components::<T>()
    }

    pub fn iter_components_mut<T: 'static>(&mut self) -> impl Iterator<Item = (usize, &mut T)>
    {
        self.storage.iter_components_mut::<T>()
    }

    pub fn entities_count(&self) -> usize
    {
        self.storage.entity_components_bitset.len()
    }

    pub fn storage(&self) -> &ECSStorage
    {
        &self.storage
    }

    pub fn storage_mut(&mut self) -> &mut ECSStorage
    {
        &mut self.storage
    }

    pub fn register_system<TSystem>(&mut self) where TSystem: System + 'static
    {
        self.dynamic_systems.insert(TypeId::of::<TSystem>(), Box::new(TSystem::new()));
    }

    pub fn start(&mut self)
    {
        for system in self.dynamic_systems.values()
        {
            system.start(&mut self.storage);
        }
    }

    pub fn update(&mut self)
    {
        for system in self.dynamic_systems.values()
        {
            system.update(&mut self.storage);
        }
    }

    pub fn fixed_update(&mut self)
    {
        for system in self.dynamic_systems.values()
        {
            system.fixed_update(&mut self.storage);
        }
    }

    pub fn render(&mut self)
    {
        for system in self.dynamic_systems.values()
        {
            system.render(&mut self.storage);
        }
    }

    pub fn serialize<T: serde::Serialize + 'static>(&self) -> Result<String, serde_json::Error>
    {
        self.storage.query::<(&T,)>().map(|(entity, component)| {
            serde_json::to_string(&component)
        }).collect()
    }
}