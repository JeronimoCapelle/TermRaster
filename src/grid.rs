pub struct Grid {
    contents: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            contents: vec![' '; width * height],
            width,
            height,
        }
    }
    pub fn set(&mut self, x: usize, y: usize) {
        self.contents[x + y * self.width] = '#';
    }
    pub fn get(&self, x: usize, y: usize) -> char {
        self.contents[x + y * self.width]
    }

    fn _resize(&mut self) {
        todo!()
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
}
