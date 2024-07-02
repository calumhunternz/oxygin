use std::{any::TypeId, collections::HashMap};

use nalgebra::Vector3;

use crate::ecs::Entity;

use super::InstanceRaw;

#[derive(Debug)]
pub struct Renderable {
    pub model: Model,
    pub id: TypeId,
    pub entities: Vec<Entity>,
}

impl Renderable {
    pub fn new(model: Model, id: TypeId) -> Self {
        Self {
            model,
            id,
            entities: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Model {
    pub vertices: Vec<ColorVertex>,
    pub indicies: Vec<u16>,
}

impl Model {
    pub fn adjust_for_aspect_ratio(&mut self, aspect_ratio: f32) {
        for vertex in &mut self.vertices {
            let mut pos = Vector3::new(vertex.position[0], vertex.position[1], vertex.position[2]);
            if aspect_ratio > 1.0 {
                pos.x /= aspect_ratio;
            } else {
                pos.y *= aspect_ratio;
            }
            vertex.position = [pos.x, pos.y, pos.z];
        }
    }
}

pub struct ReCalculate(bool);

impl ReCalculate {
    pub fn new() -> Self {
        Self(true)
    }
    pub fn inner(&self) -> &bool {
        &self.0
    }
    pub fn moved(&mut self) {
        self.0 = true;
    }
    pub fn finish(&mut self) {
        self.0 = false;
    }
}

pub struct InstanceContainer {
    pub instances: Vec<InstanceRaw>,
    pub entity: Vec<Entity>,
    pub stale: Vec<ReCalculate>,
}

pub struct AssetInstance {
    pub raw: InstanceRaw,
    pub entity: Entity,
    pub stale: bool,
}

impl InstanceContainer {
    pub fn new() -> Self {
        Self {
            instances: Vec::new(),
            entity: Vec::new(),
            stale: Vec::new(),
        }
    }
}

pub struct AssetManager {
    pub assets: Vec<Renderable>,
    pub instances: HashMap<TypeId, InstanceContainer>,
    pub asset_instance: HashMap<TypeId, Vec<AssetInstance>>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            assets: Vec::new(),
            instances: HashMap::new(),
            asset_instance: HashMap::new(),
        }
    }

    pub fn register(&mut self, model: Model, id: TypeId) {
        self.assets.push(Renderable::new(model, id));
        self.instances.insert(id, InstanceContainer::new());
    }

    pub fn add_asset<T>(&mut self, entity: Entity)
    where
        T: Into<Model> + 'static,
    {
        let instance = self.instances.get_mut(&TypeId::of::<T>()).unwrap();
        instance.instances.push(InstanceRaw::default());
        instance.stale.push(ReCalculate::new());
        instance.entity.push(entity);
    }

    pub fn mark_instance_change<T>(&mut self, entity: Entity)
    where
        T: Into<Model> + 'static,
    {
        let instance = self.instances.get_mut(&TypeId::of::<T>()).unwrap();
        for i in 0..instance.entity.len() {
            if instance.entity[i] == entity {
                instance.stale[i].moved();
            }
        }
    }

    pub fn get_instance_data(&self, model: &TypeId) -> &Vec<InstanceRaw> {
        &self.instances.get(model).unwrap().instances
    }
}

pub trait Vertex: Copy + Clone + std::fmt::Debug + bytemuck::Pod + bytemuck::Zeroable {
    const ATTRIBS: &'static [wgpu::VertexAttribute];
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex for ColorVertex {
    const ATTRIBS: &'static [wgpu::VertexAttribute] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3].as_slice();
}

#[derive(Clone)]
pub struct Square {
    pub vertices: Vec<ColorVertex>,
    pub indicies: Vec<u16>,
}

impl Square {
    pub fn new() -> Self {
        Self {
            vertices: vec![
                ColorVertex {
                    position: [-1.0, 1.0, 0.0],
                    color: [1.0, 0.0, 0.0],
                },
                ColorVertex {
                    position: [-1.0, -1.0, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                ColorVertex {
                    position: [1.0, 1.0, 0.0],
                    color: [0.0, 0.0, 1.0],
                },
                ColorVertex {
                    position: [1.0, -1.0, 0.0],
                    color: [1.0, 0.0, 1.0],
                },
            ],
            indicies: vec![0, 1, 2, 3, 2, 1],
        }
    }
}

impl Into<Model> for Square {
    fn into(self) -> Model {
        Model {
            vertices: self.vertices,
            indicies: self.indicies,
        }
    }
}

#[derive(Clone)]
pub struct Food {
    pub vertices: Vec<ColorVertex>,
    pub indicies: Vec<u16>,
}

impl Food {
    pub fn new() -> Self {
        Self {
            vertices: vec![
                ColorVertex {
                    position: [-1.0, 1.0, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                ColorVertex {
                    position: [-1.0, -1.0, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                ColorVertex {
                    position: [1.0, 1.0, 0.0],
                    color: [0.0, 1.0, 1.0],
                },
                ColorVertex {
                    position: [1.0, -1.0, 0.0],
                    color: [1.0, 1.0, 1.0],
                },
            ],
            indicies: vec![0, 1, 2, 3, 2, 1],
        }
    }
}

impl Into<Model> for Food {
    fn into(self) -> Model {
        Model {
            vertices: self.vertices,
            indicies: self.indicies,
        }
    }
}
