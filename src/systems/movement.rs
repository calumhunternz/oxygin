use crate::{InputState, Physics, Position, ECS};

pub fn move_system(game: &mut ECS) {
    let input_component = game.get_component::<InputState>().unwrap();

    let physics_component = game.get_component::<Physics>().unwrap();

    let mut position_component = game.get_mut_component::<Position>().unwrap();

    for (player_id, input) in input_component.iter() {
        let speed = physics_component.get(player_id).unwrap();
        if let Some(player) = position_component.get_mut(player_id) {
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
}
