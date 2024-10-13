use crate::ops::Sum;
use core::ops::{Add, Index};
use core::simd::num::{SimdFloat, SimdInt, SimdUint};
use core::simd::{Simd, SimdElement};

#[repr(simd)]
#[derive(Debug)]
pub struct Vector<T, const D: usize>([T; D])
where
    T: SimdElement;

impl<T, const D: usize> Default for Vector<T, D>
where
    T: SimdElement + Default,
{
    #[inline]
    fn default() -> Self {
        Self([T::default(); D])
    }
}

impl<T, const D: usize> From<[T; D]> for Vector<T, D>
where
    T: SimdElement,
{
    #[inline]
    fn from(value: [T; D]) -> Self {
        Self(value)
    }
}

impl<T, const D: usize> Index<usize> for Vector<T, D>
where
    T: SimdElement,
{
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

// impl<const D: usize> Add for Vector<f32, D> {
//     type Output = Self;

//     #[inline]
//     fn add(self, rhs: Self) -> Self::Output {
//         let (lhs_prefix, lhs_middle, lhs_suffix) = self.0.as_simd();
//         let (rhs_prefix, rhs_middle, rhs_suffix) = rhs.0.as_simd();

//         todo!()
//     }
// }

macro_rules! impl_vector {
    ($($t:ty),*$(,)?) => {
        $(
            impl<const D: usize> Sum for Vector<$t, D> {
                type Output = $t;

                #[inline]
                fn sum(self) -> Self::Output {
                    let (prefix, middle, suffix) = self.0.as_simd();

                    let mut sums = Simd::from_array([<$t>::default(); 64]);
                    sums[0] = prefix.iter().copied().sum();
                    sums[1] = suffix.iter().copied().sum();

                    middle.iter().copied().fold(sums, Simd::add).reduce_sum()
                }
            }
        )*
    };
}

impl_vector! {
    f32, f64,
    u8, u16, u32, u64, usize,
    i8, i16, i32, i64, isize,
}
