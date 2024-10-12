use core::simd::SimdElement;

pub struct Matrix<T, const R: usize, const C: usize>([[T; R]; C])
where
    T: SimdElement;
