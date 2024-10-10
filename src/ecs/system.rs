use std::any::Any;

use super::ECSStorage;

pub trait System
{
    fn start       (&self, ecs: &mut ECSStorage) { }
    fn update      (&self, ecs: &mut ECSStorage) { }
    fn fixed_update(&self, ecs: &mut ECSStorage) { }
    fn render      (&self, ecs: &mut ECSStorage) { }
}