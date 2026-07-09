use crate::pixelbuffer::PixelBuffer;

pub struct Canvas {
    mem_grid: PixelBuffer,
    offset: (i32, i32),
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas {
            mem_grid: PixelBuffer::default(),
            offset: (0, 0),
        }
    }

    pub fn set(&mut self, coord: (i32, i32), char: char) {
        if self.is_outside_bounds(coord) {
            self.resize_bounds(coord);
        }

        let x = (coord.0 - self.offset.0) as usize;
        let y = (coord.1 - self.offset.1) as usize;

        self.mem_grid.set((x, y), char);
    }

    pub fn get(&self, coord: (i32, i32)) -> char {
        if self.is_outside_bounds(coord) {
            return ' ';
        }
        let x = (coord.0 - self.offset.0) as usize;
        let y = (coord.1 - self.offset.1) as usize;

        self.mem_grid.get((x, y))
    }

    fn is_outside_bounds(&self, coord: (i32, i32)) -> bool {
        coord.0 < self.offset.0
            || coord.1 < self.offset.1
            || coord.0 >= self.offset.0 + self.mem_grid.get_width() as i32
            || coord.1 >= self.offset.1 + self.mem_grid.get_height() as i32
    }

    fn resize_bounds(&mut self, coord: (i32, i32)) {
        if coord.0 < self.offset.0 {
            self.mem_grid.expand_x((self.offset.0 - coord.0) as usize);
            self.offset.0 -= coord.0;
        } else if coord.0 >= self.offset.0 + self.mem_grid.get_width() as i32 {
            self.mem_grid.expand_x(dbg!(
                (coord.0 - self.offset.0 + 1) as usize - self.mem_grid.get_width()
            ));
        }

        if coord.1 < self.offset.1 {
            self.mem_grid.expand_y((self.offset.1 - coord.1) as usize);
            self.offset.1 -= coord.1;
        } else if coord.1 >= self.offset.1 + self.mem_grid.get_height() as i32 {
            self.mem_grid
                .expand_y((coord.1 - self.offset.1 + 1) as usize - self.mem_grid.get_height());
        }
    }
}
