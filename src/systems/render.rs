use sdl2::{rect::Rect, render::Canvas, video::Window};

use crate::{
    components::{ColorComponent, Position, Size},
    ecs::ECS,
};

pub fn render_system(game: &ECS, canvas: &mut Canvas<Window>) {
    let position_component = game.get_component::<Position>().unwrap();
    let size_component = game.get_component::<Size>().unwrap();
    let color_component = game.get_component::<ColorComponent>().unwrap();

    for (key, position) in position_component.iter() {
        let size = size_component.get(key).unwrap();
        let color = color_component.get(key).unwrap();

        canvas.set_draw_color(color.rgb);
        canvas
            .fill_rect(Rect::new(position.x, position.y, size.size, size.size))
            .unwrap();
    }
}
