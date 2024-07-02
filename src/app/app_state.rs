use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop};

use crate::systems::handle_input_system;

use super::app::App;

pub enum AppState<'a, I: FnOnce(&mut App)> {
    Initialised(App<'a>),
    Unitialised { init: I },
}

impl<'a, I: FnMut(&mut App)> AppState<'a, I> {
    pub fn init(&mut self, event_loop: &ActiveEventLoop) {
        *self = match self {
            Self::Initialised(_app) => panic!(),
            Self::Unitialised { init } => {
                let mut app = App::new(event_loop);
                init(&mut app);
                Self::Initialised(app)
            }
        }
    }
}

impl<'a, I: FnMut(&mut App)> ApplicationHandler for AppState<'a, I> {
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

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match self {
            AppState::Unitialised { .. } => self.init(event_loop),
            AppState::Initialised(_app) => (),
        }
    }

    fn new_events(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _cause: winit::event::StartCause,
    ) {
        match self {
            AppState::Unitialised { .. } => self.init(event_loop),
            AppState::Initialised(_app) => (),
        }
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, _event: ()) {
        match self {
            AppState::Unitialised { .. } => self.init(event_loop),
            AppState::Initialised(_app) => (),
        }
    }

    fn device_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        _event: winit::event::DeviceEvent,
    ) {
        match self {
            AppState::Unitialised { .. } => self.init(event_loop),
            AppState::Initialised(_app) => (),
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
            AppState::Initialised(_app) => (),
        }
    }

    fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match self {
            AppState::Unitialised { .. } => self.init(event_loop),
            AppState::Initialised(_app) => (),
        }
    }

    fn memory_warning(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match self {
            AppState::Unitialised { .. } => self.init(event_loop),
            AppState::Initialised(_app) => (),
        }
    }
}
