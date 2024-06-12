use crate::{
    components::{InputState, Physics, Position},
    ecs::{Entity, ECS},
    resources::Player,
};

pub fn move_system(game: &mut ECS) {
    let player = game.resources.get::<Player>().unwrap().inner();
    let (input, speed) = gather_data(player, game);
    let position_component = game.get_mut_component::<Position>().unwrap();

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

pub fn gather_data(player: Entity, game: &ECS) -> (InputState, Physics) {
    let input_component = game.get_component::<InputState>().unwrap();
    let physics_component = game.get_component::<Physics>().unwrap();
    (
        input_component.get(player).unwrap().clone(),
        physics_component.get(player).unwrap().clone(),
    )
}
