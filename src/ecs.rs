use std::{any::{Any, TypeId}, collections::HashMap, iter, ops::Deref};

pub mod component;
pub mod system;
pub mod entity;

use component::{Component, ComponentTypeUUID, ComponentUUID};
use entity::EntityUUID;
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
        if let Some(bit_set) = self.entity_components_bitset.remove(&uuid)
        {
            //iterate over bits and remove components
            let mut component_type_uuid: ComponentTypeUUID = 0;
            for bits in bit_set.data()
            {
                for _ in 0..64
                {
                    if bits & 1 != 0
                    {
                        if let Some(components) = self.components.get_mut(&component_type_uuid)
                        {
                            components.remove(uuid as usize);
                        }
                    }
                    component_type_uuid+=1;
                }
            }
            
            self.deleted_entities.push(uuid);
            true
        }
        else
        {
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
    pub fn for_each_component<T>(&self, mut action: &dyn Fn(usize, &T)) where T: 'static
    {
        let component_type_id = TypeId::of::<T>();
        let component_type_uuid = match self.component_type_id_to_uuid.get(&component_type_id) {
            Some(&uuid) => uuid,
            None => return,
        };

        if let Some(components) = self.components.get(&component_type_uuid)
        {
            components.for_each::<T>(&mut action);
        }
    }

    pub fn for_each_component_mut<T>(&mut self, mut action: impl FnMut(usize, &mut T)) where T: 'static
    {
        let component_type_id = TypeId::of::<T>();
        let component_type_uuid = match self.component_type_id_to_uuid.get(&component_type_id) {
            Some(&uuid) => uuid,
            None => return,
        };

        if let Some(components) = self.components.get_mut(&component_type_uuid)
        {
            components.for_each_mut::<T>(&mut action);
        }
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

    pub fn for_each_component<T>(&self, action: &dyn Fn(usize, &T)) where T: 'static
    {
        self.storage.for_each_component::<T>(action);
    }

    pub fn for_each_component_mut<T>(&mut self, action: impl FnMut(usize, &mut T)) where T: 'static
    {
        self.storage.for_each_component_mut::<T>(action);
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
        self.dynamic_systems.entry(TypeId::of::<TSystem>()).or_insert_with(|| Box::new(TSystem::new()));
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
}