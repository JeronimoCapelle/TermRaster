use crate::Canvas;

pub struct Display {}

impl Display {
    pub fn draw(canvas: &Canvas) {
        for i in 0..100 {
            for j in 0..100 {
                print!("{}", canvas.get((j, i)));
            }
            println!();
        }
    }
}
