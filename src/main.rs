mod canvas;
mod display;
mod grapher;
mod pixelbuffer;
mod shapes;

use crate::canvas::Canvas;
use crate::grapher::Grapher;
use crate::shapes::Shape;

fn main() {
    let mut canvas = Canvas::new();
    let mut grapher = Grapher::new();

    grapher.graph(Shape::Point((2, 2), 'P'));
    grapher.graph(Shape::Point((4, 2), 'E'));
    grapher.graph(Shape::Point((4, 4), 'C'));
    grapher.graph(Shape::Circle((15, 15), 7, '#'));

    grapher.draw(&mut canvas);

    display::display(&canvas);
}
