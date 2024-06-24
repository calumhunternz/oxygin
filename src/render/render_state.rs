use std::{any::TypeId, collections::HashMap};

use wgpu::{
    util::DeviceExt, Adapter, Backends, BindGroup, BindGroupLayout, Buffer, CommandEncoder, Device,
    DeviceDescriptor, Features, InstanceDescriptor, PipelineCompilationOptions, Queue, RenderPass,
    RequestAdapterOptions, ShaderModule, Surface, SurfaceCapabilities, SurfaceConfiguration,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{components::Render, ecs::ECS};

use super::{InstanceRaw, Uniforms, Vertex, INDICES, VERTICES};

pub struct ModelBuffer {
    vertex: Buffer,
    index: Buffer,
    instance: Buffer,
}

impl ModelBuffer {
    pub fn new(vertex: Buffer, index: Buffer, instance: Buffer) -> Self {
        Self {
            vertex,
            index,
            instance,
        }
    }
}

pub struct RenderState<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: &'a Window,
    pub render_pipeline: wgpu::RenderPipeline,
    pub model_buffers: HashMap<TypeId, ModelBuffer>,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub num_vertices: u32,
    pub num_indices: u32,
    pub instance_buffer: Option<wgpu::Buffer>,
    pub uniforms: Uniforms,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
}
impl<'a> RenderState<'a> {
    pub fn new(window: &'a Window, ecs: &mut ECS) -> RenderState<'a> {
        let size = window.inner_size();
        let instance = Self::get_wgpu_instance();
        let surface = instance.create_surface(window).unwrap();
        let adapter = Self::get_adapter(&surface, &instance);
        let (device, queue) = pollster::block_on(Self::get_device(&adapter));
        let config = Self::create_config(surface.get_capabilities(&adapter), size);
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let num_vertices = VERTICES.len() as u32;
        let num_indices = INDICES.len() as u32;
        let uniforms = Uniforms::new(size.width as f32 / size.height as f32);
        let uniform_buffer = Self::create_uniform_buffer(uniforms, &device);
        let (uniform_bind_group, uniform_bind_group_layout) =
            Self::create_uniform_bind_group(&device, &uniform_buffer);
        let render_pipeline =
            Self::create_render_pipeline(&device, &uniform_bind_group_layout, &shader, &config);

        let model_buffers = Self::create_model_buffers(&device, &ecs);

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer: None,
            model_buffers,
            num_vertices,
            num_indices,
            index_buffer: None,
            instance_buffer: None,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            let updated_uniforms = Uniforms::new(new_size.width as f32 / new_size.height as f32);
            self.queue.write_buffer(
                &self.uniform_buffer,
                0,
                bytemuck::cast_slice(&[updated_uniforms]),
            );
        }
    }

    pub fn render(&mut self, game: &mut ECS) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // TODO: Create and store vertex/index buffer on asset register - Done
        // TODO: Add instance cacheing -> Store instance buffer -> on update check for change (per
        // instance) if there is a change update instance rebuild buffer, otherwise use exisiting
        // TODO: Add extra length to instance buffer so that as more are added does not need to
        // immediatly create new buffer
        // TODO: Potential improvement combine vertex buffers with offsets so one buffer is re-used
        // for all vertex's
        //
        // Stopping point for renderer
        // Buffer re-use
        // Instance buffer padding
        // instance cacheing
        // Added sprites to vectors
        //
        // After the above add entity removal so ring buffer can be implemented
        //
        // would like to do this but this will need to come after entity removal is implemented
        // would have to co-inside with a max component num feature because as instances are
        // overridden the entity will live on in the ecs
        // Overwrite first instance when buffer padding is filled (instance buffer ring method)

        let mut model_buffs = Vec::new();
        let mut instance_buffs = Vec::new();
        let mut instance_lens = Vec::new();

        self.update_instance_data(game);

