use std::any::TypeId;

use bundle_derive::Bundle;
use rand::Rng;

use crate::{
    components::{Edible, Gravity, InputState, Physics, Position, Render, Size, Velocity},
    ecs::{Bundle, Bundles, ComponentStorage, Entity},
};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub position: Position,
    pub size: Size,
    pub input: InputState,
    pub physics: Physics,
    pub velocity: Velocity,
    pub grav: Gravity,
    pub render: Render,
}

impl PlayerBundle {
    pub fn new(x: i32, y: i32, size: u32, t: f32, b: f32, scale: f32) -> Self {
        Self {
            position: Position::new(x, y),
            size: Size::new(size),
            input: InputState::new(),
            physics: Physics { speed: 0.05 },
            velocity: Velocity { vx: 0, vy: 0 },
            grav: Gravity {
                gx: 0.0,
                gy: 9.81 / 10.0,
            },
            render: Render::new(t, b, 0.0, scale),
        }
    }
}

#[derive(Bundle)]
pub struct FoodBundle {
    pub size: Size,
    pub position: Position,
    pub edible: Edible,
    pub velocity: Velocity,
    pub grav: Gravity,
    pub render: Render,
}

impl FoodBundle {
    pub fn new() -> Self {
        let x = rand::thread_rng().gen_range(-1.0..1.0);
        let y = rand::thread_rng().gen_range(-1.0..1.0);

        Self {
            size: Size { size: 10 },
            position: Position::random(),
            edible: Edible {
                eaten: false,
                calories: 10,
            },
            velocity: Velocity { vx: 0, vy: 0 },
            grav: Gravity {
                gx: 0.0,
                gy: 9.81 / 10.0,
            },
            render: Render::new(x, y, 0.0, 0.1),
        }
    }
}
