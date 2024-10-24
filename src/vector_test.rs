use core::mem::MaybeUninit;
use core::ops::{Add, Index};
use core::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};

pub union Vector<T, const D: usize>
where
    T: SimdElement,
    LaneCount<{ mpow2::<{ D }>() }>: SupportedLaneCount,
    [(); mpow2::<{ D }>() + len::<{ D }>()]:,
{
    data: [T; { mpow2::<{ D }>() + len::<{ D }>() }], // ignore this prob unaligned mess
    simd: [Simd<T, { mpow2::<{ D }>() }>; len::<{ D }>()],
}

impl<T, const D: usize> From<[T; D]> for Vector<T, D>
where
    T: SimdElement + Default,
    LaneCount<{ mpow2::<{ D }>() }>: SupportedLaneCount,
    [(); mpow2::<{ D }>() + len::<{ D }>()]:,
    [Simd<T, { mpow2::<{ D }>() }>; len::<D>()]: Default,
{
    #[inline]
    fn from(value: [T; D]) -> Self {
        let mut simd_buf: [Simd<T, { mpow2::<{ D }>() }>; len::<D>()] =
            unsafe { MaybeUninit::uninit().assume_init() };

        // this got unrolled after looking at the asm
        value
            .chunks(mpow2::<D>())
            .enumerate()
            .for_each(|(i, chunk)| {
                let mut arr = [T::default(); mpow2::<D>()];
                arr[..chunk.len()].copy_from_slice(chunk);
                simd_buf[i] = Simd::from_array(arr);
            });

        Self { simd: simd_buf }
    }
}

impl<T, const D: usize> Add for Vector<T, D>
where
    T: SimdElement,
    LaneCount<{ mpow2::<{ D }>() }>: SupportedLaneCount,
    Simd<T, { mpow2::<D>() }>: Add<Output = Simd<T, { mpow2::<D>() }>>,
    [(); mpow2::<{ D }>() + len::<{ D }>()]:,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let mut result_buf: [Simd<T, { mpow2::<D>() }>; len::<D>()] =
            unsafe { MaybeUninit::uninit().assume_init() };

        let lhs = unsafe { self.simd };
        let rhs = unsafe { rhs.simd };

        // this got unrolled after looking at the asm
        for (i, (l, r)) in lhs.iter().copied().zip(rhs.iter().copied()).enumerate() {
            result_buf[i] = l + r;
        }

        Self { simd: result_buf }
    }
}

// ignore this unaligned prob `.data` access
impl<T, const D: usize> Index<usize> for Vector<T, D>
where
    T: SimdElement,
    LaneCount<{ mpow2::<{ D }>() }>: SupportedLaneCount,
    [(); mpow2::<{ D }>() + len::<{ D }>()]:,
{
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        // Check bounds to ensure index is within 0..D
        assert!(
            index < D,
            "Index out of bounds: {} is not less {}",
            index,
            D
        );

        unsafe {
            &self.data[index]
        }
    }
}

pub const fn mpow2<const N: usize>() -> usize {
    match N {
        0 => 1,
        n if n >= 64 => 64,
        n => n.next_power_of_two(),
    }
}

pub const fn len<const N: usize>() -> usize {
    (N + mpow2::<N>() - 1) / mpow2::<N>()
}

#[test]
fn test() {
    let a = Vector::from([1.0, 2.0, 3.0, 4.0, 5.0]);
    let b = Vector::from([3.0, 2.0, 1.0, 0.0, 1.0]);
    dbg!(a[4]);
    dbg!(unsafe { (a + b).data });
}
