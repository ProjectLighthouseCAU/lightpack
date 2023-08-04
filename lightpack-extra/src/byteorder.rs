use byteorder::{LittleEndian, BigEndian, ByteOrder};
use lightpack::{Size, Pack, Unpack, unpack::Result};

/// A wrapper that always encodes the type as little endian.
#[derive(Size)]
pub struct LE<T>(T);

/// A wrapper that always encodes the type as big endian.
#[derive(Size)]
pub struct BE<T>(T);

impl<T> Pack for LE<T> where T: Pack {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        self.0.pack::<LittleEndian>(buffer)
    }
}

impl<T> Pack for BE<T> where T: Pack {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        self.0.pack::<BigEndian>(buffer)
    }
}

impl<T> Unpack for LE<T> where T: Unpack {
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        Ok(Self(T::unpack::<LittleEndian>(buffer)?))
    }
}

impl<T> Unpack for BE<T> where T: Unpack {
    fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: ByteOrder {
        Ok(Self(T::unpack::<BigEndian>(buffer)?))
    }
}

// TODO: Write unit tests
