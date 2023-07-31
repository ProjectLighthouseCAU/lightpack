use byteorder::ByteOrder;

pub trait Unpack {
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
