use yuri::{vector, Vector};

fn main() {
    let a = vector![2.0; 5];
    let b = vector![3.0; 5];

    dbg!(add_test(a, b));
}

#[inline(never)]
fn add_test(a: Vector<f64, 5>, b: Vector<f64, 5>) -> Vector<f64, 5> {
    a + b
}
