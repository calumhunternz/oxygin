use crate::{
    bundles::FoodBundle,
    components::InputState,
    ecs::ECS,
    render::asset_manager::{AssetManager, Food},
    resources::Player,
};

pub fn spawn_edible(game: &mut ECS, asset_manager: &mut AssetManager) {
    let player = game.resources.get::<Player>().unwrap().into();
    let input = game.query_mut::<InputState>(player).unwrap();

    if input.space {
        for i in 0..1000 {
            let food = game.add_bundle(FoodBundle::new()).unwrap();
            asset_manager.add_asset::<Food>(food)
        }
    }
}
