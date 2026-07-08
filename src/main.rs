use crate::{grid::Grid, shapes::graph_circle};

mod cast;
mod coord;
mod grid;
mod shapes;
fn main() {
    let radius = get_input();
    let mut grid = Grid::new((radius * 2) + 1, (radius * 2) + 1);
    graph_circle(&mut grid, radius, radius, radius);
    display_matrix(&grid);
}

fn get_input() -> usize {
    let mut args = std::env::args().skip(1);
    args.next()
        .expect("x not supplied")
        .parse()
        .expect("Not a number")
}

fn display_matrix(grid: &Grid) {
    for i in 0..grid.get_height() {
        for j in 0..grid.get_width() {
            print!("{}", grid.get(j, i));
        }
        println!();
    }
}
