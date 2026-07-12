use crate::{
    canvas::Canvas,
    converter::{self, usize_to_i32},
    renderer, shapes,
};

///The renderer holds the objects to be drawn on a canvas and then displayed.
pub struct Renderer {
    shapes: Vec<crate::shapes::Object>,
}

impl Renderer {
    ///Create an empty renderer.
    pub fn new() -> Renderer {
        Renderer { shapes: Vec::new() }
    }

    ///Add a new shape to the collection of the renderer
    pub fn add_shape(&mut self, shape: crate::Object) {
        self.shapes.push(shape);
    }

    ///Render all the shapes held into a canvas, doesnt consume them.
    pub fn render(&self, canvas: &mut Canvas) {
        for shape in &self.shapes {
            match shape {
                shapes::Object::Point(coord, char) => {
                    renderer::Renderer::render_point(canvas, *coord, *char);
                }
                shapes::Object::CircleOutline(coord, radius, char) => {
                    renderer::Renderer::render_circle_outline(canvas, *coord, *radius, *char);
                }
                shapes::Object::RectangleOutline(coord_1, coord_2, char) => {
                    renderer::Renderer::render_rectangle_outline(canvas, *coord_1, *coord_2, *char);
                }
                shapes::Object::Line(coord_1, coord_2, char) => {
                    renderer::Renderer::render_line(canvas, *coord_1, *coord_2, *char);
                }
                shapes::Object::Circle(coord, radius, char) => {
                    renderer::Renderer::render_circle(canvas, *coord, *radius, *char);
                }
                shapes::Object::Rectangle(coord_1, coord_2, char) => {
                    renderer::Renderer::render_rectangle(canvas, *coord_1, *coord_2, *char);
                }
                shapes::Object::Text(coord, text) => {
                    renderer::Renderer::render_text(canvas, *coord, text);
                }
            }
        }
    }

    fn render_text(canvas: &mut Canvas, coord: (i32, i32), string: &str) {
        for i in 0..string.chars().count() {
            canvas.set(
                (coord.0 + usize_to_i32(i), coord.1),
                string.chars().nth(i).unwrap(),
            );
        }
    }

    fn render_line(canvas: &mut Canvas, coord_1: (i32, i32), coord_2: (i32, i32), char: char) {
        let (lower_x, upper_x) = if coord_1.0 <= coord_2.0 {
            (coord_1.0, coord_2.0)
        } else {
            (coord_2.0, coord_1.0)
        };

        let (lower_y, upper_y) = if coord_1.1 <= coord_2.1 {
            (coord_1.1, coord_2.1)
        } else {
            (coord_2.1, coord_1.1)
        };

        let slope = f64::from(coord_2.1 - coord_1.1) / f64::from(coord_2.0 - coord_1.0);

        for x in lower_x..upper_x {
            let y = converter::f64_to_i32(
                (slope * f64::from(x - coord_1.0) + f64::from(coord_1.1)).round(),
            );
            canvas.set((x, y), char);
        }

        let slope = f64::from(coord_2.0 - coord_1.0) / f64::from(coord_2.1 - coord_1.1);

        for y in lower_y..upper_y {
            let x = converter::f64_to_i32(
                (slope * f64::from(y - coord_1.1) + f64::from(coord_1.0)).round(),
            );
            canvas.set((x, y), char);
        }
    }

    fn render_rectangle_outline(
        canvas: &mut Canvas,
        coord_1: (i32, i32),
        coord_2: (i32, i32),
        char: char,
    ) {
        let (lower_x, upper_x) = if coord_1.0 <= coord_2.0 {
            (coord_1.0, coord_2.0)
        } else {
            (coord_2.0, coord_1.0)
        };

        let (lower_y, upper_y) = if coord_1.1 <= coord_2.1 {
            (coord_1.1, coord_2.1)
        } else {
            (coord_2.1, coord_1.1)
        };

        for i in lower_x..upper_x {
            canvas.set((i, coord_1.1), char);
            canvas.set((i, coord_2.1), char);
        }
        for i in lower_y..upper_y {
            canvas.set((coord_1.0, i), char);
            canvas.set((coord_2.0, i), char);
        }
    }

    fn render_rectangle(canvas: &mut Canvas, coord_1: (i32, i32), coord_2: (i32, i32), char: char) {
        let (lower_x, upper_x) = if coord_1.0 <= coord_2.0 {
            (coord_1.0, coord_2.0)
        } else {
            (coord_2.0, coord_1.0)
        };

        let (lower_y, upper_y) = if coord_1.1 <= coord_2.1 {
            (coord_1.1, coord_2.1)
        } else {
            (coord_2.1, coord_1.1)
        };

        for x in lower_x..upper_x {
            for y in lower_y..upper_y {
                canvas.set((x, y), char);
            }
        }
    }

    fn render_circle(canvas: &mut Canvas, coord: (i32, i32), radius: i32, char: char) {
        for y in -radius..=radius {
            let mod_x = converter::f64_to_i32(f64::from(radius * radius - y * y).sqrt().round());
            for x in -mod_x..=mod_x {
                canvas.set((coord.0 + x, coord.1 + y), char);
            }
        }
        for x in -radius..=radius {
            let mod_y = converter::f64_to_i32(f64::from(radius * radius - x * x).sqrt().round());

            for y in -mod_y..=mod_y {
                canvas.set((coord.0 + x, coord.1 + y), char);
            }
        }
    }

    fn render_circle_outline(canvas: &mut Canvas, coord: (i32, i32), radius: i32, char: char) {
        for y in -radius..=radius {
            let mod_x = converter::f64_to_i32(f64::from(radius * radius - y * y).sqrt().round());
            canvas.set((coord.0 + mod_x, coord.1 + y), char);
            canvas.set((coord.0 - mod_x, coord.1 + y), char);
        }
        for x in -radius..=radius {
            let mod_y = converter::f64_to_i32(f64::from(radius * radius - x * x).sqrt().round());
            canvas.set((coord.0 + x, coord.1 + mod_y), char);
            canvas.set((coord.0 + x, coord.1 - mod_y), char);
        }
    }

    fn render_point(canvas: &mut Canvas, coord: (i32, i32), char: char) {
        canvas.set(coord, char);
    }
}
