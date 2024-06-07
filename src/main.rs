extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::{KeyboardState, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use slotmap::{DefaultKey, SecondaryMap, SlotMap};
use std::any::{Any, TypeId};
// use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use std::time::Duration;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

type Entity = DefaultKey;

type EntityAllocator<K, V> = SlotMap<K, V>;

type EntityMap<K, V> = SecondaryMap<K, V>;

// TO IMPROVE
// I want to remove the call to get ref then get component
// Have a way to get the player entity more easily
// Add components to entities (if only syntactically) eg player.add_component<Position>(position)
// maybe chainable
// Break structure project to seperate out engine
// Traces within the ECS rather than unwraps
// Investigate plugins
// investigate queries
struct ComponentStorage {
    storage: HashMap<TypeId, Rc<RefCell<Box<dyn Any>>>>,
}

impl ComponentStorage {
    fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    fn insert_into_entity_map<T>(&self, entity: Entity, component: T)
    where
        T: 'static + Component,
    {
        if let Some(mut entity_map) = self.get_mut::<T>() {
            entity_map.insert(entity, component);
        } else {
            dbg!("poop");
        }
    }

    fn register<T: Component + 'static>(&mut self)
    where
        T: 'static + Component,
    {
        let component: EntityMap<Entity, T> = EntityMap::new();

        self.storage.insert(
            TypeId::of::<T>(),
            Rc::new(RefCell::new(Box::new(component))),
        );
    }

    fn get<T>(&self) -> Option<Ref<'_, EntityMap<Entity, T>>>
    where
        T: 'static + Component,
    {
        let component_ref = self.storage.get(&TypeId::of::<T>())?;
        Some(Ref::map(component_ref.borrow(), |component| {
            component.downcast_ref::<EntityMap<Entity, T>>().unwrap()
        }))
    }

    fn get_mut<T>(&self) -> Option<RefMut<'_, EntityMap<Entity, T>>>
    where
        T: 'static + Component,
    {
        let component_ref = self.storage.get(&TypeId::of::<T>())?;
        Some(RefMut::map(component_ref.borrow_mut(), |component| {
            component.downcast_mut::<EntityMap<Entity, T>>().unwrap()
        }))
    }
}

struct ECS2 {
    components: ComponentStorage,
    entity_allocator: EntityAllocator<Entity, ()>,
}

impl ECS2 {
    fn new() -> Self {
        Self {
            components: ComponentStorage::new(),
            entity_allocator: EntityAllocator::new(),
        }
    }

    fn create_entity(&mut self) -> Entity {
        self.entity_allocator.insert(())
    }

    fn register_component<T>(&mut self)
    where
        T: 'static + Component,
    {
        self.components.register::<T>();
    }

    pub fn get_component<T>(&self) -> Option<Ref<'_, EntityMap<Entity, T>>>
    where
        T: 'static + Component,
    {
        self.components.get::<T>()
    }

    pub fn get_mut_component<T>(&self) -> Option<RefMut<'_, EntityMap<Entity, T>>>
    where
        T: 'static + Component,
    {
        self.components.get_mut::<T>()
    }

    fn add_component<T: Component + 'static>(&mut self, entity: Entity, component: T)
    where
        T: 'static + Component,
    {
        self.components.insert_into_entity_map(entity, component);
    }
}

struct ECS {
    components: HashMap<TypeId, Rc<RefCell<Box<dyn Any>>>>,
    // cool_components: ComponentStorage,
    entity_allocator: EntityAllocator<Entity, ()>,
}

impl ECS {
    fn new() -> Self {
        let entity_allocator: EntityAllocator<Entity, ()> = EntityAllocator::new();
        let components: HashMap<TypeId, Rc<RefCell<Box<dyn Any>>>> = HashMap::new();
        Self {
            components,
            // cool_components: ComponentStorage::new(),
            entity_allocator,
        }
    }

    fn register_component<T: Component + 'static>(&mut self) {
        let component: EntityMap<Entity, T> = EntityMap::new();

        self.components.insert(
            TypeId::of::<EntityMap<Entity, T>>(),
            Rc::new(RefCell::new(Box::new(component))),
        );
    }

    fn create_entity(&mut self) -> Entity {
        self.entity_allocator.insert(())
    }

    fn add_component<T: Component + 'static>(&mut self, entity: Entity, component: T) {
        let mut binding = self
            .components
            .get(&TypeId::of::<EntityMap<Entity, T>>())
            .unwrap()
            .borrow_mut();

        let entity_map = binding.downcast_mut::<EntityMap<Entity, T>>().unwrap();

        entity_map.insert(entity, component);
    }

    fn get_component<T: Component + 'static>(&self) -> ComponentRef<T> {
        let data = self
            .components
            .get(&TypeId::of::<EntityMap<Entity, T>>())
            .unwrap()
            .borrow();

        ComponentRef {
            data,
            marker: PhantomData,
        }
    }

    fn get_mut_component<T: Component + 'static>(&self) -> ComponentRefMut<T> {
        let data = self
            .components
            .get(&TypeId::of::<EntityMap<Entity, T>>())
            .unwrap()
            .borrow_mut();

        ComponentRefMut {
            data,
            marker: PhantomData,
        }
    }
}

