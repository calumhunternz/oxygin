use std::ops::Deref;

use crate::Entity;

#[derive(Clone)]
pub struct Player(Entity);

impl Player {
    pub fn new(entity: &Entity) -> Self {
        Self(entity.clone())
    }
    pub fn inner(&self) -> Entity {
        self.clone().into()
    }
}

impl Deref for Player {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<Entity> for Player {
    fn into(self) -> Entity {
        self.0
    }
}
