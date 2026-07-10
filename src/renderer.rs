use crate::{canvas::Canvas, renderer, shapes};

///The renderer holds the objects to be drawn on a canvas and then displayed.
pub struct Renderer {
    shapes: Vec<crate::shapes::Shape>,
}

impl Renderer {
    ///Create an empty renderer.
    pub fn new() -> Renderer {
        Renderer { shapes: Vec::new() }
    }

    ///Add a new shape to the collection of the renderer
    pub fn add_shape(&mut self, shape: crate::Shape) {
        self.shapes.push(shape);
    }

    ///Render all the shapes held into a canvas, doesnt consume them.
    pub fn render(&self, canvas: &mut Canvas) {
        for shape in &self.shapes {
            match shape {
                shapes::Shape::Point(coord, char) => {
                    renderer::Renderer::render_point(canvas, *coord, *char);
                }
                shapes::Shape::Circle(coord, radius, char) => {
                    renderer::Renderer::render_circle(canvas, *coord, *radius, *char);
                }
                shapes::Shape::Rectangle(top_left, bottom_right, char) => {
                    renderer::Renderer::render_rectangle(canvas, *top_left, *bottom_right, *char);
                }
            }
        }
    }

    fn render_rectangle(
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

    fn render_circle(canvas: &mut Canvas, coord: (i32, i32), radius: i32, char: char) {
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

    fn render_point(canvas: &mut Canvas, coord: (i32, i32), char: char) {
        canvas.set(coord, char);
    }
}
