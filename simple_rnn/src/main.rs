
use nalgebra::{DMatrix};

////

type Mat = DMatrix<f64>;

fn print(m: Mat) {
    let shape = m.shape();
    print!("[");
    for row in 0..shape.0 {
        for col in 0..shape.1 {
            print!("\t{:?}\t", m[(row, col)]);
        }
        if row < shape.0-1 { println!(); }
    }
    println!("\t]");
}

////

fn main() {
    println!("Hello, world!");

    let m = DMatrix::from_row_slice(4, 3, &[
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 0.0
    ]);

    // println!("{:?}", m);
    print(m);
}
