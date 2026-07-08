const SAMPLE_SCALE_FACTOR: usize = 7;

struct Matrix {
    contents: Vec<char>,
    side_length: usize,
}

impl Matrix {
    fn new(r: usize) -> Matrix {
        let side_length = 2 * r + 1;
        Matrix {
            contents: vec![' '; side_length * side_length],
            side_length,
        }
    }
    fn set(&mut self, x: usize, y: usize) {
        self.contents[x + y * self.side_length] = '#';
    }
    fn get(&self, x: usize, y: usize) -> char {
        self.contents[x + y * self.side_length]
    }
}

fn main() {
    let radius = get_input();
    let mut matrix = Matrix::new(radius);
    write_matrix(&mut matrix, radius);
    display_matrix(&matrix);
}

fn get_input() -> usize {
    let mut args = std::env::args().skip(1);
    args.next()
        .expect("x not supplied")
        .parse()
        .expect("Not a number")
}

fn write_matrix(matrix: &mut Matrix, r: usize) {
    let sample_size = SAMPLE_SCALE_FACTOR * r;

    let r = r as f64;
    for t in 0..sample_size {
        let rad = f64::to_radians(t as f64 / sample_size as f64 * 360.);
        let x = f64::cos(rad) * r + r;
        let y = f64::sin(rad) * r + r;

        assert!(x >= 0.0 && y >= 0.0);

        let x: usize = f64_to_usize(x);
        let y: usize = f64_to_usize(y);

        dbg!(x, y);
        matrix.set(x, y);
    }
}

fn display_matrix(matrix: &Matrix) {
    for i in 0..matrix.side_length {
        for j in 0..matrix.side_length {
            print!("{}", matrix.get(j, i));
        }
        println!();
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn f64_to_usize(x: f64) -> usize {
    x.round().abs() as usize
}
