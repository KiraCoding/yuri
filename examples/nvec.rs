#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use core::hint::black_box;
use yuri::vector_test::Vector;

fn main() {
    black_box(create_add_vector());
}

#[inline(never)]
fn create_add_vector() -> Vector<f64, 100> {
    let a = black_box(Vector::from([1.0; 100]));
    let b = black_box(Vector::from([1.0; 100]));

    a + b
}
