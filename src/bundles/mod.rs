use std::any::TypeId;

use bundle_derive::Bundle;

use crate::{
    components::{ColorComponent, Edible, InputState, Physics, Position, Size},
    ecs::{Bundle, Bundles, ComponentStorage, Entity},
};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub position: Position,
    pub size: Size,
    pub input: InputState,
    pub color: ColorComponent,
    pub physics: Physics,
}

impl PlayerBundle {
    pub fn new(x: i32, y: i32, size: u32) -> Self {
        Self {
            position: Position::new(x, y),
            size: Size::new(size),
            input: InputState::new(),
            color: ColorComponent::new(255, 255, 255),
            physics: Physics { speed: 10 },
        }
    }
}

#[derive(Bundle)]
pub struct FoodBundle {
    pub size: Size,
    pub position: Position,
    pub color: ColorComponent,
    pub edible: Edible,
}

impl FoodBundle {
    pub fn new() -> Self {
        Self {
            size: Size { size: 10 },
            position: Position::random(),
            color: ColorComponent::new(0, 255, 0),
            edible: Edible {
                eaten: false,
                calories: 10,
            },
        }
    }
}
