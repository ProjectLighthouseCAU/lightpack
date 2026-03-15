//! The [`Unpack`] trait and primitive implementations.

use core::mem::{MaybeUninit, self};

use byteorder::ByteOrder;
use pastey::paste;

use crate::Size;

/// An error during unpacking.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Error {
    InvalidChar(u32),
    InvalidEnumValueU8(u8),
    InvalidEnumValueU16(u16),
    InvalidEnumValueU32(u32),
    InvalidEnumValueU64(u64),
    InvalidEnumValueI8(i8),
    InvalidEnumValueI16(i16),
    InvalidEnumValueI32(i32),
    InvalidEnumValueI64(i64),
    BufferTooSmall { actual: usize, expected: usize },
    /// A user-defined error with some user-defined error code.
    Custom(u32),
}

/// The result type for unpacked results.
pub type Result<T> = core::result::Result<T, Error>;

/// Types that can be decoded from a binary representation.
pub trait Unpack: Size {
    /// Decodes the type from a binary representation.
    /// Panics if the buffer is too small.
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder, Self: Sized;

    /// Decodes the type and returns an error if the buffer is too small.
    fn unpack_safely<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder, Self: Sized {
        if buffer.len() < Self::SIZE {
            Err(Error::BufferTooSmall { actual: buffer.len(), expected: Self::SIZE })
        } else {
            Self::unpack::<B>(buffer)
        }
    }
}

macro_rules! impl_unpack_number {
    ($t:ty) => {
        impl Unpack for $t {
            fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder, Self: Sized {
                paste!{ Ok(B::[<read_$t>](buffer)) }
            }
        }
    };
}

impl Unpack for u8 {
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        Ok(buffer[0])
    }
}

impl_unpack_number!(u16);
impl_unpack_number!(u32);
impl_unpack_number!(u64);

impl Unpack for i8 {
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        Ok(buffer[0] as i8)
    }
}

impl_unpack_number!(i16);
impl_unpack_number!(i32);
impl_unpack_number!(i64);

impl_unpack_number!(f32);
impl_unpack_number!(f64);

impl Unpack for bool {
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        Ok(buffer[0] != 0)
    }
}

impl Unpack for char {
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        let value = u32::unpack::<B>(buffer)?;
        char::from_u32(value).ok_or(Error::InvalidChar(value))
    }
}

macro_rules! impl_unpack_tuple {
    ($($idx:tt),*) => {
        paste! {
            impl<$([<T$idx>],)*> Unpack for ($([<T$idx>],)*)
            where
                $([<T$idx>]: Unpack,)*
            {
                #[allow(unused)] // because we generate one more "let buffer = ..." statement than we need
                fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
                    $(
                        let [<t$idx>] = [<T$idx>]::unpack::<B>(buffer)?;
                        let buffer = &buffer[[<T$idx>]::SIZE..];
                    )*
                    Ok(($([<t$idx>],)*))
                }
            }
        }
    };
}

impl Unpack for () {
    fn unpack<B>(_buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        Ok(())
    }
}

impl_unpack_tuple!(0);
impl_unpack_tuple!(0, 1);
impl_unpack_tuple!(0, 1, 2);
impl_unpack_tuple!(0, 1, 2, 3);

impl<T> Unpack for Option<T> where T: Unpack {
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        let is_some = bool::unpack::<B>(buffer)?;
        if is_some {
            Ok(Some(T::unpack::<B>(&buffer[bool::SIZE..])?))
        } else {
            Ok(None)
        }
    }
}

impl<T, const N: usize> Unpack for [T; N] where T: Unpack {
    fn unpack<B>(mut buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        // Unfortunately, Rust doesn't provide a great way to initialize
        // arrays dynamically without the overhead of double initialization
        // (which additionally would require a `T: Default + Copy` bound
        // or similar). Therefore we'll use uninitialized memory as per
        // this trick: https://doc.rust-lang.org/nomicon/unchecked-uninit.html

        if N == 0 {
            // SAFETY: Empty arrays are always inhabited, without further invariants
            return Ok(unsafe { mem::zeroed() });
        }

        // SAFETY: The type we are claimining to initialize is an array of `MaybeUninit`s
        // which do not require initialization. For more details on this pattern
        // see https://doc.rust-lang.org/nomicon/unchecked-uninit.html
        let mut result: [MaybeUninit<T>; N] = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        // NOTE: If `T::unpack` throws an error in the loop, we need to make
        // sure that the already parsed `T`s will be dropped. Therefore we'll
        // use a `Guard` that drops all parsed `T`s if the function exits
        // before the array is fully parsed (and `mem::forget` is called).

        // Inspired by https://github.com/bincode-org/bincode/blob/224e41274b/src/de/impl_core.rs#L31
        // which is MIT-licensed (Copyright (c) 2014 Ty Overby).

        struct Guard<'a, T, const N: usize> {
            result: &'a mut [MaybeUninit<T>; N],
            initialized_count: usize,
        }

        impl<'a, T, const N: usize> Drop for Guard<'a, T, N> {
            fn drop(&mut self) {
                // SAFETY: The first `initialized_count` values are guaranteed to be initialized,
                // see the for-loop. The cast to `*mut [T]` is safe since `MaybeUninit<T>` and `T`
                // have the same memory layout and we have mutable/exclusive access already.
                unsafe {
                    let initialized: &mut [MaybeUninit<T>] = self.result.get_unchecked_mut(..self.initialized_count);
                    core::ptr::drop_in_place(initialized as *mut [MaybeUninit<T>] as *mut [T]);
                }
            }
        }

        let mut guard = Guard { result: &mut result, initialized_count: 0 };
        
        for i in 0..N {
            guard.result[i] = MaybeUninit::new(T::unpack::<B>(buffer)?);
            guard.initialized_count += 1;
            buffer = &buffer[T::SIZE..];
        }

        // The array is fully initialized, so skip dropping the parsed `T`s.
        mem::forget(guard);

        // SAFETY: The array is initialized, `MaybeUninit<T>` and `T` have the same layout and
        // `MaybeUninit` does not drop, so transmuting `[MaybeUninit<T>; N]` to `[T; N]` is safe.
        // We cannot use `mem::transmute` since the compiler doesn't accept it for generic lengths,
        // see https://github.com/rust-lang/rust/issues/61956.
        Ok(unsafe {
            (&result as *const _ as *const [T; N]).read()
        })
    }
}
