use crate::{bundles::FoodBundle, components::InputState, ecs::ECS, resources::Player};

pub fn spawn_edible(game: &mut ECS, count: &mut u32) {
    let player = game.resources.get::<Player>().unwrap().inner();
    let input = game.query::<InputState>(player).unwrap();

    if input.space {
        game.add_bundle(FoodBundle::new()).unwrap();
        *count = *count + 1;

        dbg!(&count);
    }
}
