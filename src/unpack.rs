use core::mem::{MaybeUninit, self};

use byteorder::ByteOrder;

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
}

/// The result type for unpacked results.
pub type Result<T> = core::result::Result<T, Error>;

/// Types that can be decoded from a binary representation.
pub trait Unpack: Size {
    /// Decodes the type from a binary representation.
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder, Self: Sized;
}

impl Unpack for u8 {
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        Ok(buffer[0])
    }
}

impl Unpack for u16 {
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        Ok(B::read_u16(buffer))
    }
}

impl Unpack for u32 {
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        Ok(B::read_u32(buffer))
    }
}

impl Unpack for u64 {
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        Ok(B::read_u64(buffer))
    }
}

impl Unpack for i8 {
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        Ok(buffer[0] as i8)
    }
}

impl Unpack for i16 {
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        Ok(B::read_i16(buffer))
    }
}

impl Unpack for i32 {
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        Ok(B::read_i32(buffer))
    }
}

impl Unpack for i64 {
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        Ok(B::read_i64(buffer))
    }
}

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

// TODO: Abstract over the tuple size with a macro

impl Unpack for () {
    fn unpack<B>(_buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        Ok(())
    }
}

impl<T0> Unpack for (T0,) where T0: Unpack {
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        Ok((T0::unpack::<B>(buffer)?,))
    }
}

impl<T0, T1> Unpack for (T0, T1) where T0: Unpack, T1: Unpack {
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        let x0 = T0::unpack::<B>(buffer)?;
        let buffer = &buffer[T0::SIZE..];
        let x1 = T1::unpack::<B>(buffer)?;
        Ok((x0, x1))
    }
}

impl<T0, T1, T2> Unpack for (T0, T1, T2) where T0: Unpack, T1: Unpack, T2: Unpack {
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        let x0 = T0::unpack::<B>(buffer)?;
        let buffer = &buffer[T0::SIZE..];
        let x1 = T1::unpack::<B>(buffer)?;
        let buffer = &buffer[T1::SIZE..];
        let x2 = T2::unpack::<B>(buffer)?;
        Ok((x0, x1, x2))
    }
}

impl<T0, T1, T2, T3> Unpack for (T0, T1, T2, T3) where T0: Unpack, T1: Unpack, T2: Unpack, T3: Unpack {
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        let x0 = T0::unpack::<B>(buffer)?;
        let buffer = &buffer[T0::SIZE..];
        let x1 = T1::unpack::<B>(buffer)?;
        let buffer = &buffer[T1::SIZE..];
        let x2 = T2::unpack::<B>(buffer)?;
        let buffer = &buffer[T2::SIZE..];
        let x3 = T3::unpack::<B>(buffer)?;
        Ok((x0, x1, x2, x3))
    }
}

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
        
        for i in 0..N {
            // NOTE: If `T::unpack` throws an error, then the `Drop` implementation of
            // the already parsed `T`s will not be called. This could leak external
            // resources held by those `T`s, but should otherwise not be unsafe. 
            // See https://doc.rust-lang.org/nomicon/unchecked-uninit.html
            // For plain-old data types, which we expect to be the dominant use-case,
            // this is thus not an issue.
            result[i] = MaybeUninit::new(T::unpack::<B>(buffer)?);
            buffer = &buffer[T::SIZE..];
        }

        // SAFETY: We use the trick from https://github.com/bincode-org/bincode/blob/224e41274b/src/de/impl_core.rs#L184
        // The array is initialized, `MaybeUninit<T>` and `T` have the same layout and
        // `MaybeUninit` does drop drop, so transmuting `[MaybeUninit<T>; N]` to `[T; N]` is safe.
        // We cannot use `mem::transmute` since the compiler doesn't accept it for generic lengths,
        // see https://github.com/rust-lang/rust/issues/61956.
        Ok(unsafe {
            (&result as *const _ as *const [T; N]).read()
        })
    }
}
