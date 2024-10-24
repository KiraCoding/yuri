#![feature(repr_simd)]
#![feature(portable_simd)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub mod matrix;
pub mod ops;
mod vector;
pub mod vector_test;

pub use vector::Vector;
