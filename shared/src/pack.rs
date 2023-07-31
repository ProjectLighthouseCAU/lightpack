use byteorder::ByteOrder;

pub trait Pack {
    /// Encodes `self` to a binary representation.
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder;
}

// TODO: Abstract over this with a macro

impl Pack for u8 {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        buffer[0] = *self;
    }
}

impl Pack for u16 {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        B::write_u16(buffer, *self);
    }
}

impl Pack for u32 {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        B::write_u32(buffer, *self);
    }
}

impl Pack for u64 {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        B::write_u64(buffer, *self);
    }
}


impl Pack for i8 {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        buffer[0] = *self as u8;
    }
}

impl Pack for i16 {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        B::write_i16(buffer, *self);
    }
}

impl Pack for i32 {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        B::write_i32(buffer, *self);
    }
}

impl Pack for i64 {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        B::write_i64(buffer, *self);
    }
}
