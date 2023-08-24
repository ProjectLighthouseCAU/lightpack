//! The [`Size`] trait and primitive implementations.

/// Types that have an encoded size.
pub trait Size {
    /// The type's encoded size in bytes.
    const SIZE: usize;
}

impl Size for u8 {
    const SIZE: usize = 1;
}

impl Size for u16 {
    const SIZE: usize = 2;
}

impl Size for u32 {
    const SIZE: usize = 4;
}

impl Size for u64 {
    const SIZE: usize = 8;
}

impl Size for i8 {
    const SIZE: usize = 1;
}

impl Size for i16 {
    const SIZE: usize = 2;
}

impl Size for i32 {
    const SIZE: usize = 4;
}

impl Size for i64 {
    const SIZE: usize = 8;
}

impl Size for f32 {
    const SIZE: usize = 4;
}

impl Size for f64 {
    const SIZE: usize = 8;
}

impl Size for bool {
    const SIZE: usize = 1;
}

impl Size for char {
    const SIZE: usize = 4;
}

impl Size for () {
    const SIZE: usize = 0;
}

// TODO: Abstract over the tuple size with a macro

impl<T1> Size for (T1,) where T1: Size {
    const SIZE: usize = T1::SIZE;
}

impl<T1, T2> Size for (T1, T2) where T1: Size, T2: Size {
    const SIZE: usize = T1::SIZE + T2::SIZE;
}

impl<T1, T2, T3> Size for (T1, T2, T3) where T1: Size, T2: Size, T3: Size {
    const SIZE: usize = T1::SIZE + T2::SIZE + T3::SIZE;
}

impl<T1, T2, T3, T4> Size for (T1, T2, T3, T4) where T1: Size, T2: Size, T3: Size, T4: Size {
    const SIZE: usize = T1::SIZE + T2::SIZE + T3::SIZE + T4::SIZE;
}

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
