#![feature(portable_simd)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use yuri::{vector, Vector};

#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

#[divan::bench_group]
mod add {
    use core::hint::black_box;
    use yuri::{vector, Vector};

    #[divan::bench]
    fn scalar() -> [f32; 3] {
        let a = [2.0f32; 3];
        let b = [2.0f32; 3];

        let mut result = [0.0f32; 3];

        for (r, (&x, &y)) in result.iter_mut().zip(a.iter().zip(b.iter())) {
            *r = black_box(x + y);
        }

        result
    }

    #[divan::bench]
    fn simd() -> Vector<f32, 3> {
        let a = vector![2.0f32; 3];
        let b = vector![2.0f32; 3];

        black_box(a + b)
    }
}

#[divan::bench_group]
mod add_assign {
    use core::hint::black_box;
    use yuri::{vector, Vector};

    #[divan::bench]
    fn scalar() -> [f32; 3] {
        let mut a = [2.0f32; 3];
        let b = [2.0f32; 3];

        for (x, &y) in a.iter_mut().zip(b.iter()) {
            *x = black_box(*x + y);
        }

        a
    }

    #[divan::bench]
    fn simd() -> Vector<f32, 3> {
        let mut a = vector![2.0f32; 3];
        let b = vector![2.0f32; 3];

        black_box(a += b);

        a
    }
}

#[divan::bench_group]
mod sub {
    use core::hint::black_box;
    use yuri::{vector, Vector};

    #[divan::bench]
    fn scalar() -> [f32; 3] {
        let a = [2.0f32; 3];
        let b = [2.0f32; 3];

        let mut result = [0.0f32; 3];

        for (r, (&x, &y)) in result.iter_mut().zip(a.iter().zip(b.iter())) {
            *r = black_box(x - y);
        }

        result
    }

    #[divan::bench]
    fn simd() -> Vector<f32, 3> {
        let a = vector![2.0f32; 3];
        let b = vector![2.0f32; 3];

        black_box(a - b)
    }
}

#[divan::bench_group]
mod mul {
    use core::hint::black_box;
    use yuri::{vector, Vector};

    #[divan::bench]
    fn scalar() -> [f32; 3] {
        let a = [2.0f32; 3];
        let b = [2.0f32; 3];

        let mut result = [0.0f32; 3];

        for (r, (&x, &y)) in result.iter_mut().zip(a.iter().zip(b.iter())) {
            *r = black_box(x * y);
        }

        result
    }

    #[divan::bench]
    fn simd() -> Vector<f32, 3> {
        let a = vector![2.0f32; 3];
        let b = vector![2.0f32; 3];

        black_box(a * b)
    }
}

#[divan::bench_group]
mod div {
    use core::hint::black_box;
    use yuri::{vector, Vector};

    #[divan::bench]
    fn scalar() -> [f32; 3] {
        let a = [2.0f32; 3];
        let b = [2.0f32; 3];

        let mut result = [0.0f32; 3];

        for (r, (&x, &y)) in result.iter_mut().zip(a.iter().zip(b.iter())) {
            *r = black_box(x / y);
        }

        result
    }

    #[divan::bench]
    fn simd() -> Vector<f32, 3> {
        let a = vector![2.0f32; 3];
        let b = vector![2.0f32; 3];

        black_box(a / b)
    }
}

fn main() {
    divan::main();
}

use divan::black_box;

#[divan::bench]
fn original() -> Vector<f64, 64> {
    let a = vector![2.0; 64];
    let b = vector![2.0; 64];
    black_box(a + b)
}

#[divan::bench]
fn prototype() -> yuri::vector_test::Vector<f64, 64> {
    let a = yuri::vector_test::Vector::from([2.0; 64]);
    let b = yuri::vector_test::Vector::from([2.0; 64]);
    black_box(a + b)
}