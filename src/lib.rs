#![feature(repr_simd)]
#![feature(portable_simd)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

mod vector;
pub mod ops;
pub mod matrix;

pub use vector::Vector;
