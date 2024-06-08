use crate::{
    components::{InputState, Physics, Position},
    resources::Player,
    ECS,
};

pub fn move_system(game: &mut ECS) {
    let player = game.resources.get::<Player>().unwrap().inner();
    let input_component = game.get_component::<InputState>().unwrap();
    let physics_component = game.get_component::<Physics>().unwrap();

    let mut position_component = game.get_mut_component::<Position>().unwrap();

    let input = input_component.get(player).unwrap();
    let speed = physics_component.get(player).unwrap();

    if let Some(player) = position_component.get_mut(player) {
        if input.up {
            player.y = player.y - speed.speed;
        }
        if input.right {
            player.x = player.x + speed.speed;
        }
        if input.down {
            player.y = player.y + speed.speed;
        }
        if input.left {
            player.x = player.x - speed.speed;
        }
    }
}
