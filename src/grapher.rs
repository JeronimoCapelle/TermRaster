use crate::{canvas::Canvas, grapher, shapes};

pub struct Renderer {
    shapes: Vec<crate::shapes::Shape>,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer { shapes: Vec::new() }
    }

    pub fn add_shape(&mut self, shape: crate::Shape) {
        self.shapes.push(shape);
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        for shape in &self.shapes {
            match shape {
                shapes::Shape::Point(coord, char) => {
                    grapher::Renderer::draw_point(canvas, *coord, *char)
                }
                shapes::Shape::Circle(coord, radius, char) => {
                    grapher::Renderer::draw_circle(canvas, *coord, *radius, *char)
                }
                shapes::Shape::Rectangle(top_left, bottom_right, char) => {
                    grapher::Renderer::draw_rectangle(canvas, *top_left, *bottom_right, *char)
                }
            }
        }
    }

    fn draw_rectangle(
        canvas: &mut Canvas,
        top_left: (i32, i32),
        bottom_right: (i32, i32),
        char: char,
    ) {
        for i in top_left.0..bottom_right.0 {
            canvas.set((i, top_left.1), char);
            canvas.set((i, bottom_right.1), char);
        }
        for i in top_left.1..bottom_right.1 {
            canvas.set((top_left.0, i), char);
            canvas.set((bottom_right.0, i), char);
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
