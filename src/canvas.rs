use crate::pixel_buffer::PixelBuffer;

///A canvas is an object in which individual pixels can be drawn, as well as shapes using the renderer, then a display can draw the contents to a screen.
pub struct Canvas {
    mem_grid: PixelBuffer,
    offset: (i32, i32),
}

impl Canvas {
    ///Return a new empty canvas object
    pub fn new() -> Canvas {
        Canvas {
            mem_grid: PixelBuffer::default(),
            offset: (0, 0),
        }
    }

    ///Set a specific point in space to a provided char
    pub fn set(&mut self, coord: (i32, i32), char: char) {
        if self.is_outside_bounds(coord) {
            self.resize_bounds(coord);
        }

        let x = (coord.0 - self.offset.0) as usize;
        let y = (coord.1 - self.offset.1) as usize;

        self.mem_grid.set((x, y), char);
    }

    ///Get the specific char stored at the provided point
    pub fn get(&self, coord: (i32, i32)) -> char {
        if self.is_outside_bounds(coord) {
            return ' ';
        }
        let x = usize::try_from(coord.0 - self.offset.0).expect("usize to i32 convertion error");
        let y = usize::try_from(coord.1 - self.offset.1).expect("usize to i32 convertion error");

        self.mem_grid.get((x, y))
    }

    ///Checks wether FrameBuffer has to be expanded
    fn is_outside_bounds(&self, coord: (i32, i32)) -> bool {
        coord.0 < self.offset.0
            || coord.1 < self.offset.1
            || coord.0 >= self.offset.0 + self.mem_grid.width() as i32
            || coord.1 >= self.offset.1 + self.mem_grid.height() as i32
    }

    ///Resizes the internal frameBuffer, changes the shift accordingly
    fn resize_bounds(&mut self, coord: (i32, i32)) {
        if coord.0 < self.offset.0 {
            self.mem_grid
                .resize_horizontally_by((self.offset.0 - coord.0) as usize);
            self.offset.0 -= coord.0;
        } else if coord.0 >= self.offset.0 + self.mem_grid.width() as i32 {
            self.mem_grid.resize_horizontally_by(
                (coord.0 - self.offset.0 + 1) as usize - self.mem_grid.width(),
            );
        }

        if coord.1 < self.offset.1 {
            self.mem_grid
                .resize_vertically_by((self.offset.1 - coord.1) as usize);
            self.offset.1 -= coord.1;
        } else if coord.1 >= self.offset.1 + self.mem_grid.height() as i32 {
            self.mem_grid.resize_vertically_by(
                (coord.1 - self.offset.1 + 1) as usize - self.mem_grid.height(),
            );
        }
    }
}
