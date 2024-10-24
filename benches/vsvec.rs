#![feature(portable_simd)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use core::hint::black_box;
use yuri::{vector, Vector};

#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

#[divan::bench]
fn original() -> Vector<f64, 3> {
    let a = vector![2.0; 3];
    let b = vector![2.0; 3];
    black_box(a + b)
}

#[divan::bench]
fn prototype() -> yuri::vector_test::Vector<f64, 3> {
    let a = yuri::vector_test::Vector::from([2.0; 3]);
    let b = yuri::vector_test::Vector::from([2.0; 3]);
    black_box(a + b)
}

fn main() {
    divan::main();
}
