use crate::Grid;
use crate::SAMPLE_SCALE_FACTOR;
use crate::convertion_helper::f64_to_usize;

pub fn set_circle(grid: &mut Grid, center_x: usize, center_y: usize, r: usize) {
    let sample_size = SAMPLE_SCALE_FACTOR * r;

    let r = r as f64;
    for t in 0..sample_size {
        let rad = f64::to_radians(t as f64 / sample_size as f64 * 360.);
        let x = f64::cos(rad) * r + center_x as f64;
        let y = f64::sin(rad) * r + center_y as f64;

        assert!(x >= 0.0 && y >= 0.0);

        let x: usize = f64_to_usize(x);
        let y: usize = f64_to_usize(y);

        grid.set(x, y);
    }
}
