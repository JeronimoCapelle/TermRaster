pub struct MemGrid {
    contents: Vec<char>,
    width: usize,
    height: usize,
}

impl MemGrid {
    pub fn new(width: usize, height: usize) -> MemGrid {
        MemGrid {
            contents: vec![' '; width * height],
            width,
            height,
        }
    }

    pub fn default() -> MemGrid {
        MemGrid {
            contents: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn get(&self, coord: (i32, i32)) -> char {
        let index = self.coord_to_1_d(coord);
        self.contents[index]
    }

    fn coord_to_1_d(&self, coord: (i32, i32)) -> usize {
        coord.0 as usize + coord.1 as usize * self.width
    }

    pub fn set(&mut self, coord: (i32, i32), char: char) {
        let index = self.coord_to_1_d(coord);
        self.contents[index] = char;
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_width(&self) -> usize {
        self.width
    }

    //mem_grid is not resizeable right now, make a new instance and discard the last
    pub fn _resize(&mut self) {
        todo!()
    }
}
