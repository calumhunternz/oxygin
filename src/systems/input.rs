// use sdl2::keyboard::{KeyboardState, Scancode};
use winit::{
    event::{ElementState, KeyEvent},
    keyboard::{Key, NamedKey},
    platform::modifier_supplement::KeyEventExtModifierSupplement,
};

use crate::{components::InputState, ecs::ECS, resources::Player};

pub fn handle_input_system(event: &KeyEvent, game: &mut ECS) {
    let player = game.resources.get::<Player>().unwrap().into();
    let input_component = game.get_mut_component::<InputState>().unwrap();

    let input = input_component.get_mut(player).unwrap();

    if event.state == ElementState::Pressed {
        match event.key_without_modifiers().as_ref() {
            Key::Named(NamedKey::ArrowUp) => {
                input.up = true;
            }
            Key::Named(NamedKey::ArrowRight) => {
                input.right = true;
            }
            Key::Named(NamedKey::ArrowDown) => {
                input.down = true;
            }
            Key::Named(NamedKey::ArrowLeft) => {
                input.left = true;
            }
            _ => (),
        }
    }
    if event.state == ElementState::Pressed
        && event.key_without_modifiers().as_ref() == Key::Named(NamedKey::Space)
        && !event.repeat
    {
        input.space = true;
    }
    if event.state == ElementState::Released {
        match event.key_without_modifiers().as_ref() {
            Key::Named(NamedKey::ArrowUp) => {
                input.up = false;
            }
            Key::Named(NamedKey::ArrowRight) => {
                input.right = false;
            }
            Key::Named(NamedKey::ArrowDown) => {
                input.down = false;
            }
            Key::Named(NamedKey::ArrowLeft) => {
                input.left = false;
            }
            Key::Named(NamedKey::Space) => {
                input.space = false;
            }
            _ => (),
        }
    }
    //
    // input.right = keyboard.is_scancode_pressed(Scancode::Right);
    // input.down = keyboard.is_scancode_pressed(Scancode::Down);
    // input.left = keyboard.is_scancode_pressed(Scancode::Left);
    // input.space = keyboard.is_scancode_pressed(Scancode::Space);
}
