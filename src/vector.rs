use crate::ops::{Dot, Sum};
use core::ops::{Add, Index, Mul};
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

impl<const D: usize> Add for Vector<f32, D> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let (lhs_prefix, lhs_middle, lhs_suffix) = self.0.as_simd::<64>();
        let (rhs_prefix, rhs_middle, rhs_suffix) = rhs.0.as_simd::<64>();

        let mut result = [0.0; D];
        let prefix_len = lhs_prefix.len();
        let suffix_len = lhs_suffix.len();
        let middle_start = prefix_len;

        lhs_prefix
            .iter()
            .copied()
            .zip(rhs_prefix.iter().copied())
            .enumerate()
            .for_each(|(i, (l, r))| result[i] = l + r);

        for i in 0..suffix_len {
            result[D - suffix_len + i] = lhs_suffix[i] + rhs_suffix[i];
        }

        for (i, (l, r)) in lhs_middle.iter().zip(rhs_middle.iter()).enumerate() {
            let simd_result = *l + *r;
            let simd_array = simd_result.to_array();
            let start_idx = middle_start + i * simd_array.len();
            result[start_idx..start_idx + simd_array.len()].copy_from_slice(&simd_array);
        }

        Self(result)
    }
}

impl<const D: usize> Mul for Vector<f32, D> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let (lhs_prefix, lhs_middle, lhs_suffix) = self.0.as_simd::<64>();
        let (rhs_prefix, rhs_middle, rhs_suffix) = rhs.0.as_simd::<64>();

        let mut result = [0.0; D];
        let prefix_len = lhs_prefix.len();
        let suffix_len = lhs_suffix.len();
        let middle_start = prefix_len;

        for i in 0..prefix_len {
            result[i] = lhs_prefix[i] * rhs_prefix[i];
        }

        for i in 0..suffix_len {
            result[D - suffix_len + i] = lhs_suffix[i] * rhs_suffix[i];
        }

        for (i, (l, r)) in lhs_middle.iter().zip(rhs_middle.iter()).enumerate() {
            let simd_result = *l * *r;
            let simd_array = simd_result.to_array();
            let start_idx = middle_start + i * simd_array.len();
            result[start_idx..start_idx + simd_array.len()].copy_from_slice(&simd_array);
        }

        Self(result)
    }
}

impl<const D: usize> Dot for Vector<f32, D> {
    type Output = f32;

    fn dot(self, rhs: Self) -> Self::Output {
        (self * rhs).sum()
    }
}

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

/// `vector![]`
#[macro_export]
macro_rules! vector {
    [$elem:expr; $n:expr] => {
        $crate::Vector::from([$elem; $n])
    };
    [$($elem:expr),+$(,)?] => {
        $crate::Vector::from([$($elem),*])
    };
}
