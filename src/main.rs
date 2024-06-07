extern crate sdl2;

use oxygin::eat::eat_system;
use oxygin::ecs::ECS;
use oxygin::input::handle_input_system;
use oxygin::movement::move_system;
use oxygin::render::render_system;
use oxygin::{ColorComponent, Edible, InputState, Physics, Position, Size};
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

// Have a way to get the player entity more easily
// Add components to entities (if only syntactically) eg player.add_component<Position>(position)
// maybe chainable
// Break structure project to seperate out engine
// Traces within the ECS rather than unwraps
// Investigate component / resource register
// Investigate plugins
// investigate queries

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
    game.register_component::<Position>();
    game.register_component::<Size>();
    game.register_component::<InputState>();
    game.register_component::<Physics>();
    game.register_component::<ColorComponent>();
    game.register_component::<Edible>();

    let player = game.create_entity();
    game.add_component(player, Size { size: 20 });
    game.add_component(player, Position { x: 400, y: 400 });
    game.add_component(player, InputState::new());
    game.add_component(player, Physics { speed: 10 });
    game.add_component(player, ColorComponent::new(255, 255, 255));

    let food = game.create_entity();
    game.add_component(food, Size { size: 10 });
    game.add_component(food, Position::random());
    game.add_component(food, ColorComponent::new(0, 255, 0));
    game.add_component(
        food,
        Edible {
            eaten: false,
            calories: 10,
        },
    );

    let drug = game.create_entity();
    game.add_component(drug, Size { size: 10 });
    game.add_component(drug, Position::random());
    game.add_component(drug, ColorComponent::new(0, 0, 255));
    game.add_component(
        drug,
        Edible {
            eaten: false,
            calories: 100,
        },
    );
    let drink = game.create_entity();
    game.add_component(drink, Size { size: 10 });
    game.add_component(drink, Position::random());
    game.add_component(drink, ColorComponent::new(255, 0, 0));
    game.add_component(
        drink,
        Edible {
            eaten: false,
            calories: 50,
        },
    );

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    loop {
        canvas.clear();

        handle_input_system(event_pump.keyboard_state(), &mut game);

        move_system(&mut game);
        eat_system(&mut game);

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
