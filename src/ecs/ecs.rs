use std::any::TypeId;

use crate::render::{Instance, Vertex};

use super::{Bundle, Bundles, Component, ComponentStorage, EntityMap, ResourceStorage};

use slotmap::DefaultKey;

pub type Entity = DefaultKey;

pub struct ECS<'a> {
    pub store: ComponentStorage<'a>,
    pub resources: ResourceStorage,
    pub bundles: Bundles,
    pub vertices: &'a [Vertex],
    pub instances: Vec<Instance>,
    pub indices: &'a [u16],
}

impl<'a> ECS<'a> {
    pub fn new() -> Self {
        Self {
            store: ComponentStorage::new(),
            resources: ResourceStorage::new(),
            bundles: Bundles::new(),
            vertices: &[
                Vertex {
                    // Index 0
                    position: [-1.0, 1.0, 0.0],
                    color: [1.0, 0.0, 0.0],
                },
                Vertex {
                    // Index 1
                    position: [-1.0, -1.0, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    // Index 2
                    position: [1.0, 1.0, 0.0],
                    color: [0.0, 0.0, 1.0],
                },
                Vertex {
                    // Index 3
                    position: [1.0, -1.0, 0.0],
                    color: [1.0, 0.0, 0.0],
                },
            ],
            instances: vec![],
            indices: &[0, 1, 2, 3, 2, 1],
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.store.create()
    }

    pub fn register_component<T>(&mut self)
    where
        T: Component + 'static,
    {
        self.store.register::<T>();
    }

    pub fn get_component<T>(&self) -> Option<&EntityMap<Entity, T>>
    where
        T: Component + 'static,
    {
        self.store.get()
    }

    pub fn get_mut_component<T>(&mut self) -> Option<&mut EntityMap<Entity, T>>
    where
        T: Component + 'static,
    {
        self.store.get_mut::<T>()
    }

    pub fn add_component<T>(&mut self, entity: Entity, component: T)
    where
        T: Component + 'static,
    {
        self.get_mut_component::<T>()
            .unwrap()
            .insert(entity, component);
    }

    pub fn query<T>(&self, entity: Entity) -> Option<&T>
    where
        T: Component + 'static,
    {
        self.get_component::<T>().unwrap().get(entity)
    }

    pub fn query_mut<T>(&mut self, entity: Entity) -> Option<&mut T>
    where
        T: Component + 'static,
    {
        self.get_mut_component::<T>().unwrap().get_mut(entity)
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

    pub fn register_bundle<T>(&mut self)
    where
        T: Bundle + 'static,
    {
        T::register(&mut self.store);
        self.bundles.insert(TypeId::of::<T>(), ());
    }

    pub fn add_bundle<T>(&mut self, bundle: T) -> Option<Entity>
    where
        T: Bundle + 'static,
    {
        bundle.add_data(&mut self.store, &self.bundles)
    }
}

//
// PARKING FOR NOW (SKILL ISSUED)
//
// Get specific component maps eg get position, physics, size
// Q needs to be an implementation of query data
// get component gets it with type id of T as long as T is bound by component trait
// Query needs needs many impls for varying lengths of tuples
// They cannot be typeids they must be generic
// query cannot do the querying

// pub struct Query<Q: QueryData> {
//     pub data: Q,
// }
//
// impl<Q: QueryData> Query<Q> {
//     pub fn fetch(ecs: ECS) -> Q {
//         todo!()
//     }
// }
//
// trait QueryData {
//     type Data: ComponentData;
//     pub fn get_data(ecs: ECS) -> Self
//     where
//         Self: Sized;
// }
//
// trait ComponentData {}
//
// impl<T1, T2> ComponentData for (T1, T2)
// where
//     T1: Component,
//     T2: Component,
// {
// }
//
// impl<T1, T2> QueryData for (T1, T2)
// where
//     T1: Component,
//     T2: Component,
// {
//     type Data = (T1, T2);
//
//     pub fn get_data(ecs: ECS) -> (Ref<'_, EntityMap<Entity, T1>>, Ref<'_, EntityMap<Entity, T2>>)
//     where
//         Self: Sized,
//     {
//
//     }
// }
