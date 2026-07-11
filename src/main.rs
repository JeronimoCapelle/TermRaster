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

    renderer.add_shape(Shape::Point((2, 2), 'P'));
    renderer.add_shape(Shape::Point((4, 2), 'E'));
    renderer.add_shape(Shape::Point((4, 4), 'C'));
    renderer.add_shape(Shape::Circle((15, 15), 7, '#'));
    renderer.add_shape(Shape::Rectangle((6, 6), (10, 10), '='));

    renderer.add_shape(Shape::Line((30, 20), (34, 27), '%'));

    renderer.render(&mut canvas);

    display::display(&canvas);
}
