use std::marker::PhantomData;

use anymap::AnyMap;
use slotmap::{SecondaryMap, SlotMap};

use super::Entity;

pub type ComponentMap = AnyMap;

pub trait Component {}

pub type EntityAllocator<K, V> = SlotMap<K, V>;

pub type EntityMap<K, V> = SecondaryMap<K, V>;

pub struct ComponentStorage<'a> {
    pub allocator: EntityAllocator<Entity, ()>,
    pub components: ComponentMap,
    _marker: PhantomData<&'a ()>,
}

impl<'a> ComponentStorage<'a> {
    pub fn new() -> Self {
        Self {
            allocator: EntityAllocator::new(),
            components: ComponentMap::new(),
            _marker: PhantomData,
        }
    }

    pub fn create(&mut self) -> Entity {
        self.allocator.insert(())
    }

    pub fn register<T>(&mut self)
    where
        T: Component + 'static,
    {
        let new_component: EntityMap<Entity, T> = EntityMap::new();
        self.components.insert(new_component);
    }

    pub fn try_register<T>(&mut self)
    where
        T: Component + 'static,
    {
        if self.components.get::<EntityMap<Entity, T>>().is_none() {
            self.register::<T>()
        }
    }

    pub fn get<T>(&self) -> Option<&EntityMap<Entity, T>>
    where
        T: Component + 'static,
    {
        self.components.get::<EntityMap<Entity, T>>()
    }

    pub fn get_mut<T>(&mut self) -> Option<&mut EntityMap<Entity, T>>
    where
        T: Component + 'static,
    {
        self.components.get_mut::<EntityMap<Entity, T>>()
    }
}

pub trait ComponentStore {
    type Item: Component;

    fn get(&self, entity: Entity) -> Option<&Self::Item>;
    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item>;
    fn insert(&mut self, entity: Entity, component: Self::Item);
}

impl<T> ComponentStore for EntityMap<Entity, T>
where
    T: Component,
{
    type Item = T;

    fn get(&self, entity: Entity) -> Option<&T> {
        self.get(entity)
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.get_mut(entity)
    }

    fn insert(&mut self, entity: Entity, component: T) {
        self.insert(entity, component);
    }
}
