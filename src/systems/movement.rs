use crate::{
    components::{InputState, Physics, Render},
    ecs::{Entity, ECS},
    resources::Player,
};

pub fn move_system(game: &mut ECS) {
    let player = game.resources.get::<Player>().unwrap().inner();
    let (input, speedd) = gather_data(player, game);
    let position_component = game.get_mut_component::<Render>().unwrap();

    let speed = 0.050;

    if let Some(player) = position_component.get_mut(player) {
        if input.up {
            player.transform.y = player.transform.y + speed;
        }
        if input.right {
            player.transform.x = player.transform.x + speed;
        }
        if input.down {
            player.transform.y = player.transform.y - speed;
        }
        if input.left {
            player.transform.x = player.transform.x - speed;
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
