pub enum Shape {
    Point((i32, i32), char),
    Circle((i32, i32), i32, char),
    Rectangle((i32, i32), (i32, i32), char),
}
