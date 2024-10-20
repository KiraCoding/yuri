use yuri::{vector, Vector};

fn main() {
    let a = vector![2.0; 3];
    let b = vector![3.0; 3];

    dbg!(a / b);
}