use std::time::{Duration, Instant};

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{
    ecs::ECS,
    render::{asset_manager::AssetManager, RenderState},
    systems::{handle_input_system, move_system, spawn_edible},
};

pub struct Config {
    pub aspect_ratio: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            width: 1500.0,
            height: 1500.0 / (16.0 / 9.0),
        }
    }
}

pub struct App<'a> {
    pub config: Config,
    pub ecs: ECS<'a>,
    pub assets: AssetManager,
    runner: Scheduler,
}

pub struct Scheduler {
    pub ticks_per_second: u64,
    pub tick_duration: Duration,
    pub dt: Duration,
    pub last_tick_time: Instant,
    pub accumulated_time: Duration,
}

impl Scheduler {
    pub fn new() -> Self {
        let ticks_per_second = 60;
        Self {
            ticks_per_second,
            tick_duration: Duration::from_secs_f64(1.0 / ticks_per_second as f64),
            last_tick_time: Instant::now(),
            accumulated_time: Duration::ZERO,
            dt: Duration::ZERO,
        }
    }
    pub fn tick<T>(&mut self, mut update: T)
    where
        T: FnMut(),
    {
        let now = Instant::now();
        self.dt = now - self.last_tick_time;
        self.last_tick_time = now;
        self.accumulated_time += self.dt;
        while self.accumulated_time >= self.tick_duration {
            update();
            self.accumulated_time -= self.tick_duration;
        }
    }
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self {
            ecs: ECS::new(),
            runner: Scheduler::new(),
            assets: AssetManager::new(16.0 / 9.0),
            config: Config::default(),
        }
    }

    pub fn init(&mut self, init_function: impl FnOnce(&mut Self)) {
        init_function(self);
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(self.config.width, self.config.height))
            .build(&event_loop)
            .unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut state = RenderState::new(&window, &mut self.assets);
        event_loop
            .run(move |event, control_flow| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window().id() => match event {
                    WindowEvent::CloseRequested => control_flow.exit(),

                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size, self.config.aspect_ratio);
                    }

                    WindowEvent::KeyboardInput { event, .. } => {
                        handle_input_system(&event, &mut self.ecs);
                    }

                    WindowEvent::RedrawRequested => {
                        state.window().request_redraw();

                        self.update();

                        match state.render(&mut self.ecs, &mut self.assets) {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                state.resize(state.size, self.config.aspect_ratio)
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => control_flow.exit(),
                            Err(wgpu::SurfaceError::Timeout) => {}
                        }
                    }
                    _ => {}
                },
                _ => {}
            })
            .unwrap();
    }

    fn update(&mut self) {
        self.runner.tick(|| {
            move_system(&mut self.ecs, &mut self.assets);
            spawn_edible(&mut self.ecs, &mut self.assets);
        });
    }
}