struct ComponentRef<'a, T> {
    data: Ref<'a, Box<dyn Any>>,
    marker: PhantomData<T>,
}

impl<'a, T> ComponentRef<'a, T>
where
    T: 'static + Component,
{
    fn get(&self) -> &EntityMap<Entity, T> {
        self.data.downcast_ref::<EntityMap<Entity, T>>().unwrap()
    }
}

struct ComponentRefMut<'a, T> {
    data: RefMut<'a, Box<dyn Any>>,
    marker: PhantomData<T>,
}

impl<'a, T> ComponentRefMut<'a, T>
where
    T: 'static + Component,
{
    fn get_mut(&mut self) -> &mut EntityMap<Entity, T> {
        self.data.downcast_mut::<EntityMap<Entity, T>>().unwrap()
    }
}

trait Component {}

struct ComponentRegister {
    component_register: Vec<TypeId>,
}

impl ComponentRegister {
    fn register_component<T: Component>(&mut self) {}
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Component for Position {}

struct Size {
    size: u32,
}

impl Component for Size {}

struct InputState {
    up: bool,
    right: bool,
    down: bool,
    left: bool,
}

impl Component for InputState {}

impl InputState {
    fn new() -> Self {
        Self {
            up: false,
            right: false,
            down: false,
            left: false,
        }
    }
}

struct Physics {
    speed: i32,
}

impl Component for Physics {}

struct ColorComponent {
    rgb: Color,
}
impl ColorComponent {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            rgb: Color::RGB(r, g, b),
        }
    }
}

impl Component for ColorComponent {}

struct Edible {
    eaten: bool,
    calories: u32,
}

impl Component for Edible {}

fn main() {
    const RENDER_NORMALIZATION: u32 = 1_000_000_000u32 / 60;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .opengl()
        .borderless()
        .fullscreen_desktop()
        .build()
        .unwrap();

    dbg!(window.size());

    let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    let mut game = ECS::new();
    game.register_component::<Position>();
    game.register_component::<Size>();
    game.register_component::<InputState>();
    game.register_component::<Physics>();
    game.register_component::<ColorComponent>();
    game.register_component::<Edible>();

    let mut cool_game = ECS2::new();
    cool_game.register_component::<Position>();

    let cool_player = cool_game.create_entity();
    cool_game.add_component::<Position>(cool_player, Position { x: 10, y: 10 });

    let cool_position_component = cool_game.get_component::<Position>().unwrap();

    let position = cool_position_component.get(cool_player).unwrap();

    dbg!(position);

    let player = game.create_entity();
    game.add_component::<Size>(player, Size { size: 20 });
    game.add_component::<Position>(player, Position { x: 400, y: 400 });
    game.add_component::<InputState>(player, InputState::new());
    game.add_component::<Physics>(player, Physics { speed: 10 });
    game.add_component::<ColorComponent>(player, ColorComponent::new(255, 255, 255));

    let food = game.create_entity();
    game.add_component::<Size>(food, Size { size: 10 });
    game.add_component::<Position>(food, Position { x: 800, y: 800 });
    game.add_component::<ColorComponent>(food, ColorComponent::new(0, 255, 0));
    game.add_component::<Edible>(
        food,
        Edible {
            eaten: false,
            calories: 10,
        },
    );

    let drug = game.create_entity();
    game.add_component::<Size>(drug, Size { size: 10 });
    game.add_component::<Position>(drug, Position { x: 300, y: 300 });
    game.add_component::<ColorComponent>(drug, ColorComponent::new(0, 0, 255));
    game.add_component::<Edible>(
        drug,
        Edible {
            eaten: false,
            calories: 100,
        },
    );

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    loop {
        canvas.clear();

        handle_input_system(event_pump.keyboard_state(), &mut game);

        move_system(&mut game);
        eat_system(&mut game);

        render_system(&game, &mut canvas);
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        canvas.present();

        ::std::thread::sleep(Duration::new(0, RENDER_NORMALIZATION));
        if !handle_events(&mut event_pump) {
            break;
        }
    }
}

