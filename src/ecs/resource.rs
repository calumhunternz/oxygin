use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct ResourceStorage {
    pub storage: HashMap<TypeId, Box<dyn Any>>,
}

impl ResourceStorage {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn insert<T: 'static>(&mut self, resource: T) {
        self.storage.insert(TypeId::of::<T>(), Box::new(resource));
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        let resource_ref = self.storage.get(&TypeId::of::<T>())?;
        resource_ref.downcast_ref::<T>()
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let resource_ref = self.storage.get_mut(&TypeId::of::<T>())?;
        resource_ref.downcast_mut::<T>()
    }
}
