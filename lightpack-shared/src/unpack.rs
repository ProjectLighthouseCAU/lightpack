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
