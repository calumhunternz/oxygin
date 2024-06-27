use nalgebra::{Matrix4, Vector3};

pub const INDICES: &[u16] = &[0, 1, 2, 3, 2, 1];

pub struct Instance {
    pub position: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        let position = Matrix4::new_translation(&self.position);
        let scale = Matrix4::new_nonuniform_scaling(&self.scale);
        InstanceRaw {
            model: (position * scale).into(),
        }
    }
}
pub fn create_projection_matrix(aspect_ratio: f32) -> Matrix4<f32> {
    Matrix4::new_nonuniform_scaling(&nalgebra::Vector3::new(1.0 / aspect_ratio, 1.0, 1.0))
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub projection: [[f32; 4]; 4],
}

impl Uniforms {
    pub fn new(aspect_ratio: f32) -> Self {
        let projection_matrix = create_projection_matrix(aspect_ratio);
        Self {
            projection: projection_matrix.into(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct InstanceRaw {
    pub model: [[f32; 4]; 4],
}

impl InstanceRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }

    pub fn size() -> usize {
        std::mem::size_of::<Self>()
    }
}
