use std::{any::Any, collections::HashMap};

use data_structures::{sparse_set::SparseSet, type_erased_vec::TypeErasedVec};
use ecs::{archtype::Component, system::System, ECS};
use serde::Serialize;

mod ecs;
mod data_structures;

#[derive(Default)]
struct Test {
    x: i32,
}

impl Component for Test {
}

pub fn main()
{
    let mut ecs = ECS::new();

    ecs.storage.register::<Test>(|| Test { x: 0 });
    
    let entity = ecs.storage.create_entity();
    
    ecs.storage.add_component::<Test>(entity);
    
    ecs.storage.update_archtypes();

    for (e, a) in ecs.storage.iter::<Test>()
    {
        println!("Entity: {:?}", e);
    }
}