use sdl2::keyboard::{KeyboardState, Scancode};

use crate::{InputState, ECS};

pub fn handle_input_system(keyboard: KeyboardState, game: &mut ECS) {
    let mut input_component = game.get_mut_component::<InputState>().unwrap();

    for input in input_component.values_mut() {
        input.up = keyboard.is_scancode_pressed(Scancode::Up);
        input.right = keyboard.is_scancode_pressed(Scancode::Right);
        input.down = keyboard.is_scancode_pressed(Scancode::Down);
        input.left = keyboard.is_scancode_pressed(Scancode::Left);
    }
}
