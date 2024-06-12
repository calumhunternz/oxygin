use rand::Rng;
use sdl2::pixels::Color;

use crate::ecs::Component;

#[derive(Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn random() -> Self {
        Self {
            x: rand::thread_rng().gen_range(1..1920),
            y: rand::thread_rng().gen_range(1..1080),
        }
    }
}

impl Component for Position {}

#[derive(Clone)]
pub struct Size {
    pub size: u32,
}

impl Size {
    pub fn new(size: u32) -> Self {
        Self { size }
    }
}

impl Component for Size {}

#[derive(Clone)]
pub struct InputState {
    pub up: bool,
    pub right: bool,
    pub down: bool,
    pub left: bool,
    pub space: bool,
}

impl Component for InputState {}

impl InputState {
    pub fn new() -> Self {
        Self {
            up: false,
            right: false,
            down: false,
            left: false,
            space: false,
        }
    }
}

#[derive(Clone)]
pub struct Physics {
    pub speed: i32,
}

impl Component for Physics {}

#[derive(Clone)]
pub struct ColorComponent {
    pub rgb: Color,
}
impl ColorComponent {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            rgb: Color::RGB(r, g, b),
        }
    }
}

impl Component for ColorComponent {}

#[derive(Clone)]
pub struct Edible {
    pub eaten: bool,
    pub calories: u32,
}

impl Component for Edible {}
