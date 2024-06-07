use std::cell::{Ref, RefMut};

use super::{Component, ComponentStorage};

use slotmap::{DefaultKey, SecondaryMap, SlotMap};

pub type Entity = DefaultKey;

pub type EntityAllocator<K, V> = SlotMap<K, V>;

pub type EntityMap<K, V> = SecondaryMap<K, V>;
pub struct ECS {
    pub components: ComponentStorage,
    pub entity_allocator: EntityAllocator<Entity, ()>,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            components: ComponentStorage::new(),
            entity_allocator: EntityAllocator::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_allocator.insert(())
    }

    pub fn register_component<T>(&mut self)
    where
        T: 'static + Component,
    {
        self.components.register::<T>();
    }

    pub fn get_component<T>(&self) -> Option<Ref<'_, EntityMap<Entity, T>>>
    where
        T: 'static + Component,
    {
        self.components.get::<T>()
    }

    pub fn get_mut_component<T>(&self) -> Option<RefMut<'_, EntityMap<Entity, T>>>
    where
        T: 'static + Component,
    {
        self.components.get_mut::<T>()
    }

    pub fn add_component<T: Component + 'static>(&mut self, entity: Entity, component: T)
    where
        T: 'static + Component,
    {
        self.components.insert_into_entity_map(entity, component);
    }
}
