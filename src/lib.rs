#![feature(repr_simd)]
#![feature(portable_simd)]

pub mod matrix;
pub mod ops;
mod vector;

pub use vector::Vector;
