use nalgebra::{Matrix4, Vector3};
use rand::Rng;
use sdl2::pixels::Color;

use crate::{ecs::Component, render::InstanceRaw};

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Velocity {
    pub vx: i32,
    pub vy: i32,
}

impl Velocity {
    pub fn new(vx: i32, vy: i32) -> Self {
        Self { vx, vy }
    }
}

impl Component for Velocity {}

#[derive(Clone)]
pub struct Gravity {
    pub gx: f32,
    pub gy: f32,
}

impl Gravity {
    pub fn new(gx: f32, gy: f32) -> Self {
        Self { gx, gy }
    }
}

impl Component for Gravity {}

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

#[derive(Clone, Debug)]
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
    pub speed: f32,
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

#[derive(Clone, Debug)]
pub struct Render {
    pub transform: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl Render {
    pub fn new(x: f32, y: f32, z: f32, scale: f32) -> Self {
        Self {
            transform: Vector3::new(x, y, z),
            scale: Vector3::new(scale, scale, 1.0),
        }
    }

    pub fn to_raw(&self) -> InstanceRaw {
        let transform = Matrix4::new_translation(&self.transform);
        let scale = Matrix4::new_nonuniform_scaling(&self.scale);
        InstanceRaw {
            model: (transform * scale).into(),
        }
    }
}

impl Component for Render {}
