use std::{any::TypeId, collections::HashMap};

use super::{Component, ComponentStorage, Entity};

pub type Bundles = HashMap<TypeId, ()>;

pub trait Bundle {
    fn register(store: &mut ComponentStorage);
    fn add_data(self, store: &mut ComponentStorage, bundles: &Bundles) -> Option<Entity>;
}

impl<C> Bundle for C
where
    C: Component + 'static,
{
    fn register(store: &mut ComponentStorage) {
        store.register::<Self>();
    }

    fn add_data(self, store: &mut ComponentStorage, bundles: &Bundles) -> Option<Entity> {
        if let Some(_) = bundles.get(&TypeId::of::<Self>()) {
            let entity = store.create();
            store.get_mut::<C>().unwrap().insert(entity, self);
            Some(entity)
        } else {
            None
        }
    }
}

impl<C> Bundle for (C,)
where
    C: Component + 'static,
{
    fn register(store: &mut ComponentStorage) {
        store.try_register::<C>();
    }

    fn add_data(self, store: &mut ComponentStorage, bundles: &Bundles) -> Option<Entity> {
        if let Some(_) = bundles.get(&TypeId::of::<Self>()) {
            let entity = store.create();
            store.get_mut::<C>().unwrap().insert(entity, self.0);
            Some(entity)
        } else {
            None
        }
    }
}

impl<T1, T2> Bundle for (T1, T2)
where
    T1: Component + 'static,
    T2: Component + 'static,
{
    fn register(store: &mut ComponentStorage) {
        store.try_register::<T1>();
        store.try_register::<T2>();
    }

    fn add_data(self, store: &mut ComponentStorage, bundles: &Bundles) -> Option<Entity> {
        if let Some(_) = bundles.get(&TypeId::of::<Self>()) {
            let entity = store.create();
            store.get_mut::<T1>().unwrap().insert(entity, self.0);
            store.get_mut::<T2>().unwrap().insert(entity, self.1);
            Some(entity)
        } else {
            None
        }
    }
}

impl<T1, T2, T3> Bundle for (T1, T2, T3)
where
    T1: Component + 'static,
    T2: Component + 'static,
    T3: Component + 'static,
{
    fn register(store: &mut ComponentStorage) {
        store.try_register::<T1>();
        store.try_register::<T2>();
        store.try_register::<T3>();
    }

    fn add_data(self, store: &mut ComponentStorage, bundles: &Bundles) -> Option<Entity> {
        if let Some(_) = bundles.get(&TypeId::of::<Self>()) {
            let entity = store.create();
            store.get_mut::<T1>().unwrap().insert(entity, self.0);
            store.get_mut::<T2>().unwrap().insert(entity, self.1);
            store.get_mut::<T3>().unwrap().insert(entity, self.2);
            Some(entity)
        } else {
            None
        }
    }
}
