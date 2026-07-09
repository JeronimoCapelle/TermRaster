use crate::mem_grid::MemGrid;

pub struct Canvas {
    mem_grid: MemGrid,
    shift: (i32, i32),
}

impl Canvas {
    pub fn _new() -> Canvas {
        Canvas {
            mem_grid: MemGrid::default(),
            shift: (0, 0),
        }
    }

    pub fn _debug_with_memory() -> Canvas {
        Canvas {
            mem_grid: (MemGrid::new(100, 100)),
            shift: (0, 0),
        }
    }

    pub fn set(&mut self, coord: (i32, i32), char: char) {
        // self.update_bounds(coord);
        self.mem_grid.set(coord, char);
    }

    pub fn get(&self, coord: (i32, i32)) -> char {
        if coord.0 < self.shift.0
            || coord.1 < self.shift.1
            || coord.0 >= self.shift.0 + self.mem_grid.get_width() as i32
            || coord.1 >= self.shift.1 + self.mem_grid.get_height() as i32
        {
            return ' ';
        }

        self.mem_grid.get(coord)
    }

    fn _update_bounds(&mut self, _coord: (i32, i32)) {
        todo!()
    }
}
