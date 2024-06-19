use wgpu::{
    util::DeviceExt, Adapter, Backends, BindGroup, BindGroupLayout, Buffer, CommandEncoder, Device,
    DeviceDescriptor, Features, InstanceDescriptor, PipelineCompilationOptions, Queue, RenderPass,
    RequestAdapterOptions, ShaderModule, Surface, SurfaceCapabilities, SurfaceConfiguration,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{components::Render, ecs::ECS};

use super::{Instance, InstanceRaw, Uniforms, Vertex, INDICES, VERTICES};

pub struct RenderState<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: &'a Window,
    pub render_pipeline: wgpu::RenderPipeline,
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
    pub fn new(window: &'a Window) -> RenderState<'a> {
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

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer: None,
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

    pub fn render(&mut self, game: &ECS) -> Result<(), wgpu::SurfaceError> {
        let vertex_buffer = self.create_vertex_buffer(game);
        let index_buffer = self.create_index_buffer(game);
        let instances = self.get_instance_data(game);
        let instance_buffer = self.create_instance_buffer(&instances);

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        let mut render_pass = self.create_render_pass(&view, &mut encoder)?;
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..instances.len() as _);
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

    fn get_instance_data(&self, ecs: &ECS) -> Vec<InstanceRaw> {
        let instances = ecs
            .get_component::<Render>()
            .unwrap()
            .iter()
            .map(|(_key, val)| Instance {
                position: val.transform,
                scale: val.scale,
            })
            .collect::<Vec<_>>();

        instances.iter().map(Instance::to_raw).collect::<Vec<_>>()
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

    fn create_instance_buffer(&self, instances: &Vec<InstanceRaw>) -> Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(instances),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            })
    }

    fn create_vertex_buffer(&self, ecs: &ECS) -> Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&ecs.vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            })
    }

    fn create_index_buffer(&self, ecs: &ECS) -> Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(ecs.indices),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            })
    }

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
                entry_point: "vs_main", // 1.
                buffers: &[Vertex::desc(), InstanceRaw::desc()],
                compilation_options: PipelineCompilationOptions::default(), // 2.
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
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
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
                None, // Trace path
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
