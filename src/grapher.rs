use crate::{canvas::Canvas, grapher, shapes};

pub struct Grapher {
    shapes: Vec<crate::shapes::Shape>,
}

impl Grapher {
    pub fn new() -> Grapher {
        Grapher { shapes: Vec::new() }
    }

    pub fn graph(&mut self, shape: crate::Shape) {
        self.shapes.push(shape);
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        for shape in &self.shapes {
            match shape {
                shapes::Shape::Point(coord, char) => {
                    grapher::Grapher::draw_point(canvas, *coord, *char)
                }
            }
        }
    }

    fn draw_point(canvas: &mut Canvas, coord: (i32, i32), char: char) {
        canvas.set(coord, char);
    }
}
