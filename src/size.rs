//! The [`Size`] trait and primitive implementations.

use core::mem::size_of;
use pastey::paste;

/// Types that have an encoded size.
pub trait Size {
    /// The type's encoded size in bytes.
    const SIZE: usize;
}

macro_rules! impl_size_primitive {
    ($t:ty) => {
        impl Size for $t {
            const SIZE: usize = size_of::<$t>();
        }
    };
}

impl_size_primitive!(u8);
impl_size_primitive!(u16);
impl_size_primitive!(u32);
impl_size_primitive!(u64);

impl_size_primitive!(i8);
impl_size_primitive!(i16);
impl_size_primitive!(i32);
impl_size_primitive!(i64);

impl_size_primitive!(f32);
impl_size_primitive!(f64);

impl_size_primitive!(bool);

impl_size_primitive!(char);

impl_size_primitive!(());

macro_rules! sum {
    ($e:expr, $($es:expr),+) => {
        $e + sum!($($es),+)
    };
    ($e:expr) => {
        $e
    }
}

macro_rules! impl_size_tuple {
    ($($idx:tt),+) => {
        paste! {
            impl<$([<T$idx>],)*> Size for ($([<T$idx>],)+)
            where
                $([<T$idx>]: Size,)+
            {
                const SIZE: usize = sum!($( [<T$idx>]::SIZE ),*);
            }
        }
    };
}

impl_size_tuple!(0);
impl_size_tuple!(0, 1);
impl_size_tuple!(0, 1, 2);
impl_size_tuple!(0, 1, 2, 3);

impl<T> Size for &T where T: Size {
    const SIZE: usize = T::SIZE;
}

impl<T> Size for &mut T where T: Size {
    const SIZE: usize = T::SIZE;
}

impl<T> Size for Option<T> where T: Size {
    const SIZE: usize = T::SIZE + 1;
}

impl<T, const N: usize> Size for [T; N] where T: Size {
    const SIZE: usize = T::SIZE * N;
}
