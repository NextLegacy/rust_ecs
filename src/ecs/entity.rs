#[derive(Copy, Clone, PartialEq, Eq, Default, Hash)]
pub struct EntityUUID(usize);

impl From<EntityUUID> for usize {
    fn from(val: EntityUUID) -> Self {
        val.0
    }
}

impl From<usize> for EntityUUID {
    fn from(value: usize) -> Self {
        EntityUUID(value)
    }
}

impl std::fmt::Debug for EntityUUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EntityUUID({})", self.0)
    }
}
