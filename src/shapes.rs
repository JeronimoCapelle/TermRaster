const SAMPLE_SCALE_FACTOR: usize = 7;

use crate::Grid;
use crate::cast::round_and_clamp;
use crate::cast::usize_to_f64;

// pub enum Shapes {
//     Rectangle,
//     Square,
//     Circle,
// }

// impl Shapes {
//     pub fn graph(self) {
//         match (self) {
//             Circle() => graph_circle(),
//         }
//     }
// }

pub fn graph_circle(grid: &mut Grid, center_x: usize, center_y: usize, r: usize) {
    let sample_size = SAMPLE_SCALE_FACTOR * r;

    let r = usize_to_f64(r);
    for t in 0..sample_size {
        let rad = f64::to_radians(usize_to_f64(t) / usize_to_f64(sample_size) * 360.);
        let x = f64::cos(rad) * r + usize_to_f64(center_x);
        let y = f64::sin(rad) * r + usize_to_f64(center_y);

        assert!(x >= 0.0 && y >= 0.0);

        let x: usize = round_and_clamp(x);
        let y: usize = round_and_clamp(y);

        grid.set(x, y);
    }
}
