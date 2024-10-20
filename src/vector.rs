use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use core::simd::num::SimdFloat;
use core::simd::{Simd, SimdElement};

use crate::ops::Sum;

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

macro_rules! impl_vector {
    ($($n:expr),+$(,)?) => {
        $(
            impl<T> Add for Vector<T, $n>
            where
                T: SimdElement + Default,
                Simd<T, { mpow2::<$n>() }>: Add<Output = Simd<T, { mpow2::<$n>() }>>,
            {
                type Output = Self;

                #[inline]
                fn add(self, rhs: Self) -> Self::Output {
                    let lhs = Simd::<T, { mpow2::<$n>() }>::load_or_default(&self.0);
                    let rhs = Simd::<T, { mpow2::<$n>() }>::load_or_default(&rhs.0);

                    Self(unsafe { (lhs + rhs)[..$n].try_into().unwrap_unchecked() })
                }
            }

            impl<T> AddAssign for Vector<T, $n>
            where
                T: SimdElement + Default,
                Simd<T, { mpow2::<$n>() }>: Add<Output = Simd<T, { mpow2::<$n>() }>>,
            {
                #[inline]
                fn add_assign(&mut self, rhs: Self) {
                    let lhs = Simd::<T, { mpow2::<$n>() }>::load_or_default(&self.0);
                    let rhs = Simd::<T, { mpow2::<$n>() }>::load_or_default(&rhs.0);

                    self.0 = unsafe { (lhs + rhs)[..$n].try_into().unwrap_unchecked() };
                }
            }

            impl<T> Sub for Vector<T, $n>
            where
                T: SimdElement + Default,
                Simd<T, { mpow2::<$n>() }>: Sub<Output = Simd<T, { mpow2::<$n>() }>>,
            {
                type Output = Self;

                #[inline]
                fn sub(self, rhs: Self) -> Self::Output {
                    let lhs = Simd::<T, { mpow2::<$n>() }>::load_or_default(&self.0);
                    let rhs = Simd::<T, { mpow2::<$n>() }>::load_or_default(&rhs.0);

                    Self(unsafe { (lhs - rhs)[..$n].try_into().unwrap_unchecked() })
                }
            }

            impl<T> SubAssign for Vector<T, $n>
            where
                T: SimdElement + Default,
                Simd<T, { mpow2::<$n>() }>: Sub<Output = Simd<T, { mpow2::<$n>() }>>,
            {
                #[inline]
                fn sub_assign(&mut self, rhs: Self) {
                    let lhs = Simd::<T, { mpow2::<$n>() }>::load_or_default(&self.0);
                    let rhs = Simd::<T, { mpow2::<$n>() }>::load_or_default(&rhs.0);

                    self.0 = unsafe { (lhs - rhs)[..$n].try_into().unwrap_unchecked() };
                }
            }

            impl<T> Mul for Vector<T, $n>
            where
                T: SimdElement + Default,
                Simd<T, { mpow2::<$n>() }>: Mul<Output = Simd<T, { mpow2::<$n>() }>>,
            {
                type Output = Self;

                #[inline]
                fn mul(self, rhs: Self) -> Self::Output {
                    let lhs = Simd::<T, { mpow2::<$n>() }>::load_or_default(&self.0);
                    let rhs = Simd::<T, { mpow2::<$n>() }>::load_or_default(&rhs.0);

                    Self(unsafe { (lhs * rhs)[..$n].try_into().unwrap_unchecked() })
                }
            }

            impl<T> MulAssign for Vector<T, $n>
            where
                T: SimdElement + Default,
                Simd<T, { mpow2::<$n>() }>: Mul<Output = Simd<T, { mpow2::<$n>() }>>,
            {
                #[inline]
                fn mul_assign(&mut self, rhs: Self) {
                    let lhs = Simd::<T, { mpow2::<$n>() }>::load_or_default(&self.0);
                    let rhs = Simd::<T, { mpow2::<$n>() }>::load_or_default(&rhs.0);

                    self.0 = unsafe { (lhs * rhs)[..$n].try_into().unwrap_unchecked() };
                }
            }

            impl<T> Div for Vector<T, $n>
            where
                T: SimdElement + Default,
                Simd<T, { mpow2::<$n>() }>: Div<Output = Simd<T, { mpow2::<$n>() }>>,
            {
                type Output = Self;

                #[inline]
                fn div(self, rhs: Self) -> Self::Output {
                    let lhs = Simd::<T, { mpow2::<$n>() }>::load_or_default(&self.0);
                    let rhs = Simd::<T, { mpow2::<$n>() }>::load_or_default(&rhs.0);

                    Self(unsafe { (lhs / rhs)[..$n].try_into().unwrap_unchecked() })
                }
            }

            impl<T> DivAssign for Vector<T, $n>
            where
                T: SimdElement + Default,
                Simd<T, { mpow2::<$n>() }>: Div<Output = Simd<T, { mpow2::<$n>() }>>,
            {
                #[inline]
                fn div_assign(&mut self, rhs: Self) {
                    let lhs = Simd::<T, { mpow2::<$n>() }>::load_or_default(&self.0);
                    let rhs = Simd::<T, { mpow2::<$n>() }>::load_or_default(&rhs.0);

                    self.0 = unsafe { (lhs / rhs)[..$n].try_into().unwrap_unchecked() };
                }
            }
        )*
    };
}

impl Sum for Vector<f32, 2> {
    type Output = f32;

    fn sum(self) -> Self::Output {
        Simd::<f32, { mpow2::<2>() }>::load_or_default(&self.0).reduce_sum()
    }
}

impl_vector![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 17, 33, 64];

const fn mpow2<const N: usize>() -> usize {
    match N {
        0 => 1,
        n if n >= 64 => 64,
        n => 1 << (64 - (n - 1).leading_zeros()),
    }
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
