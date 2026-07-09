mod canvas;
mod display;
mod grapher;
mod pixelbuffer;
mod shapes;

use crate::canvas::Canvas;
use crate::grapher::Renderer;
use crate::shapes::Shape;

fn main() {
    let mut canvas = Canvas::new();
    let mut renderer = Renderer::new();

    renderer.add_shape(Shape::Point((2, 2), 'P'));
    renderer.add_shape(Shape::Point((4, 2), 'E'));
    renderer.add_shape(Shape::Point((4, 4), 'C'));
    renderer.add_shape(Shape::Circle((15, 15), 7, '#'));
    renderer.add_shape(Shape::Rectangle((6, 6), (10, 10), '='));

    renderer.draw(&mut canvas);

    display::display(&canvas);
}
