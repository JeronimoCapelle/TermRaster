///Maps 2D coordinates to a 1D vector, holds chars, has a specified size, and can be resized explicitily.
pub struct PixelBuffer {
    contents: Vec<char>,
    width: usize,
    height: usize,
}

impl PixelBuffer {
    ///Create new ``PixelBuffer`` with specified capacity.
    pub fn new(width: usize, height: usize) -> PixelBuffer {
        PixelBuffer {
            contents: vec![' '; width * height],
            width,
            height,
        }
    }

    ///Create new ``PixelBuffer`` with empty capacity.
    pub fn default() -> PixelBuffer {
        PixelBuffer {
            contents: vec![' '; 1],
            width: 1,
            height: 1,
        }
    }

    ///Get char at index(x,y).
    pub fn get(&self, coord: (usize, usize)) -> char {
        let index = self.coord_to_index(coord);
        self.contents[index]
    }

    ///Set char at index(x,y).
    pub fn set(&mut self, coord: (usize, usize), char: char) {
        let index = self.coord_to_index(coord);
        self.contents[index] = char;
    }

    ///Height getter.
    pub fn height(&self) -> usize {
        self.height
    }

    ///Width getter.
    pub fn width(&self) -> usize {
        self.width
    }

    ///Transforms a 2D coordinate to the internal vector index.
    fn coord_to_index(&self, (x, y): (usize, usize)) -> usize {
        y * self.width + x
    }

    ///Resize ``PixelBuffer``, truncates values if smaller, fills with space if bigger
    pub fn resize(&mut self, new_size: (usize, usize)) {
        let mut temp_vec = vec![' '; new_size.0 * new_size.1];

        for y in 0..usize::min(self.height, new_size.1) {
            for x in 0..usize::min(self.width, new_size.0) {
                temp_vec[y * new_size.0 + x] = self.contents[self.coord_to_index((x, y))];
            }

            for x in self.width..new_size.0 {
                temp_vec[y * new_size.0 + x] = ' ';
            }
        }

        for index in temp_vec
            .iter_mut()
            .take(new_size.0 * new_size.1)
            .skip(new_size.0 * self.height)
        {
            *index = ' ';
        }

        self.contents = temp_vec;
        self.width = new_size.0;
        self.height = new_size.1;
    }

    /// Resize the array to set height.
    pub fn resize_horizontally(&mut self, new_width: usize) {
        self.resize((new_width, self.height));
    }

    /// Resize the array to set width.
    pub fn resize_vertically(&mut self, new_height: usize) {
        self.resize((self.width, new_height));
    }

    /// resize the array horizontally by ``delta_x``.
    pub fn resize_horizontally_by(&mut self, delta_x: usize) {
        self.resize_horizontally(self.width + delta_x);
    }

    /// Resize the array vertically by ``delta_y``.
    pub fn resize_vertically_by(&mut self, delta_y: usize) {
        self.resize_vertically(self.height + delta_y);
    }
}

#[cfg(test)]
mod tests_pixel_buffer {
    use super::*;

    #[test]
    fn new() {
        let buffer = PixelBuffer::new(100, 50);
        assert_eq!(buffer.width(), 100);
        assert_eq!(buffer.height(), 50);
    }

    #[test]
    fn set_and_get() {
        let mut buffer = PixelBuffer::default();
        buffer.set((0, 0), 'G');
        assert_eq!(buffer.get((0, 0)), 'G');
    }

    #[test]
    fn resize() {
        let mut buffer = PixelBuffer::new(10, 10);
        assert_eq!(buffer.width(), 10);
        assert_eq!(buffer.height(), 10);

        buffer.set((7, 7), '2');

        buffer.resize((8, 8));
        assert_eq!(buffer.width(), 8);
        assert_eq!(buffer.height(), 8);

        buffer.resize_horizontally(10);
        assert_eq!(buffer.width(), 10);
        assert_eq!(buffer.height(), 8);

        buffer.resize_vertically(10);
        assert_eq!(buffer.width(), 10);
        assert_eq!(buffer.height(), 10);

        buffer.resize_horizontally_by(2);
        assert_eq!(buffer.width(), 12);
        assert_eq!(buffer.height(), 10);

        buffer.resize_vertically_by(3);
        assert_eq!(buffer.width(), 12);
        assert_eq!(buffer.height(), 13);

        assert_eq!(buffer.get((7, 7)), '2');
    }

    #[test]
    fn index() {
        let mut buffer = PixelBuffer::new(5, 5);
        assert_eq!(buffer.coord_to_index((3, 3)), 18);
        buffer.resize((4, 4));
        assert_eq!(buffer.coord_to_index((3, 3)), 15);
    }
}
