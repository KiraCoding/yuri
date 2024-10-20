#![feature(repr_simd)]
#![feature(portable_simd)]

mod vector;
pub mod ops;
pub mod matrix;

pub use vector::Vector;
