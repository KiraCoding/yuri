use core::hint::black_box;
use yuri::{ops::Sum, Vector};

#[divan::bench]
fn vector_simd_sum() -> f64 {
    let vector = black_box(Vector::from([2.0; 999]));

    black_box(vector.sum())
}

#[divan::bench]
fn vector_linear_sum() -> f64 {
    let array = black_box([2.0; 999]);
    let iter = black_box(array.iter());
    black_box(iter.sum())
}

fn main() {
    divan::main();
}
