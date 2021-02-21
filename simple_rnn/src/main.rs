
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use nalgebra::{DMatrix, DVector};

mod util;
mod matrix;
mod compile;

////

fn main() {
    println!("Hello, world!");

    let m = DMatrix::from_row_slice(3, 3, &[
        2.0, 0.0, 0.0,
        0.0, 2.0, 0.0,
        0.0, 0.0, 2.0,
    ]);

    let v = matrix::row(vec![1.0, 2.0, 3.0]);
    // let v = col(vec![1.0, 2.0, 3.0]);

    // let n = m.clone().transpose();

    // println!("{:?}", m);
    // print(m*n);

    matrix::print(v*m);
    // print(m*v);
}


/*

NOTES:

- keep agents/step functions in Python
    - just do compilation in Rust
- we want to optimize (list o_1 ... o_n) for (size, depth) where size takes sharing into account
- it should be possible to not use petgraph (maybe just for visualization?)

TODO:
- do some basic parsing/rewriting with egg
- read egg paper
- compile functions with egg
- figure out how to use rust from python (with PyO3?)
*/