// Necessary to link to openblas. See
// https://github.com/bluss/rust-ndarray/issues/251#issuecomment-267951439 .
extern crate openblas_src;

#[macro_use(s)]
extern crate ndarray;

use std::iter;
use ndarray::{aview1, arr1, arr2, stack, Axis, Array, Dim};

fn print_title(title: &str) {
    let line = iter::repeat("-").take(title.chars().count()).collect::<String>();
    println!("\n{}", line);
    println!("{}", title);
    println!("{}", line);
}

fn exo7() {
    print_title("Exo 7");

    let factors = arr2(&[[1, 3, 6]]);
    let factors_column = factors.t();
    let ones = Array::from_shape_vec((1, 19), vec![1; 19]).unwrap();
    let wide_matrix = factors_column.dot(&ones);
    println!("Wide matrix = \n{:?}", wide_matrix);

    let wide_matrix2 = stack(Axis(1), &vec![factors_column; 19]).unwrap();
    println!("Wide matrix again = \n{:?}", wide_matrix2);
}

fn exo8() {
    print_title("Exo 8");

    let M = Array::from_iter(1..145).into_shape((12, 12)).unwrap();
    println!("M = \n{:?}", M);

    let subblock1 = M.slice(s![0..6, 6..12]);
    println!("M[[0, 5], [6, 11]] = \n{:?}", subblock1);

    let subblock2_indices = &[0, 1, 4, 5, 8, 9];
    let subblock2 = M.select(Axis(0), subblock2_indices).select(Axis(1), subblock2_indices);
    println!("M[{:?}, {:?}] = \n{:?}", subblock2_indices, subblock2_indices, subblock2);

    let evens = Array::from_iter((0..12).flat_map(|i| {
        (0..12).filter_map(|j| {
            if (i + j) % 2 == 0 { Some(M[(i, j)]) } else { None }
        }).collect::<Vec<_>>()
    })).into_shape((12, 6)).unwrap();
    println!("M[[i, j] if i+j even] = \n{:?}", evens);
}

fn main() {
    print_title("Exos 1-6: ok");
    exo7();
    exo8();
}
