use std::{any::TypeId, collections::HashMap, num::NonZeroU64};

use wgpu::{
    util::{DeviceExt, StagingBelt},
    Adapter, Backends, Buffer, CommandEncoder, Device, DeviceDescriptor, Features,
    InstanceDescriptor, PipelineCompilationOptions, Queue, RenderPass, RequestAdapterOptions,
    ShaderModule, Surface, SurfaceCapabilities, SurfaceConfiguration,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{components::Render, ecs::ECS};

use super::{asset_manager::AssetManager, InstanceRaw, Vertex, INDICES, VERTICES};

pub struct ModelBuffer {
    vertex: Buffer,
    index: Buffer,
    instance: Buffer,
    capacity: usize,
}

impl ModelBuffer {
    pub fn new(vertex: Buffer, index: Buffer, instance: Buffer, capacity: usize) -> Self {
        Self {
            vertex,
            index,
            instance,
            capacity,
        }
    }

    pub fn increase_capacity(&mut self, new_capacity: usize, device: &Device) {
        self.instance = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (InstanceRaw::size() * new_capacity) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.capacity = new_capacity;
    }
}

pub struct RenderState<'render> {
    pub surface: wgpu::Surface<'render>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: &'render Window,
    pub render_pipeline: wgpu::RenderPipeline,
    pub model_buffers: HashMap<TypeId, ModelBuffer>,
    pub staging_belt: StagingBelt,
    pub staging_capacity: usize,
    pub num_vertices: u32,
    pub num_indices: u32,
}
impl<'render> RenderState<'render> {
    pub fn new(window: &'render Window, asset_manager: &mut AssetManager) -> RenderState<'render> {
        let size = window.inner_size();
        let instance = Self::get_wgpu_instance();
        let surface = instance.create_surface(window).unwrap();
        let adapter = Self::get_adapter(&surface, &instance);
        let (device, queue) = pollster::block_on(Self::get_device(&adapter));
        let config = Self::create_config(surface.get_capabilities(&adapter), size);
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let num_vertices = VERTICES.len() as u32;
        let num_indices = INDICES.len() as u32;
        let render_pipeline = Self::create_render_pipeline(&device, &shader, &config);
        let staging_belt = StagingBelt::new((InstanceRaw::size() * 20) as u64);

        let staging_capacity = 20;
        let model_buffers = Self::create_model_buffers(&device, &asset_manager, 10);

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            model_buffers,
            staging_belt,
            staging_capacity,
            num_vertices,
            num_indices,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, aspect_ratio: f32) {
        if new_size.width > 0 && new_size.height > 0 {
            let adjusted_size = self.adjust_aspect_ratio(new_size, aspect_ratio);

            self.size = adjusted_size;
            self.config.width = adjusted_size.width;
            self.config.height = adjusted_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn adjust_aspect_ratio(
        &self,
        new_size: PhysicalSize<u32>,
        aspect_ratio: f32,
    ) -> PhysicalSize<u32> {
        let new_width = new_size.width as f32;
        let new_height = new_size.height as f32;

        if new_width / new_height > aspect_ratio {
            // Width is too large, adjust the width
            let adjusted_width = (new_height * aspect_ratio).round() as u32;
            PhysicalSize::new(adjusted_width, new_size.height)
        } else {
            // Height is too large, adjust the height
            let adjusted_height = (new_width / aspect_ratio).round() as u32;
            PhysicalSize::new(new_size.width, adjusted_height)
        }
    }

    // TODO: Potential improvement combine vertex buffers with offsets so one buffer is re-used
    // for all vertex's
    // TODO: Investigate using vec2 instead of vec 3 with a seperate float for depth
    // TODO Investigate world space
    //
    pub fn render(
        &mut self,
        game: &mut ECS,
        assets: &mut AssetManager,
    ) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.update_instance_data(game, assets);

        self.update_buffer_capacity(assets);

        let mut model_buffs = Vec::with_capacity(assets.assets.len());
        let mut instance_lens = Vec::with_capacity(assets.assets.len());

        for renderable in assets.assets.iter() {
            let model_buff = self.model_buffers.get(&renderable.id).unwrap();
            let instances = assets.get_instance_data(&renderable.id);
            Self::write_staging_buff(
                instances,
                &model_buff.instance,
                &mut encoder,
                &mut self.staging_belt,
                &mut self.device,
            );

            model_buffs.push(model_buff);
            instance_lens.push(instances.len());
        }

        let mut render_pass = self.create_render_pass(&view, &mut encoder)?;
        render_pass.set_pipeline(&self.render_pipeline);

        for i in 0..model_buffs.len() {
            render_pass.set_vertex_buffer(0, model_buffs[i].vertex.slice(..));
            render_pass.set_index_buffer(model_buffs[i].index.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_vertex_buffer(1, model_buffs[i].instance.slice(..));
            render_pass.draw_indexed(0..self.num_indices, 0, 0..instance_lens[i] as _);
        }

        drop(render_pass);
        self.queue.submit(std::iter::once(encoder.finish()));
        self.staging_belt.recall();
        output.present();

        Ok(())
    }

    fn update_buffer_capacity<'a>(&mut self, asset_manager: &AssetManager) {
        for renderable in asset_manager.assets.iter() {
            let model_buff = self.model_buffers.get_mut(&renderable.id).unwrap();
            let instances = asset_manager.get_instance_data(&renderable.id);
            if instances.len() > model_buff.capacity {
                let new_capacity = instances.len() * 2;
                model_buff.increase_capacity(new_capacity, &self.device);
                if instances.len() > self.staging_capacity {
                    let new_staging_capacity = instances.len() * 2;
                    let new_staging_belt = self.create_bigger_staging_buffer(new_staging_capacity);
                    self.staging_belt = new_staging_belt;
                    self.staging_capacity = new_staging_capacity
                }
            }
        }
    }

    fn create_bigger_staging_buffer(&mut self, new_capacity: usize) -> StagingBelt {
        let new_belt = StagingBelt::new((InstanceRaw::size() * new_capacity) as u64);
        new_belt
    }

    fn write_staging_buff(
        instance_data: &[InstanceRaw],
        instance_buffer: &Buffer,
        encoder: &mut CommandEncoder,
        staging_belt: &mut StagingBelt,
        device: &Device,
    ) {
        if let Some(data_size) = NonZeroU64::new((InstanceRaw::size() * instance_data.len()) as u64)
        {
            let mut staging_buffer_view =
                staging_belt.write_buffer(encoder, instance_buffer, 0, data_size, device);
            let data = bytemuck::cast_slice(instance_data);
            staging_buffer_view.copy_from_slice(data);
            drop(staging_buffer_view);
            staging_belt.finish();
        }
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

    fn update_instance_data(&self, ecs: &mut ECS, assets_manager: &mut AssetManager) {
        for asset in assets_manager.assets.iter() {
            let model = assets_manager.instances.get_mut(&asset.id).unwrap();
            assert!(model.instances.len() == model.entity.len());
            assert!(model.instances.len() == model.stale.len());
            for i in 0..model.instances.len() {
                if *model.stale[i].inner() {
                    let instance_component = ecs.query::<Render>(model.entity[i]).unwrap();
                    model.instances[i] = instance_component.to_raw();
                    model.stale[i].finish();
                }
            }
        }
    }

    fn create_model_buffers(
        device: &Device,
        asset_manager: &AssetManager,
        capacity: usize,
    ) -> HashMap<TypeId, ModelBuffer> {
        let mut buffers = HashMap::new();

        for renderable in asset_manager.assets.iter() {
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
                size: (InstanceRaw::size() * capacity) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            buffers.insert(
                renderable.id,
                ModelBuffer::new(vertex_buffer, index_buffer, instance_buffer, capacity),
            );
        }
        buffers
    }

    fn create_render_pipeline(
        device: &Device,
        shader: &ShaderModule,
        config: &SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
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
