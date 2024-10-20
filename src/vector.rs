use core::ops::{Add, Div, Mul, Sub};
use core::simd::{Simd, SimdElement};
use std::ops::AddAssign;

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
        )*
    };
}

// impl<T> Sum for Vector<T, 2>
// where
//     T: SimdElement + Default,
// {
//     type Output = Self;

//     #[inline]
//     fn sum(self) -> Self::Output {
//         let simd = Simd::<T, { mpow2::<2>() }>::load_or_default(&self.0);

//         Self(simd.reduce_sum())
//     }
// }

impl_vector! { 1, 2, 3, 4, 5 }

const fn mpow2<const N: usize>() -> usize {
    if N >= 64 {
        return 64;
    }
    if N == 0 {
        return 1;
    }
    let mut p = 1;
    while p < N {
        p <<= 1;
    }
    p
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
