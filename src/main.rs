mod canvas;
mod converter;
mod display;
mod pixel_buffer;
mod renderer;
mod shapes;

use crate::canvas::Canvas;
use crate::renderer::Renderer;
use crate::shapes::Object;

fn main() {
    let mut canvas = Canvas::new();
    let mut renderer = Renderer::new();

    renderer.add_shape(Object::CircleOutline((35, 15), 7, '#'));
    renderer.add_shape(Object::Text((32, 15), String::from("Outline")));

    renderer.add_shape(Object::Circle((15, 15), 7, '#'));
    renderer.add_shape(Object::Text((12, 15), String::from("Filled")));

    renderer.render(&mut canvas);

    display::display(&canvas);
}
