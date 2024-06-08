use rand::Rng;

use crate::{
    components::{Edible, Position, Size},
    ecs::{Entity, ECS},
    resources::Player,
};

pub fn eat_system(game: &mut ECS) {
    let eaten_entities = get_eaten_entities(game);

    set_edible_eaten(game, &eaten_entities);

    eat_edibles(game, &eaten_entities);

    handle_eaten(game, &eaten_entities);

    if eaten_entities.len() > 0 {
        grow(game, &eaten_entities);
    }
}

fn grow(game: &mut ECS, entities: &Vec<Entity>) {
    let player = game.resources.get::<Player>().unwrap().inner();
    let edible_component = game.get_component::<Edible>().unwrap();
    let mut size_component = game.get_mut_component::<Size>().unwrap();

    for eaten in entities {
        let edible = edible_component.get(*eaten).unwrap();
        let player_size = size_component.get_mut(player).unwrap();
        player_size.size = player_size.size + edible.calories;
    }
}

fn set_edible_eaten(game: &mut ECS, entities: &Vec<Entity>) {
    let mut edible_component = game.get_mut_component::<Edible>().unwrap();

    entities.into_iter().for_each(|entity| {
        let edible = edible_component.get_mut(*entity).unwrap();
        edible.eaten = true;
    })
}

fn get_eaten_entities(game: &mut ECS) -> Vec<Entity> {
    let player = game.resources.get::<Player>().unwrap().inner();
    let position_component = game.get_component::<Position>().unwrap();
    let size_component = game.get_component::<Size>().unwrap();
    let edible_component = game.get_component::<Edible>().unwrap();
    let mut eaten_entities: Vec<Entity> = vec![];

    let player_position = position_component.get(player).unwrap();
    let player_size = size_component.get(player).unwrap();

    for edible_key in edible_component.keys() {
        let edible_position = position_component.get(edible_key).unwrap();
        let edible_size = size_component.get(edible_key).unwrap();
        if check_collision(
            &player_position,
            &player_size,
            &edible_position,
            &edible_size,
        ) {
            eaten_entities.push(edible_key);
        }
    }

    eaten_entities
}

fn handle_eaten(game: &mut ECS, entities: &Vec<Entity>) {
    let mut edible_component = game.get_mut_component::<Edible>().unwrap();

    entities.into_iter().for_each(|edible| {
        let eaten_entity = edible_component.get_mut(*edible).unwrap();
        eaten_entity.eaten = false;
    })
}

fn eat_edibles(game: &mut ECS, entities: &Vec<Entity>) {
    let mut position_component = game.get_mut_component::<Position>().unwrap();

    entities.into_iter().for_each(|entity| {
        let position = position_component.get_mut(*entity).unwrap();
        position.x = rand::thread_rng().gen_range(1..1920);
        position.y = rand::thread_rng().gen_range(1..1080);
    });
}

fn check_collision(position1: &Position, size1: &Size, position2: &Position, size2: &Size) -> bool {
    if position1.x < (position2.x + size2.size as i32)
        && (position1.x + size1.size as i32) > position2.x
        && position1.y < (position2.y + size2.size as i32)
        && position1.y + size1.size as i32 > position2.y
    {
        return true;
    }
    false
}
