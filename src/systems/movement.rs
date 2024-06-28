use crate::{
    components::{InputState, Physics, Render},
    ecs::{Entity, ECS},
    render::asset_manager::{self, AssetManager, Square},
    resources::Player,
};

pub fn move_system(game: &mut ECS, asset_manager: &mut AssetManager) {
    let player = game.resources.get::<Player>().unwrap().into();
    let (input, speed) = gather_data(player, game);
    let position_component = game.get_mut_component::<Render>().unwrap();

    // let speed = 0.050;
    let mut moved = false;

    if let Some(player) = position_component.get_mut(player) {
        if input.up {
            player.transform.y = player.transform.y + speed.speed;
            moved = true;
        }
        if input.right {
            player.transform.x = player.transform.x + speed.speed;
            moved = true;
        }
        if input.down {
            player.transform.y = player.transform.y - speed.speed;
            moved = true;
        }
        if input.left {
            player.transform.x = player.transform.x - speed.speed;
            moved = true;
        }
    }

    if moved {
        asset_manager.mark_instance_change::<Square>(player);
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
