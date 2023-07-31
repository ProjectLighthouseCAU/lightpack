use byteorder::ByteOrder;

pub trait Pack {
    /// Encodes `self` to a binary representation.
    fn pack<B>(&self, buffer: &mut [u8]) -> usize where B: ByteOrder;
}

// TODO: Abstract over this with a macro

impl Pack for u8 {
    fn pack<B>(&self, buffer: &mut [u8]) -> usize where B: ByteOrder {
        buffer[0] = *self;
        1
    }
}

impl Pack for u16 {
    fn pack<B>(&self, buffer: &mut [u8]) -> usize where B: ByteOrder {
        B::write_u16(buffer, *self);
        2
    }
}

impl Pack for u32 {
    fn pack<B>(&self, buffer: &mut [u8]) -> usize where B: ByteOrder {
        B::write_u32(buffer, *self);
        4
    }
}

impl Pack for u64 {
    fn pack<B>(&self, buffer: &mut [u8]) -> usize where B: ByteOrder {
        B::write_u64(buffer, *self);
        8
    }
}


impl Pack for i8 {
    fn pack<B>(&self, buffer: &mut [u8]) -> usize where B: ByteOrder {
        buffer[0] = *self as u8;
        1
    }
}

impl Pack for i16 {
    fn pack<B>(&self, buffer: &mut [u8]) -> usize where B: ByteOrder {
        B::write_i16(buffer, *self);
        2
    }
}

impl Pack for i32 {
    fn pack<B>(&self, buffer: &mut [u8]) -> usize where B: ByteOrder {
        B::write_i32(buffer, *self);
        4
    }
}

impl Pack for i64 {
    fn pack<B>(&self, buffer: &mut [u8]) -> usize where B: ByteOrder {
        B::write_i64(buffer, *self);
        8
    }
}
