extern crate sdl2;

use oxygin::bundles::{FoodBundle, PlayerBundle};
use oxygin::ecs::ECS;
use oxygin::resources::Player;
use oxygin::systems::{eat_system, handle_input_system, move_system, render_system, spawn_edible};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::EventPump;
use std::time::Duration;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

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

fn main() {
    const RENDER_NORMALIZATION: u32 = 1_000_000_000u32 / 60;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .opengl()
        .borderless()
        .fullscreen_desktop()
        .build()
        .unwrap();

    dbg!(window.size());

    let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    let mut game = ECS::new();

    game.register_bundle::<FoodBundle>();
    game.register_bundle::<PlayerBundle>();

    let player = game.add_bundle(PlayerBundle::new(400, 400, 50)).unwrap();
    game.add_resource(Player::new(&player));

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut count: u32 = 0;
    let mut event_pump = sdl_context.event_pump().unwrap();
    loop {
        canvas.clear();

        handle_input_system(event_pump.keyboard_state(), &mut game);

        move_system(&mut game);

        eat_system(&mut game);

        spawn_edible(&mut game, &mut count);

        render_system(&game, &mut canvas);

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        canvas.present();

        ::std::thread::sleep(Duration::new(0, RENDER_NORMALIZATION));
        if !handle_events(&mut event_pump) {
            break;
        }
    }
}

fn handle_events(event_pump: &mut EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => return false,
            _ => return true,
        }
    }
    true
}
