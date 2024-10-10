use std::any::Any;

use super::{ECSStorage, ECS};

pub trait System
{
    fn new() -> Self where Self: Sized;

    fn start       (&self, ecs: &mut ECSStorage) { }
    fn update      (&self, ecs: &mut ECSStorage) { }
    fn fixed_update(&self, ecs: &mut ECSStorage) { }
    fn render      (&self, ecs: &mut ECSStorage) { }
}