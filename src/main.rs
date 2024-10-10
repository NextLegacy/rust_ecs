use ecs::system::System;
use serde::Serialize;

mod ecs;
mod data_structures;

#[derive(Serialize)]
pub struct A { x: i32 }
pub struct B { y: f32 }

pub struct MySystem;

impl ecs::system::System for MySystem 
{
    fn start(&self, ecs: &mut ecs::ECSStorage) {
        ecs.iter_components_mut::<A>().unwrap().for_each(|(entity, a)| {
            a.x = entity as i32;
        });

        ecs.iter_components_mut::<B>().unwrap().for_each(|(entity, b)| {
            b.y = entity as f32;
        });
    }

    fn update(&self, ecs: &mut ecs::ECSStorage) {
        ecs.iter_components::<A>().unwrap().for_each(|(entity, a)| {
            println!("Entity {} has A: {}", entity, a.x);
        });

        ecs.iter_components::<B>().unwrap().for_each(|(entity, b)| {
            println!("Entity {} has B: {}", entity, b.y);
        });
    }
}

fn benchmark(action: impl FnOnce()) -> std::time::Duration
{
    let start = std::time::Instant::now();
    action();
    start.elapsed()
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

    ecs.register_system::<MySystem>();

    let serialized = ecs.serialize::<A>();
    
    ecs.start();
    ecs.update();

    println!("Serialized: {:?}", serialized);
}