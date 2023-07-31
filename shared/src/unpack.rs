use byteorder::ByteOrder;

use crate::Size;

pub trait Unpack: Size {
    /// Decodes the type from a binary representation.
    fn unpack<B>(buffer: &[u8]) -> Self where B: ByteOrder ;
}

impl Unpack for u8 {
    fn unpack<B>(buffer: &[u8]) -> Self where B: ByteOrder {
        buffer[0]
    }
}

impl Unpack for u16 {
    fn unpack<B>(buffer: &[u8]) -> Self where B: ByteOrder {
        B::read_u16(buffer)
    }
}

impl Unpack for u32 {
    fn unpack<B>(buffer: &[u8]) -> Self where B: ByteOrder {
        B::read_u32(buffer)
    }
}

impl Unpack for u64 {
    fn unpack<B>(buffer: &[u8]) -> Self where B: ByteOrder {
        B::read_u64(buffer)
    }
}

impl Unpack for i8 {
    fn unpack<B>(buffer: &[u8]) -> Self where B: ByteOrder {
        buffer[0] as i8
    }
}

impl Unpack for i16 {
    fn unpack<B>(buffer: &[u8]) -> Self where B: ByteOrder {
        B::read_i16(buffer)
    }
}

impl Unpack for i32 {
    fn unpack<B>(buffer: &[u8]) -> Self where B: ByteOrder {
        B::read_i32(buffer)
    }
}

impl Unpack for i64 {
    fn unpack<B>(buffer: &[u8]) -> Self where B: ByteOrder {
        B::read_i64(buffer)
    }
}

impl Unpack for bool {
    fn unpack<B>(buffer: &[u8]) -> Self where B: ByteOrder {
        buffer[0] != 0
    }
}

impl Unpack for char {
    fn unpack<B>(buffer: &[u8]) -> Self where B: ByteOrder {
        // TODO: We might want to pass this error to the user, but would
        //       we really want to complicate the API for this rather
        //       exotic issue?
        char::from_u32(u32::unpack::<B>(buffer)).expect("Could not unpack 32-bit integer as char")
    }
}

// TODO: Abstract over the tuple size with a macro

impl Unpack for () {
    fn unpack<B>(_buffer: &[u8]) -> Self where B: ByteOrder {
        ()
    }
}

impl<T0> Unpack for (T0,) where T0: Unpack {
    fn unpack<B>(buffer: &[u8]) -> Self where B: ByteOrder {
        (T0::unpack::<B>(buffer),)
    }
}

impl<T0, T1> Unpack for (T0, T1) where T0: Unpack, T1: Unpack {
    fn unpack<B>(buffer: &[u8]) -> Self where B: ByteOrder {
        let x0 = T0::unpack::<B>(buffer);
        let buffer = &buffer[T0::SIZE..];
        let x1 = T1::unpack::<B>(buffer);
        (x0, x1)
    }
}

impl<T0, T1, T2> Unpack for (T0, T1, T2) where T0: Unpack, T1: Unpack, T2: Unpack {
    fn unpack<B>(buffer: &[u8]) -> Self where B: ByteOrder {
        let x0 = T0::unpack::<B>(buffer);
        let buffer = &buffer[T0::SIZE..];
        let x1 = T1::unpack::<B>(buffer);
        let buffer = &buffer[T1::SIZE..];
        let x2 = T2::unpack::<B>(buffer);
        (x0, x1, x2)
    }
}

impl<T0, T1, T2, T3> Unpack for (T0, T1, T2, T3) where T0: Unpack, T1: Unpack, T2: Unpack, T3: Unpack {
    fn unpack<B>(buffer: &[u8]) -> Self where B: ByteOrder {
        let x0 = T0::unpack::<B>(buffer);
        let buffer = &buffer[T0::SIZE..];
        let x1 = T1::unpack::<B>(buffer);
        let buffer = &buffer[T1::SIZE..];
        let x2 = T2::unpack::<B>(buffer);
        let buffer = &buffer[T2::SIZE..];
        let x3 = T3::unpack::<B>(buffer);
        (x0, x1, x2, x3)
    }
}

impl<T> Unpack for Option<T> where T: Unpack {
    fn unpack<B>(buffer: &[u8]) -> Self where B: ByteOrder {
        let is_some = bool::unpack::<B>(buffer);
        if is_some {
            Some(T::unpack::<B>(&buffer[bool::SIZE..]))
        } else {
            None
        }
    }
}
