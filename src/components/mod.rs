use rand::Rng;
use sdl2::pixels::Color;

use crate::Component;

pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn random() -> Self {
        Self {
            x: rand::thread_rng().gen_range(1..1920),
            y: rand::thread_rng().gen_range(1..1080),
        }
    }
}

impl Component for Position {}

pub struct Size {
    pub size: u32,
}

impl Component for Size {}

pub struct InputState {
    pub up: bool,
    pub right: bool,
    pub down: bool,
    pub left: bool,
}

impl Component for InputState {}

impl InputState {
    pub fn new() -> Self {
        Self {
            up: false,
            right: false,
            down: false,
            left: false,
        }
    }
}

pub struct Physics {
    pub speed: i32,
}

impl Component for Physics {}

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

pub struct Edible {
    pub eaten: bool,
    pub calories: u32,
}

impl Component for Edible {}
