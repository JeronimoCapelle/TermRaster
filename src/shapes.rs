///Objects to be rendered by the renderer onto a canvas, in the future some functions will be added.
pub enum Object {
    Point((i32, i32), char),
    CircleOutline((i32, i32), i32, char),
    Circle((i32, i32), i32, char),
    RectangleOutline((i32, i32), (i32, i32), char),
    Rectangle((i32, i32), (i32, i32), char),
    Line((i32, i32), (i32, i32), char),
    Text((i32, i32), String),
}
