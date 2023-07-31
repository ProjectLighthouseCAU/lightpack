use byteorder::ByteOrder;

use crate::Size;

pub trait Pack: Size {
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

impl Pack for bool {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        buffer[0] = *self as u8;
    }
}

impl Pack for char {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        (*self as u32).pack::<B>(buffer);
    }
}

// TODO: Abstract over the tuple size with a macro

impl Pack for () {
    fn pack<B>(&self, _buffer: &mut [u8]) where B: ByteOrder {
        // Do nothing
    }
}

impl<T0> Pack for (T0,) where T0: Pack {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        self.0.pack::<B>(buffer);
    }
}

impl<T0, T1> Pack for (T0, T1) where T0: Pack, T1: Pack {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        self.0.pack::<B>(buffer);
        let buffer = &mut buffer[T0::SIZE..];
        self.1.pack::<B>(buffer);
    }
}

impl<T0, T1, T2> Pack for (T0, T1, T2) where T0: Pack, T1: Pack, T2: Pack {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        self.0.pack::<B>(buffer);
        let buffer = &mut buffer[T0::SIZE..];
        self.1.pack::<B>(buffer);
        let buffer = &mut buffer[T1::SIZE..];
        self.2.pack::<B>(buffer);
    }
}

impl<T0, T1, T2, T3> Pack for (T0, T1, T2, T3) where T0: Pack, T1: Pack, T2: Pack, T3: Pack {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        self.0.pack::<B>(buffer);
        let buffer = &mut buffer[T0::SIZE..];
        self.1.pack::<B>(buffer);
        let buffer = &mut buffer[T1::SIZE..];
        self.2.pack::<B>(buffer);
        let buffer = &mut buffer[T2::SIZE..];
        self.3.pack::<B>(buffer);
    }
}
