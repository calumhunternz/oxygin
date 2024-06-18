use crate::{
    components::{Gravity, Position, Velocity},
    ecs::{Entity, ECS},
    resources::Player,
};

pub fn gravity(game: &mut ECS) {
    update_velocity(game);

    update_position(game);
}

fn update_position(game: &mut ECS) {
    let player = game.resources.get::<Player>().unwrap().inner();
    let affected = get_vel(game);
    let position = game.get_mut_component::<Position>().unwrap();
    let dt = 2;

    for vel in affected {
        let v = position.get_mut(vel.0).unwrap();
        if v.y >= 1080 - 50 {
            v.y = 0;
            continue;
        }
        v.y = v.y + (vel.1.vy * dt);

        dbg!(&v);
    }
}

fn update_velocity(game: &mut ECS) {
    let affected = get_grav(&game);

    let velocity = game.get_mut_component::<Velocity>().unwrap();
    let dt = 2;

    for grav in affected {
        let v = velocity.get_mut(grav.0).unwrap();
        if v.vy > 20 {
            v.vy = 20;
            continue;
        }
        v.vy = ((v.vy as f32) + (grav.1.gy * dt as f32)) as i32;
        dbg!(&v);
    }
}

fn get_vel(game: &ECS) -> Vec<(Entity, Velocity)> {
    let velocity = game.get_component::<Velocity>().unwrap();
    let mut affected = vec![];
    for vel in velocity {
        affected.push((vel.0, vel.1.clone()));
    }
    affected
}

fn get_grav(game: &ECS) -> Vec<(Entity, Gravity)> {
    let gravity = game.get_component::<Gravity>().unwrap();
    let mut affected = vec![];
    for grav in gravity {
        affected.push((grav.0, grav.1.clone()));
    }
    affected
}
