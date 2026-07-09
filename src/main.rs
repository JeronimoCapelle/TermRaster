mod canvas;
mod display;
mod grapher;
mod mem_grid;
mod shapes;

use crate::canvas::Canvas;
use crate::display::Display;
use crate::grapher::Grapher;
use crate::shapes::Shape;

fn main() {
    let mut canvas = Canvas::_debug_with_memory();
    let mut grapher = Grapher::new();

    grapher.graph(Shape::Point((8, 1), 'P'));
    grapher.graph(Shape::Point((1, 1), 'T'));

    grapher.draw(&mut canvas);

    Display::draw(&canvas);
}
