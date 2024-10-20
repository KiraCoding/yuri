use yuri::{ops::Dot, vector, Vector};

fn main() {
    let a = vector![2.0; 3];
    let b = vector![3.0, 2.0, 1.0];

    dbg!(a.dot(b));
}

