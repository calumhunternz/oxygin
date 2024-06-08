use std::{
    borrow::Borrow,
    cell::{Ref, RefMut},
};

use crate::{component, CoolComponentStorage, ResourceStorage};

use super::{Component, ComponentStorage};

use slotmap::{DefaultKey, SecondaryMap, SlotMap};

pub type Entity = DefaultKey;

pub type EntityAllocator<K, V> = SlotMap<K, V>;

pub type EntityMap<K, V> = SecondaryMap<K, V>;

pub struct ECS {
    pub components: ComponentStorage,
    pub resources: ResourceStorage,
    pub entity_allocator: EntityAllocator<Entity, ()>,
}
// macro_rules! impl_query {
//         ($($name:ident),*) => {
//             pub fn query<$($name),*>(
//                 &self,
//                 entity: Entity,
//             ) -> Option<($(Ref<'_, $name>,)*)>
//             where
//                 $($name: 'static + Component),*
//             {
//                 Some((
//                     $(Ref::map(self.get_component::<$name>()?, |entity_map| {
//                         entity_map.get(entity).unwrap()
//                     }),)*
//                 ))
//             }
//         }
//     }

impl ECS {
    pub fn new() -> Self {
        Self {
            components: ComponentStorage::new(),
            resources: ResourceStorage::new(),
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

    pub fn add_resource<T: 'static>(&mut self, resource: T) {
        self.resources.insert(resource)
    }

    pub fn get_resource<T: 'static>(&self) -> Option<&T> {
        self.resources.get::<T>()
    }

    pub fn get_mut_resource<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    pub fn query_mut<T>(&self, entity: Entity) -> Option<RefMut<'_, T>>
    where
        T: 'static + Component,
    {
        // let component = self.get_mut_component::<T>()?;
        Some(RefMut::map(self.get_mut_component::<T>()?, |entity_map| {
            entity_map.get_mut(entity).unwrap()
        }))
    }

    //
    // pub fn query<Q>(&self, entity: Entity) -> Option<Q>

    pub fn query<T>(&self, entity: Entity) -> Option<Ref<'_, T>>
    where
        T: 'static + Component,
    {
        Some(Ref::map(self.get_component::<T>()?, |entity_map| {
            entity_map.get(entity).unwrap()
        }))
    }
}

pub struct ECS2<'a> {
    pub components: CoolComponentStorage<'a>,
    pub resources: ResourceStorage,
    pub entity_allocator: EntityAllocator<Entity, ()>,
}

impl<'a> ECS2<'a> {
    pub fn new() -> Self {
        Self {
            components: CoolComponentStorage::new(),
            resources: ResourceStorage::new(),
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

    pub fn get_component<T>(&self) -> Option<&EntityMap<Entity, T>>
    where
        T: 'static + Component,
    {
        self.components.get::<T>()
    }

    pub fn get_mut_component<T>(&mut self) -> Option<&mut EntityMap<Entity, T>>
    where
        T: 'static + Component,
    {
        self.components.get_mut::<T>()
    }

    pub fn add_component<T>(&mut self, entity: Entity, component: T)
    where
        T: 'static + Component,
    {
        self.components.insert_into_entity_map(entity, component);
    }

    pub fn add_resource<T: 'static>(&mut self, resource: T) {
        self.resources.insert(resource)
    }

    pub fn get_resource<T: 'static>(&self) -> Option<&T> {
        self.resources.get::<T>()
    }

    pub fn get_mut_resource<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }
}

// game.query::<Position>(entity).unwrap()
// let (entity_position, entity_size) = game.query::<Position, Size>(entity).unwrap()
// game.query_mut::<Position>(entity).unwrap()
// Query builder
// query.get<T>(

// Query builder
//
// let query = QueryBuilder::get<T>().get<T>().build() -> Query
// pub fn query<c1, c2, c3, c4>...etc (&self, query: Query) -> Option<&(c1, c2, c3, c4...)>
//

// trait Query<'a> {
//     type Output;
//     fn query(&self, entity: Entity) -> Option<Self::Output>;
// }
//
// impl<'a, T1, T2, T3> Query<'a> for (T1, T2, T3)
// where
//     T1: 'static + Component,
//     T2: 'static + Component,
//     T3: 'static + Component,
// {
//     type Output = (Ref<'a, T1>, Ref<'a, T2>, Ref<'a, T3>);
//
//     fn query(&self, entity: Entity) -> Option<Self::Output> {
//         let component: EntityMap<Entity, T1> = self.0.get::<T1>();
//         Some((
//             Ref::map(self.0.get_component::<T1>()?, |entity_map| {
//                 entity_map.get(entity).unwrap()
//             }),
//             Ref::map(self.1.get_component::<T2>()?, |entity_map| {
//                 entity_map.get(entity).unwrap()
//             }),
//             Ref::map(self.2.get_component::<T3>()?, |entity_map| {
//                 entity_map.get(entity).unwrap()
//             }),
//         ))
//     }
// }
