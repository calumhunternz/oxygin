// extern crate sdl2;
use nalgebra::{Matrix4, Quaternion, SquareMatrix, Vector3};
use oxygin::bundles::{FoodBundle, PlayerBundle};
use oxygin::components::InputState;
use oxygin::ecs::ECS;
use oxygin::render::RenderState;
use oxygin::resources::Player;
use oxygin::systems::{
    eat_system, gravity, handle_input_system, move_system, render_system, spawn_edible,
};
use wgpu::util::DeviceExt;
use wgpu::{
    Backends, Device, DeviceDescriptor, Features, InstanceDescriptor, PipelineCompilationOptions,
    Queue, RequestAdapterOptions, Surface, SurfaceConfiguration,
};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::keyboard::{KeyCode, PhysicalKey};
// use sdl2::event::Event;
// use sdl2::pixels::Color;
// use sdl2::EventPump;
use std::time::{Duration, Instant};
use winit::event::{
    DeviceEvent, ElementState, Event, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent,
};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

// fn find_sdl_gl_driver() -> Option<u32> {
//     for (index, item) in sdl2::render::drivers().enumerate() {
//         if item.name == "opengl" {
//             return Some(index as u32);
//         }
//     }
//     None
// }

// TO IMPROVE
// I want to remove the call to get ref then get component DONE!!!!!
// Have a way to get the player entity more easily DONE!!! through resource storage
// Break structure project to seperate out engine DONE
// Add components to entities (if only syntactically) previously skill issued but now done
// Remove refcell from component storage (Skilled issued -> Now fookin done (with anymap)
//
// maybe chainable (Im thinking plugins for this)
// Traces within the ECS rather than unwraps
//
// investigate queries previous attempt go skill issued
// Investigate component / resource register
// Investigate plugins
//

async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut state = RenderState::new(&window).await;
    let mut surface_configured = false;

    let mut game = ECS::new();

    game.register_bundle::<FoodBundle>();
    game.register_bundle::<PlayerBundle>();

    let player = game
        .add_bundle(PlayerBundle::new(400, 400, 50, 0.5, 0.5, 0.5))
        .unwrap();
    game.add_resource(Player::new(&player));

    let mut count = 0;

    let mut last_tick_time = Instant::now();
    let mut accumulated_time = Duration::ZERO;
    let TICKS_PER_SECOND: u64 = 60;
    let TICK_DURATION: Duration = Duration::from_secs_f64(1.0 / TICKS_PER_SECOND as f64);
    event_loop
        .run(move |event, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window().id() => {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                                    ..
                                },
                            ..
                        } => control_flow.exit(),
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::KeyboardInput { event, .. } => {
                            handle_input_system(&event, &mut game);
                        }
                        WindowEvent::RedrawRequested if window_id == state.window().id() => {
                            state.window().request_redraw();
                            let now = Instant::now();
                            let dt = now - last_tick_time;
                            last_tick_time = now;
                            accumulated_time += dt;
                            while accumulated_time >= TICK_DURATION {
                                move_system(&mut game);
                                spawn_edible(&mut game, &mut count);
                                accumulated_time -= TICK_DURATION;
                            }
                            match state.render(&game) {
                                Ok(_) => {}
                                // Reconfigure the surface if it's lost or outdated
                                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                    state.resize(state.size)
                                }
                                // The system is out of memory, we should probably quit
                                Err(wgpu::SurfaceError::OutOfMemory) => {
                                    // log::error!("w");
                                    control_flow.exit();
                                }

                                // This happens when the a frame takes too long to present
                                Err(wgpu::SurfaceError::Timeout) => {
                                    // log::warn!("Surface timeout")
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .unwrap();
}

fn main() {
    pollster::block_on(run());
    // let app = App::default();
    // let event_loop = EventLoop::new().unwrap();
    // event_loop.set_control_flow(ControlFlow::Poll);

    let TICKS_PER_SECOND: u64 = 60;
    let TICK_DURATION: Duration = Duration::from_secs_f64(1.0 / TICKS_PER_SECOND as f64);
    let mut last_tick_time = Instant::now();
    let mut accumulated_time = Duration::ZERO;
    let mut count: u32 = 0;

    //     let sdl_context = sdl2::init().unwrap();
    //     let video_subsystem = sdl_context.video().unwrap();
    //     let window = video_subsystem
    //         .window("rust-sdl2 demo", 800, 600)
    //         .opengl()
    //         .borderless()
    //         .fullscreen_desktop()
    //         .build()
    //         .unwrap();
    //
    //     dbg!(window.size());
    //
    //     let mut canvas = window
    //         .into_canvas()
    //         .index(find_sdl_gl_driver().unwrap())
    //         .build()
    //         .unwrap();
    //
    //     let mut game = ECS::new();
    //
    //     game.register_bundle::<FoodBundle>();
    //     game.register_bundle::<PlayerBundle>();
    //
    //     let player = game.add_bundle(PlayerBundle::new(400, 400, 50)).unwrap();
    //     game.add_resource(Player::new(&player));
    //
    //     // let game = App::new().register_bundle::<Bundle>().register_system().register_plugin.run()
    //
    //     canvas.set_draw_color(Color::RGB(0, 255, 255));
    //     canvas.clear();
    //     canvas.present();
    //     let mut count: u32 = 0;
    //     let mut event_pump = sdl_context.event_pump().unwrap();
    //     let TICKS_PER_SECOND: u64 = 60;
    //     let TICK_DURATION: Duration = Duration::from_secs_f64(1.0 / TICKS_PER_SECOND as f64);
    //     let mut last_tick_time = Instant::now();
    //     let mut accumulated_time = Duration::ZERO;
    //
    //     loop {
    //         let now = Instant::now();
    //         let elapsed_time = now - last_tick_time;
    //         last_tick_time = now;
    //
    //         accumulated_time += elapsed_time;
    //
    //         if accumulated_time >= TICK_DURATION {
    //             handle_input_system(event_pump.keyboard_state(), &mut game);
    //
    //             move_system(&mut game);
    //
    //             eat_system(&mut game);
    //
    //             spawn_edible(&mut game, &mut count);
    //
    //             gravity(&mut game);
    //
    //             accumulated_time -= TICK_DURATION;
    //         }
    //
    //         canvas.clear();
    //         render_system(&game, &mut canvas);
    //
    //         canvas.set_draw_color(Color::RGB(0, 0, 0));
    //
    //         canvas.present();
    //
    //         // ::std::thread::sleep(Duration::new(0, RENDER_NORMALIZATION));
    //         if !handle_events(&mut event_pump) {
    //             break;
    //         }
    //     }
}
// fn handle_events(event_pump: &mut EventPump) -> bool {
//     for event in event_pump.poll_iter() {
//         match event {
//             Event::Quit { .. } => return false,
//             _ => return true,
//         }
//     }
//     true
// }
