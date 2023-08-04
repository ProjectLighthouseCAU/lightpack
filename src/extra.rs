//! Additional utilities.

use byteorder::{LittleEndian, BigEndian, ByteOrder};

use crate::{Size, Pack, Unpack, unpack::Result};

/// A wrapper that always encodes the type as little endian.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub struct LE<T>(pub T);

/// A wrapper that always encodes the type as big endian.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub struct BE<T>(pub T);

impl<T> Size for LE<T> where T: Size {
    const SIZE: usize = T::SIZE;
}

impl<T> Size for BE<T> where T: Size {
    const SIZE: usize = T::SIZE;
}

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
