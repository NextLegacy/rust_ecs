use ecs::system::System;

mod ecs;
mod data_structures;

pub struct A { x: i32 }
pub struct B { y: f32 }

pub struct MySystem;

impl ecs::system::System for MySystem {
    fn new() -> Self {
        MySystem
    }

    fn start(&self, ecs: &mut ecs::ECSStorage) {
        ecs.for_each_component_mut(&|entityUUID, component: &mut A| {
            component.x = 10;
        });

        ecs.for_each_component_mut(&|entityUUID, component: &mut B| {
            component.y = 10.0;
        });
    }

    fn update(&self, ecs: &mut ecs::ECSStorage) {
        ecs.for_each_component::<A>(&|entity, a| {
            println!("Entity {} has A: {:?}", entity, a.x);
        });

        ecs.for_each_component::<B>(&|entity, b| {
            println!("Entity {} has B: {:?}", entity, b.y);
        });
    }
}

fn main() 
{
    let mut ecs = ecs::ECS::new();

    let entity1 = ecs.create_entity();
    let entity2 = ecs.create_entity();
    let entity3 = ecs.create_entity();

    ecs.add_component::<A>(entity1);
    ecs.add_component::<A>(entity3);
    
    ecs.add_component::<B>(entity1);
    ecs.add_component::<B>(entity2);
    ecs.add_component::<B>(entity3);

    ecs.remove_component::<B>(entity2);

    if let Some(b) = ecs.get_component::<B>(entity1) {
        println!("Entity 1 has B: {:?}", b.y);
    } else {
        println!("Entity 1 does not have B");
    }

    ecs.register_system::<MySystem>();

    ecs.start();
    ecs.update();
}