trait System {}

fn render_system(game: &ECS, canvas: &mut Canvas<Window>) {
    let position_component_ref = game.get_component::<Position>();
    let position_component = position_component_ref.get();

    let size_component_ref = game.get_component::<Size>();
    let size_component = size_component_ref.get();

    let color_component_ref = game.get_component::<ColorComponent>();
    let color_component = color_component_ref.get();

    for (key, position) in position_component.iter() {
        let size = size_component.get(key).unwrap();
        let color = color_component.get(key).unwrap();

        canvas.set_draw_color(color.rgb);
        canvas
            .fill_rect(Rect::new(position.x, position.y, size.size, size.size))
            .unwrap();
    }
}

fn eat_system(game: &mut ECS) {
    // Checks for collision with player

    let eaten_entities = get_eaten_entities(game);

    set_edible_eaten(game, &eaten_entities);

    eat_edibles(game, &eaten_entities);

    handle_eaten(game, &eaten_entities);

    if eaten_entities.len() > 0 {
        grow(game, &eaten_entities);
    }
}

fn grow(game: &mut ECS, entities: &Vec<Entity>) {
    let input_component_ref = game.get_component::<InputState>();
    let input_component = input_component_ref.get();

    let edible_component_ref = game.get_component::<Edible>();
    let edible_component = edible_component_ref.get();

    let mut size_component_ref = game.get_mut_component::<Size>();
    let size_component = size_component_ref.get_mut();

    for eaten in entities {
        let edible = edible_component.get(*eaten).unwrap();
        for player in input_component.keys() {
            let player_size = size_component.get_mut(player).unwrap();
            player_size.size = player_size.size + edible.calories;
        }
    }
}

fn set_edible_eaten(game: &mut ECS, entities: &Vec<Entity>) {
    let mut edible_component_ref = game.get_mut_component::<Edible>();
    let edible_component = edible_component_ref.get_mut();

    entities.into_iter().for_each(|entity| {
        let edible = edible_component.get_mut(*entity).unwrap();
        edible.eaten = true;
    })
}

fn get_eaten_entities(game: &mut ECS) -> Vec<Entity> {
    let input_component_ref = game.get_component::<InputState>();
    let input_component = input_component_ref.get();

    let position_component_ref = game.get_component::<Position>();
    let position_component = position_component_ref.get();

    let size_component_ref = game.get_component::<Size>();
    let size_component = size_component_ref.get();

    let edible_component_ref = game.get_component::<Edible>();
    let edible_component = edible_component_ref.get();

    let mut eaten_entities: Vec<Entity> = vec![];

    for input in input_component.keys() {
        let player_position = position_component.get(input).unwrap();
        let player_size = size_component.get(input).unwrap();
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
    }

    eaten_entities
}

fn handle_eaten(game: &mut ECS, entities: &Vec<Entity>) {
    let mut edible_component_ref = game.get_mut_component::<Edible>();
    let edible_component = edible_component_ref.get_mut();

    entities.into_iter().for_each(|edible| {
        let eaten_entity = edible_component.get_mut(*edible).unwrap();
        eaten_entity.eaten = false;
    })
}

fn eat_edibles(game: &mut ECS, entities: &Vec<Entity>) {
    let mut position_component_ref = game.get_mut_component::<Position>();
    let position_component = position_component_ref.get_mut();

    entities.into_iter().for_each(|entity| {
        let position = position_component.get_mut(*entity).unwrap();
        position.x = rand::thread_rng().gen_range(1..1920);
        position.y = rand::thread_rng().gen_range(1..1080);
    });
}

fn handle_input_system(keyboard: KeyboardState, game: &mut ECS) {
    let mut input_component_ref = game.get_mut_component::<InputState>();
    let input_component = input_component_ref.get_mut();

    for input in input_component.values_mut() {
        input.up = keyboard.is_scancode_pressed(Scancode::Up);
        input.right = keyboard.is_scancode_pressed(Scancode::Right);
        input.down = keyboard.is_scancode_pressed(Scancode::Down);
        input.left = keyboard.is_scancode_pressed(Scancode::Left);
    }
}

fn move_system(game: &mut ECS) {
    let input_component_ref = game.get_component::<InputState>();
    let input_component = input_component_ref.get();

    let physics_component_ref = game.get_component::<Physics>();
    let physics_component = physics_component_ref.get();

    let mut position_component_ref = game.get_mut_component::<Position>();
    let position_component = position_component_ref.get_mut();

    for (player_id, input) in input_component {
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

// fn collision_system(game: &

//
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

fn handle_events(event_pump: &mut EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => return false,
            _ => return true,
        }
    }
    true
}
