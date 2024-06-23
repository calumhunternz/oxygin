use std::{any::TypeId, collections::HashMap};

use crate::ecs::Entity;

use super::InstanceRaw;

pub struct Renderable {
    pub model: Model,
    pub id: TypeId,
    pub entities: Vec<Entity>,
}

impl Renderable {
    pub fn new<T>(model: T) -> Self
    where
        T: Into<Model> + 'static,
    {
        Self {
            model: model.into(),
            id: TypeId::of::<T>(),
            entities: Vec::new(),
        }
    }
}

pub struct Model {
    pub vertices: Vec<ColorVertex>,
    pub indicies: Vec<u16>,
}

pub struct AssetManager {
    pub assets: Vec<Renderable>,
    pub instances: HashMap<TypeId, Vec<(Entity, InstanceRaw)>>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            assets: Vec::new(),
            instances: HashMap::new(),
        }
    }

    pub fn register<T>(&mut self, model: T)
    where
        T: Into<Model> + 'static,
    {
        self.assets.push(Renderable::new::<T>(model));
        self.instances.insert(TypeId::of::<T>(), Vec::new());
    }

    pub fn add_asset<T>(&mut self, entity: Entity)
    where
        T: Into<Model> + 'static,
    {
        let instance = self.instances.get_mut(&TypeId::of::<T>()).unwrap();
        instance.push((entity, InstanceRaw::default()));
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
