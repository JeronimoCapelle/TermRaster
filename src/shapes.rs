///Objects to be rendered by the renderer onto a canvas, in the future some functions will be added.
pub enum Shape {
    Point((i32, i32), char),
    Circle((i32, i32), i32, char),
    Rectangle((i32, i32), (i32, i32), char),
}
