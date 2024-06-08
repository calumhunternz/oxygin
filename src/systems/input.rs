use sdl2::keyboard::{KeyboardState, Scancode};

use crate::{components::InputState, ecs::ECS, resources::Player};

pub fn handle_input_system(keyboard: KeyboardState, game: &mut ECS) {
    let mut input_component = game.get_mut_component::<InputState>().unwrap();
    let player = game.resources.get::<Player>().unwrap().inner();

    let input = input_component.get_mut(player).unwrap();

    input.up = keyboard.is_scancode_pressed(Scancode::Up);
    input.right = keyboard.is_scancode_pressed(Scancode::Right);
    input.down = keyboard.is_scancode_pressed(Scancode::Down);
    input.left = keyboard.is_scancode_pressed(Scancode::Left);
}
