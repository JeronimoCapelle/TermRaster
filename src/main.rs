mod canvas;
mod converter;
mod display;
mod parser;
mod pixel_buffer;
mod renderer;
mod shapes;

use crate::canvas::Canvas;
use crate::renderer::Renderer;
use crate::shapes::Shape;

fn main() {
    let mut canvas = Canvas::new();
    let mut renderer = Renderer::new();

    renderer.add_shape(Shape::CircleOutline((35, 15), 7, '#'));
    renderer.add_shape(Shape::Circle((15, 15), 7, '#'));

    renderer.render(&mut canvas);

    display::display(&canvas);
}
