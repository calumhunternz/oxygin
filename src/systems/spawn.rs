use crate::{
    bundles::FoodBundle, components::InputState, ecs::ECS, render::asset_manager::Food,
    resources::Player,
};

pub fn spawn_edible(game: &mut ECS) {
    let player = game.resources.get::<Player>().unwrap().into();
    let input = game.query_mut::<InputState>(player).unwrap();

    if input.space {
        let food = game.add_bundle(FoodBundle::new()).unwrap();
        game.add_asset::<Food>(food)
    }
}