        for renderable in game.assets.assets.iter() {
            let model_buff = self.model_buffers.get(&renderable.id).unwrap();

            // Loop through
            // currently each instance of renderable has a list of
            let instances = self.get_instance_data(game, &renderable.id);

            // TODO: Use staging buffer to copy into instance buffer

            dbg!(self.device.limits().max_buffer_size);
            // let inst_buff = self.device.create_buffer(&wgpu::BufferDescriptor {
            //     label: Some("Instance Buffer"),
            //     size: ,
            //     usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            //     mapped_at_creation: false,
            // });
            let inst_buff = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Instance Buffer"),
                    contents: bytemuck::cast_slice(instances),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });
            model_buffs.push(model_buff);
            instance_buffs.push(inst_buff);
            instance_lens.push(instances.len());
        }

        let mut render_pass = self.create_render_pass(&view, &mut encoder)?;
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);

        for i in 0..model_buffs.len() {
            render_pass.set_vertex_buffer(0, model_buffs[i].vertex.slice(..));
            render_pass.set_index_buffer(model_buffs[i].index.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_vertex_buffer(1, instance_buffs[i].slice(..));
            render_pass.draw_indexed(0..self.num_indices, 0, 0..instance_lens[i] as _);
        }

        drop(render_pass);
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn create_render_pass<'b>(
        &'b self,
        view: &'b wgpu::TextureView,
        encoder: &'b mut CommandEncoder,
    ) -> Result<RenderPass, wgpu::SurfaceError> {
        Ok(encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        }))
    }

    fn update_instance_data(&self, ecs: &mut ECS) {
        let model_ids = ecs
            .assets
            .assets
            .iter()
            .map(|x| x.id.clone())
            .collect::<Vec<_>>();

        // gets every model and checks if it needs to be recalculated
        // if it does get the render from ecs re-calculate to_raw and then update the instance raw
        // in renderable instances
        for model in model_ids {
            let instance = ecs.assets.instances.get(&model).unwrap();
            let mut re_calculated_instances = Vec::new();
            for i in 0..instance.instances.len() {
                if *instance.re_calculate[i].inner() {
                    let instance_component = ecs.query::<Render>(instance.entity[i]).unwrap();
                    re_calculated_instances.push((i, instance_component.to_raw()));
                }
            }

            let instance = ecs.assets.instances.get_mut(&model).unwrap();
            for (i, raw) in re_calculated_instances {
                dbg!(&i);
                instance.instances[i] = raw;
                instance.re_calculate[i].finish();
            }
        }
    }

    fn get_instance_data(&self, ecs: &'a ECS, model: &TypeId) -> &'a Vec<InstanceRaw> {
        &ecs.assets.instances.get(model).unwrap().instances
    }

    fn create_model_buffers(device: &Device, ecs: &ECS) -> HashMap<TypeId, ModelBuffer> {
        let mut buffers = HashMap::new();

        for renderable in ecs.assets.assets.iter() {
            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&renderable.model.vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&renderable.model.indicies),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            });
            let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Instance Buffer"),
                size: (std::mem::size_of::<InstanceRaw>() * 5) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            buffers.insert(
                renderable.id,
                ModelBuffer::new(vertex_buffer, index_buffer, instance_buffer),
            );
        }
        buffers
    }

    fn create_uniform_bind_group(device: &Device, buffer: &Buffer) -> (BindGroup, BindGroupLayout) {
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("uniform_bind_group_layout"),
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });
        (uniform_bind_group, uniform_bind_group_layout)
    }

    fn create_uniform_buffer(uniforms: Uniforms, device: &Device) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    // fn create_instance_buffer(&self, instances: &Vec<InstanceRaw>) -> Buffer {
    //     self.device
    //         .create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //             label: Some("Instance Buffer"),
    //             contents: bytemuck::cast_slice(instances),
    //             usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    //         })
    // }

    // fn create_vertex_buffer(&self, ecs: &ECS) -> Buffer {
    //     self.device
    //         .create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //             label: Some("Vertex Buffer"),
    //             contents: bytemuck::cast_slice(&ecs.vertices),
    //             usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    //         })
    // }

    // fn create_index_buffer(&self, ecs: &ECS) -> Buffer {
    //     self.device
    //         .create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //             label: Some("Index Buffer"),
    //             contents: bytemuck::cast_slice(ecs.indices),
    //             usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
    //         })
    // }

    fn create_render_pipeline(
        device: &Device,
        uniform_bind_group_layout: &BindGroupLayout,
        shader: &ShaderModule,
        config: &SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[uniform_bind_group_layout],
                push_constant_ranges: &[],
            });
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc(), InstanceRaw::desc()],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
    }

    fn get_wgpu_instance() -> wgpu::Instance {
        wgpu::Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        })
    }

    fn get_adapter(surface: &Surface, instance: &wgpu::Instance) -> Adapter {
        pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(surface),
            force_fallback_adapter: false,
        }))
        .unwrap()
    }

    async fn get_device(adapter: &Adapter) -> (Device, Queue) {
        adapter
            .request_device(
                &DeviceDescriptor {
                    required_features: Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap()
    }

    fn create_config(
        surface_caps: SurfaceCapabilities,
        size: PhysicalSize<u32>,
    ) -> SurfaceConfiguration {
        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps
                .formats
                .iter()
                .find(|f| f.is_srgb())
                .copied()
                .unwrap_or(surface_caps.formats[0]),
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }
}
