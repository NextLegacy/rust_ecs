pub type ComponentUUID = usize;
pub type ComponentTypeUUID = usize;

pub trait Component: 'static 
{
    fn new() -> Self where Self: Sized;
}

pub struct EntityComponent<T> where T: Component
{
    pub uuid: ComponentUUID,
    pub component: T,
}