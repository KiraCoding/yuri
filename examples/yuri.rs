use core::hint::black_box;
use yuri::{ops::Sum, Vector};

fn main() {
    let left = Vector::from([1.0, 2.0, 3.0]);
    let right =  Vector::from([3.0, 2.0, 1.0]);

    dbg!(left + right);
}