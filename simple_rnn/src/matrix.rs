
use nalgebra::{DMatrix, DVector};

////

pub type Int = i32;
pub type Num = f64;
pub type Mat = DMatrix<Num>;

pub fn print(m: Mat) {
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

pub fn row(v: Vec<Num>) -> Mat { DMatrix::from_row_slice(1, v.len(), &v) }
pub fn col(v: Vec<Num>) -> Mat { DMatrix::from_row_slice(v.len(), 1, &v) }
pub fn sing(n: Num)     -> Mat { row(vec![n]) }
pub fn zrow(n: usize)   -> Mat { DMatrix::repeat(1, n, 0.0) }
pub fn zcol(n: usize)   -> Mat { DMatrix::repeat(n, 1, 0.0) }
pub fn zsqr(n: usize)   -> Mat { DMatrix::repeat(n, n, 0.0) }
pub fn zmat(r: usize, c: usize) -> Mat { DMatrix::repeat(r, c, 0.0) }

