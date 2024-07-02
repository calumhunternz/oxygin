use oxygin::app::app::App;
use oxygin::bundles::{FoodBundle, PlayerBundle};
use oxygin::render::asset_manager::{Food, Square};
use oxygin::resources::Player;

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

//
// Query 2nd attempt
// Given query i want to get data do some computation with the data
// example:
// pub fun move_system(Query<(T1: QueryData, T2: QueryData)>
// Command -> update the data

fn init(app: &mut App) {
    app.ecs.register_bundle::<FoodBundle>();
    app.ecs.register_bundle::<PlayerBundle>();

    let player = app
        .ecs
        .add_bundle(PlayerBundle::new(400, 400, 50, 0.5, 0.5, 0.5))
        .unwrap();
    app.ecs.add_resource(Player::new(&player));
    let square = Square::new();
    let square2 = Food::new();
    app.register_asset(square);
    app.register_asset(square2);
    app.assets.add_asset::<Square>(player);
}

fn main() {
    App::run(init);
}
