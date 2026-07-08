fn main() {
    let (x, y, radius) = get_inputs();
    let matrix = write_matrix(x, y, radius);
    display_matrix(&matrix);
}

fn get_inputs() -> (i32, i32, i32) {
    (
        std::env::args()
            .nth(1)
            .expect("x not supplied")
            .parse::<i32>()
            .expect("Not a number"),
        std::env::args()
            .nth(2)
            .expect("y not supplied")
            .parse::<i32>()
            .expect("Not a number"),
        std::env::args()
            .nth(3)
            .expect("radius not supplied")
            .parse::<i32>()
            .expect("Not a number"),
    )
}

fn write_matrix(center_x: i32, center_y: i32, r: i32) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> =
        vec![vec![' '; center_x.unsigned_abs() as usize * 2]; center_y.unsigned_abs() as usize * 2];

    for t in 0..360 {
        let rad = f64::to_radians(f64::from(t));
        let x = f64::cos(rad) * f64::from(r) + f64::from(center_x);
        let y = f64::sin(rad) * f64::from(r) + f64::from(center_y);

        assert!(x > 0.0 && y > 0.0);

        let x: usize = f64_to_usize(x);
        let y: usize = f64_to_usize(y);

        matrix[y][x] = '#';
    }
    matrix
}

fn display_matrix(matrix: &Vec<Vec<char>>) {
    for i in matrix {
        for j in i {
            print!("{j}");
        }
        println!();
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn f64_to_usize(x: f64) -> usize {
    x.round().abs() as usize
}
