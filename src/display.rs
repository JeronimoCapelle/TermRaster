use crate::Canvas;

pub fn display(canvas: &Canvas) {
    for i in 0..50 {
        for j in 0..50 {
            print!("{}", canvas.get((j, i)));
        }
        println!();
    }
}
