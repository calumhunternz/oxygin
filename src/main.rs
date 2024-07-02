use std::sync::Arc;

use oxygin::app::App;
use oxygin::bundles::{FoodBundle, PlayerBundle};
use oxygin::render::asset_manager::{Food, Square};
use oxygin::resources::Player;
use winit::application::ApplicationHandler;
use winit::window::Window;

// TO IMPROVE
// I want to remove the call to get ref then get component DONE!!!!!
// Have a way to get the player entity more easily DONE!!! through resource storage
// Break structure project to seperate out engine DONE
// Add components to entities (if only syntactically) previously skill issued but now done
// Remove refcell from component storage (Skilled issued -> Now fookin done (with anymap)
//
// maybe chainable (Im thinking plugins for this)
// Traces within the ECS rather than unwraps
//
// investigate queries previous attempt go skill issued
// Investigate component / resource register
// Investigate plugins
//

// fn init(app: &mut App) {
//     app.ecs.register_bundle::<FoodBundle>();
//     app.ecs.register_bundle::<PlayerBundle>();
//
//     let player = app
//         .ecs
//         .add_bundle(PlayerBundle::new(400, 400, 50, 0.5, 0.5, 0.5))
//         .unwrap();
//     app.ecs.add_resource(Player::new(&player));
//     let square = Square::new();
//     let square2 = Food::new();
//     app.assets.register(square);
//     app.assets.register(square2);
//     app.assets.add_asset::<Square>(player)
// }

fn main() {
    // let mut app = App::new();
    // app.init(init);
    App::run();
}
