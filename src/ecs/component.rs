use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    marker::PhantomData,
    rc::Rc,
};

use super::{Entity, EntityMap};

pub trait Component {}

pub struct ComponentStorage {
    pub storage: HashMap<TypeId, Rc<RefCell<Box<dyn Any>>>>,
}

impl ComponentStorage {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn register<T: Component + 'static>(&mut self)
    where
        T: 'static + Component,
    {
        let component: EntityMap<Entity, T> = EntityMap::new();

        self.storage.insert(
            TypeId::of::<T>(),
            Rc::new(RefCell::new(Box::new(component))),
        );
    }

    pub fn insert_into_entity_map<T>(&self, entity: Entity, component: T)
    where
        T: 'static + Component,
    {
        if let Some(mut entity_map) = self.get_mut::<T>() {
            entity_map.insert(entity, component);
        } else {
            dbg!("poop");
        }
    }

    pub fn get<T>(&self) -> Option<Ref<'_, EntityMap<Entity, T>>>
    where
        T: 'static + Component,
    {
        let component_ref = self.storage.get(&TypeId::of::<T>())?;
        Some(Ref::map(component_ref.borrow(), |component| {
            component.downcast_ref::<EntityMap<Entity, T>>().unwrap()
        }))
    }

    pub fn get_mut<T>(&self) -> Option<RefMut<'_, EntityMap<Entity, T>>>
    where
        T: 'static + Component,
    {
        let component_ref = self.storage.get(&TypeId::of::<T>())?;
        Some(RefMut::map(component_ref.borrow_mut(), |component| {
            component.downcast_mut::<EntityMap<Entity, T>>().unwrap()
        }))
    }
}
pub struct CoolComponentStorage<'a> {
    pub storage: HashMap<TypeId, Box<dyn Any>>,
    _marker: PhantomData<&'a ()>,
}

impl<'a> CoolComponentStorage<'a> {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
            _marker: PhantomData,
        }
    }

    pub fn register<T: Component + 'static>(&mut self)
    where
        T: 'static + Component,
    {
        let component: EntityMap<Entity, T> = EntityMap::new();

        self.storage.insert(TypeId::of::<T>(), Box::new(component));
    }

    pub fn get<T>(&'a self) -> Option<&EntityMap<Entity, T>>
    where
        T: 'static + Component,
    {
        let component_ref = self.storage.get(&TypeId::of::<T>())?;
        component_ref.downcast_ref::<EntityMap<Entity, T>>()
    }

    pub fn get_mut<'b, T>(&'a mut self) -> Option<&'b mut EntityMap<Entity, T>>
    where
        T: 'static + Component,
    {
        let component_ref = self.storage.get_mut(&TypeId::of::<T>())?;
        let map = component_ref
            .downcast_mut::<EntityMap<Entity, T>>()
            .unwrap();
        Some(map)
    }

    pub fn insert_into_entity_map<T>(&mut self, entity: Entity, component: T)
    where
        T: 'static + Component,
    {
        let component_ref = self.storage.get_mut(&TypeId::of::<T>()).unwrap();
        let map = component_ref
            .downcast_mut::<EntityMap<Entity, T>>()
            .unwrap();
        map.insert(entity, component);
    }
}
