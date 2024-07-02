use std::{
    any::TypeId,
    sync::Arc,
    time::{Duration, Instant},
};

use winit::{
    dpi::LogicalSize,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

use crate::{
    ecs::{Component, ECS},
    render::{
        asset_manager::{AssetManager, Model},
        RenderState,
    },
    systems::{move_system, spawn_edible},
};

use super::app_state::AppState;

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

trait System<In, Out> {
    // fn call(input: Query<T>) -> Out;
}

pub struct Query<'world, 'query> {
    ecs: &'query ECS<'world>,
}

impl<'w, 'q> Query<'w, 'q> {
    pub fn new(ecs: &'w ECS) -> Self {
        Self { ecs }
    }

    pub fn get<T>(self) -> impl QueryData
    where
        T: QueryData,
    {
        self.ecs.get_component::<T>().unwrap()
    }
}

// trait Query<T>
// where
//     T: QueryData,
// {
//     fn fetch_data(ecs: ECS) -> T;
// }

// Vec of System -> loop through -> fetch queryData -> call function -> pass query data to function
// -> run the function
//

trait QueryData {}

// impl<T: QueryData> Component for &T {}

impl<T: Component> QueryData for &T {}

pub struct Scheduler {
    pub ticks_per_second: u64,
    pub tick_duration: Duration,
    pub dt: Duration,
    pub last_tick_time: Instant,
    pub accumulated_time: Duration,
    // pub systems: Vec<Into<System>>,
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

pub struct App<'a> {
    pub window: Arc<Window>,
    pub config: Config,
    pub ecs: ECS<'a>,
    pub assets: AssetManager,
    pub runner: Scheduler,
    pub render_state: RenderState<'a>,
}

impl<'a> App<'a> {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        let config = Config::default();
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_inner_size(LogicalSize::new(config.width, config.height)),
                )
                .unwrap(),
        );
        let assets = AssetManager::new();
        let render_state = RenderState::new(window.clone());
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

    pub fn register_asset<T: Into<Model> + 'static>(&mut self, model: T) {
        let id = TypeId::of::<T>();
        let mut new_model: Model = model.into();
        new_model.adjust_for_aspect_ratio(self.config.aspect_ratio);

        self.render_state.register_new_buffer(10, &new_model, id);
        self.assets.register(new_model, id);
    }

    pub fn add_system<T: FnMut(ECS) + 'static>(&mut self, system: T) {
        self.runner.systems.push(Box::new(system));
    }

    pub fn run<I: FnMut(&mut App)>(init: I) {
        let events: EventLoop<()> = winit::event_loop::EventLoop::new().unwrap();
        let mut state = AppState::Unitialised { init };
        events.run_app(&mut state).unwrap();
    }

    pub fn update(&mut self) {
        self.runner.tick(|| {
            move_system(&mut self.ecs, &mut self.assets);
            spawn_edible(&mut self.ecs, &mut self.assets);
        });
    }
}
