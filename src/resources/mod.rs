use crate::ecs::Entity;

pub struct Player(Entity);

impl Player {
    pub fn new(entity: &Entity) -> Self {
        Self(*entity)
    }

    pub fn as_ref(&self) -> &Entity {
        &self.0
    }
}

impl Into<Entity> for &Player {
    fn into(self) -> Entity {
        self.0
    }
}
