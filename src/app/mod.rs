use std::{
    any::TypeId,
    sync::Arc,
    time::{Duration, Instant},
};

use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{self, ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes},
};

use crate::{
    bundles::{FoodBundle, PlayerBundle},
    ecs::{Entity, ECS},
    render::{
        asset_manager::{AssetManager, Food, Model, Square},
        RenderState,
    },
    resources::Player,
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
    pub window: Arc<Window>,
    pub config: Config,
    pub ecs: ECS<'a>,
    pub assets: AssetManager,
    runner: Scheduler,
    render_state: RenderState<'a>,
}

pub enum AppState<'a> {
    Initialised(App<'a>),
    Unitialised,
}

impl<'a> AppState<'a> {
    pub fn init(&mut self, event_loop: &ActiveEventLoop) {
        *self = match self {
            Self::Initialised(_app) => panic!(),
            Self::Unitialised => {
                let mut app = App::new(event_loop);

                app.ecs.register_bundle::<FoodBundle>();
                app.ecs.register_bundle::<PlayerBundle>();

                let player = app
                    .ecs
                    .add_bundle(PlayerBundle::new(400, 400, 50, 0.5, 0.5, 0.5))
                    .unwrap();
                app.ecs.add_resource(Player::new(&player));
                let square = Square::new();
                let square2 = Food::new();
                app.register_asset(square);
                app.register_asset(square2);
                // app.assets.register(square);
                // app.assets.register(square2);
                app.assets.add_asset::<Square>(player);
                Self::Initialised(app)
            }
        }
    }
}

impl<'a> ApplicationHandler for AppState<'a> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match self {
            AppState::Unitialised { .. } => self.init(event_loop),
            AppState::Initialised(app) => (),
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let app = match self {
            AppState::Unitialised { .. } => {
                self.init(event_loop);
                return;
            }
            AppState::Initialised(app) => app,
        };

        if app.window.id() != window_id {
            return;
        }
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(physical_size) => {
                app.render_state
                    .resize(physical_size, app.config.aspect_ratio);
            }

            WindowEvent::KeyboardInput { event, .. } => {
                handle_input_system(&event, &mut app.ecs);
            }

            WindowEvent::RedrawRequested => {
                app.window.request_redraw();
                app.update();

                match app.render_state.render(&mut app.ecs, &mut app.assets) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => app
                        .render_state
                        .resize(app.render_state.size, app.config.aspect_ratio),
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    Err(wgpu::SurfaceError::Timeout) => {}
                }
            }
            _ => {}
        };
    }

    fn new_events(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        match self {
            AppState::Unitialised { .. } => self.init(event_loop),
            AppState::Initialised(app) => (),
        }
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: ()) {
        match self {
            AppState::Unitialised { .. } => self.init(event_loop),
            AppState::Initialised(app) => (),
        }
    }

    fn device_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        match self {
            AppState::Unitialised { .. } => self.init(event_loop),
            AppState::Initialised(app) => (),
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match self {
            AppState::Unitialised { .. } => self.init(event_loop),
            AppState::Initialised(app) => app.window.request_redraw(),
        }
    }

    fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match self {
            AppState::Unitialised { .. } => self.init(event_loop),
            AppState::Initialised(app) => (),
        }
    }

    fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match self {
            AppState::Unitialised { .. } => self.init(event_loop),
            AppState::Initialised(app) => (),
        }
    }

    fn memory_warning(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match self {
            AppState::Unitialised { .. } => self.init(event_loop),
            AppState::Initialised(app) => (),
        }
    }
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
    pub fn new(event_loop: &event_loop::ActiveEventLoop) -> Self {
        let config = Config::default();
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_inner_size(LogicalSize::new(config.width, config.height)),
                )
                .unwrap(),
        );
        let assets = AssetManager::new(16.0 / 9.0);
        let render_state = RenderState::new(window.clone(), &assets);
        Self {
            window,
            ecs: ECS::new(),
            runner: Scheduler::new(),
            render_state,
            assets,
            config: Config::default(),
        }
    }

    pub fn init(&mut self, mut init_function: impl FnMut(&mut Self)) {
        init_function(self);
    }

    pub fn register_asset<T: Into<Model> + Clone + 'static>(&mut self, model: T) {
        let id = TypeId::of::<T>();
        let mut new_model = model.into();
        new_model.adjust_for_aspect_ratio(self.config.aspect_ratio);

        self.render_state.register_new_buffer(10, &new_model, id);
        self.assets.register(new_model, id);
    }

    pub fn run() {
        let events: EventLoop<()> = winit::event_loop::EventLoop::new().unwrap();
        let mut state = AppState::Unitialised;
        events.run_app(&mut state).unwrap();
        // let event_loop = EventLoop::new().unwrap();
        // let window = WindowBuilder::new()
        // .with_inner_size(LogicalSize::new(self.config.width, self.config.height))
        // .build(&event_loop)
        // .unwrap();
        // event_loop.set_control_flow(ControlFlow::Poll);
        //
        // let mut state = RenderState::new(&window, &mut self.assets);
        // event_loop
        //     .run(move |event, control_flow| match event {
        //         Event::WindowEvent {
        //             ref event,
        //             window_id,
        //         } if window_id == state.window().id() => match event {
        //             WindowEvent::CloseRequested => control_flow.exit(),
        //
        //             WindowEvent::Resized(physical_size) => {
        //                 state.resize(*physical_size, self.config.aspect_ratio);
        //             }
        //
        //             WindowEvent::KeyboardInput { event, .. } => {
        //                 handle_input_system(&event, &mut self.ecs);
        //             }
        //
        //             WindowEvent::RedrawRequested => {
        //                 state.window().request_redraw();
        //
        //                 self.update();
        //
        //                 match state.render(&mut self.ecs, &mut self.assets) {
        //                     Ok(_) => {}
        //                     Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
        //                         state.resize(state.size, self.config.aspect_ratio)
        //                     }
        //                     Err(wgpu::SurfaceError::OutOfMemory) => control_flow.exit(),
        //                     Err(wgpu::SurfaceError::Timeout) => {}
        //                 }
        //             }
        //             _ => {}
        //         },
        //         _ => {}
        //     })
        //     .unwrap();
    }

    // pub fn resize(&mut self, physical_size: PhysicalSize<u32>) {
    //     self.render_state
    //         .resize(physical_size, self.config.aspect_ratio);
    // }
    //
    // pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    //     self.render_state.render(&mut self.ecs, &mut self.assets)?;
    //     Ok(())
    // }

    pub fn update(&mut self) {
        self.runner.tick(|| {
            move_system(&mut self.ecs, &mut self.assets);
            spawn_edible(&mut self.ecs, &mut self.assets);
        });
    }
}
