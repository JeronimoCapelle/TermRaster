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
                shapes::Shape::Circle(coord, radius, char) => {
                    grapher::Grapher::draw_circle(canvas, *coord, *radius, *char)
                }
            }
        }
    }

    fn draw_circle(canvas: &mut Canvas, coord: (i32, i32), radius: i32, char: char) {
        for y in -radius..radius {
            let mod_x = f64::from(radius * radius - y * y).sqrt().round() as i32;
            canvas.set((coord.0 + mod_x, coord.1 + y), char);
            canvas.set((coord.0 - mod_x, coord.1 + y), char);
        }
        for x in -radius..radius {
            let mod_y = f64::from(radius * radius - x * x).sqrt().round() as i32;
            canvas.set((coord.0 + x, coord.1 + mod_y), char);
            canvas.set((coord.0 + x, coord.1 - mod_y), char);
        }
    }

    fn draw_point(canvas: &mut Canvas, coord: (i32, i32), char: char) {
        canvas.set(coord, char);
    }
}
