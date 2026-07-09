///Maps 2D coordinates to a 1D vector, holds chars, has a specified size, and can be resized explicitily
pub struct PixelBuffer {
    contents: Vec<char>,
    width: usize,
    height: usize,
}

impl PixelBuffer {
    ///Create new PixelBuffer with specified capacity
    pub fn new(width: usize, height: usize) -> PixelBuffer {
        PixelBuffer {
            contents: vec![' '; width * height],
            width,
            height,
        }
    }

    ///Create new pixelBuffer with empty capacity
    pub fn default() -> PixelBuffer {
        PixelBuffer {
            contents: vec![' '; 1],
            width: 1,
            height: 1,
        }
    }

    ///Get char at index(x,y)
    pub fn get(&self, coord: (usize, usize)) -> char {
        let index = coord.0 + coord.1 * self.width;
        self.contents[index]
    }

    ///Set char at index(x,y)
    pub fn set(&mut self, coord: (usize, usize), char: char) {
        let index = coord.0 + coord.1 * self.width;
        self.contents[index] = char;
    }

    ///Height getter
    pub fn get_height(&self) -> usize {
        self.height
    }

    ///Width getter
    pub fn get_width(&self) -> usize {
        self.width
    }

    ///Resize framebuffer, truncates values if smaller, fills with space if bigger
    pub fn resize(&mut self, new_size: (usize, usize)) {
        let mut vec = vec![' '; new_size.0 * new_size.1];

        for y in 0..=usize::min(self.height, new_size.1) - 1 {
            for x in 0..=usize::min(self.width, new_size.0) - 1 {
                vec[y * new_size.0 + x] = self.contents[y * self.width + x];
            }

            for x in self.width..new_size.0 {
                vec[y * new_size.0 + x] = ' ';
            }
        }

        for y in new_size.0 * self.height..new_size.0 * new_size.1 {
            vec[y] = ' ';
        }

        self.contents = vec;
        self.width = new_size.0;
        self.height = new_size.1;
    }

    pub fn resize_x(&mut self, new_width: usize) {
        self.resize((new_width, self.height));
    }

    pub fn resize_y(&mut self, new_height: usize) {
        self.resize((self.width, new_height));
    }

    pub fn expand_x(&mut self, delta_x: usize) {
        self.resize_x(self.width + delta_x);
    }

    pub fn expand_y(&mut self, delta_y: usize) {
        self.resize_y(self.height + delta_y);
    }
}
