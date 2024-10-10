use data_structures::type_erased_vec::TypeErasedVec;
use ecs::system::System;
use serde::Serialize;

mod ecs;
mod data_structures;

#[derive(Serialize)]
pub struct A { x: i32 }
pub struct B { y: f32 }
pub struct C { z: f32 }

pub struct MySystem;

impl ecs::system::System for MySystem 
{
    fn new() -> Self {
        Self
    }

    fn start(&self, ecs: &mut ecs::ECSStorage) {
        ecs.iter_components_mut::<A>().for_each(|(entity, a)| {
            a.x = 42;
        });

        ecs.iter_components_mut::<B>().for_each(|(entity, b)| {
            b.y = entity as f32;
        });
    }

    fn update(&self, ecs: &mut ecs::ECSStorage) {
        ecs.iter_components::<A>().for_each(|(entity, a)| {
            println!("Entity {} has A: {}", entity, a.x);
        });

        ecs.iter_components::<B>().for_each(|(entity, b)| {
            println!("Entity {} has B: {}", entity, b.y);
        });

        ecs.query::<(&A,)>().for_each(|(entity, a)| {
            println!("Entity {} has A: {}", entity, a.x);
        });
        
        ecs.query_mut::<(&A, &mut B)>().for_each(|(entity, a, b)| {
            b.y = 42.0;
            println!("Entity {} has A: {} and B: {}", entity, a.x, b.y);
        });

        ecs.query_mut::<(&A, &mut B, &mut C)>().for_each(|(entity, a, b, c)| {
            b.y = 42.0;
            c.z = 42.0;
            println!("Entity {} has A: {}, B: {} and C: {}", entity, a.x, b.y, c.z);
        });
    }
}

fn benchmark(action: impl FnOnce()) -> std::time::Duration
{
    let start = std::time::Instant::now();
    action();
    start.elapsed()
}

fn benchmark_main()
{
    let a = benchmark(|| {
        let mut vec = vec![0; 1000000];
        for i in 0..10000
        {
            vec[rand::random::<usize>() % 1000000] = i;
        }

        for i in 0..1000
        {
            let _: Option<&i32> = vec.get(rand::random::<usize>() % 1000000);
        }

        for i in 0..1000
        {
            vec.swap_remove(rand::random::<usize>() % vec.len());
        }

        vec.iter().for_each(|x| { let _ = x; });
    });

    let b = benchmark(|| {
        let mut vec = data_structures::sparse_set::SparseSet::<1000>::new::<i32>();

        for i in 0..10000
        {
            vec.set(rand::random::<usize>() % 1000000, i);
        }
            
        for i in 0..1000
        {
            let _: Option<&i32> = vec.get(rand::random::<usize>() % 1000000);
        }

        for i in 0..1000
        { 
            vec.remove(rand::random::<usize>() % vec.len());
        }

        vec.iter::<i32>().for_each(|(index, x)| { let _ = (index, x); });
    });

    println!("Vec: {:?}, SparseSet: {:?}", a, b);
}

fn main() 
{
    let mut vec = TypeErasedVec::new::<u8>();
    vec.push::<i32>(1);
    vec.as_typed_slice_mut::<u8>()[0] = 1;
    vec.as_typed_slice_mut::<u8>()[1] = 2;
    vec.as_typed_slice_mut::<u8>()[2] = 3;
    vec.as_typed_slice_mut::<u8>()[3] = 4;

    println!("{:?}", vec.as_typed_slice_mut::<u8>());

    let mut ecs = ecs::ECS::new();

    let entity1 = ecs.create_entity();
    let entity2 = ecs.create_entity();
    let entity3 = ecs.create_entity();

    ecs.add_component::<A>(entity1);
    ecs.add_component::<B>(entity1);

    ecs.add_component::<A>(entity2);
    ecs.add_component::<B>(entity3);

    ecs.register_system::<MySystem>();

    ecs.remove_entity(entity1);
    
    ecs.add_component::<A>(entity1);
    ecs.add_component::<B>(entity1);
    ecs.add_component::<C>(entity1);
    ecs.add_component::<C>(entity2);
    ecs.add_component::<C>(entity3);
    ecs.add_component::<B>(entity2);

    ecs.start();
    ecs.update();

    let serialized = ecs.serialize::<A>();
    println!("Serialized: {:?}", serialized);

    // stress_test();
    
}

fn stress_test() -> u32
{
    let mut ecs = ecs::ECS::new();
    ecs.register_system::<MySystem>();
    let start = std::time::Instant::now();

    for i in 0..100000
    {
        let entity = ecs.create_entity();
        ecs.add_component::<A>(entity);
        ecs.add_component::<B>(entity);
        ecs.add_component::<C>(entity);
    }

    for i in 0..100000
    {
        let entity = ecs.create_entity();
        ecs.add_component::<A>(entity);
        ecs.add_component::<B>(entity);
    }

    for i in 0..100000
    {
        let entity = ecs.create_entity();
        ecs.add_component::<A>(entity);
    }

    for i in 0..100000
    {
        let entity = ecs.create_entity();
        ecs.add_component::<B>(entity);
    }

    for i in 0..100000
    {
        let entity = ecs.create_entity();
        ecs.add_component::<C>(entity);
    }

    for i in 0..100000
    {
        let entity = ecs.create_entity();
        ecs.add_component::<A>(entity);
        ecs.add_component::<B>(entity);
        ecs.add_component::<C>(entity);
    }

    for i in 0..100000
    {
        let entity = ecs.create_entity();
        ecs.add_component::<A>(entity);
        ecs.add_component::<B>(entity);
    }

    for i in 0..100000
    {
        let entity = ecs.create_entity();
        ecs.add_component::<A>(entity);
    }

    for i in 0..100000
    {
        let entity = ecs.create_entity();
        ecs.add_component::<B>(entity);
    }

    for i in 0..100000
    {
        let entity = ecs.create_entity();
        ecs.add_component::<C>(entity);
    }

    println!("Entities created: {}", ecs.entities_count());

    ecs.start();
    ecs.update();

    let elapsed = start.elapsed();

    println!("Stress test took: {:?}", elapsed);

    elapsed.as_millis() as u32
}