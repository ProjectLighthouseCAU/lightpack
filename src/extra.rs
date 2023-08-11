//! Additional utilities.

use byteorder::{LittleEndian, BigEndian, ByteOrder};

use crate::{Size, Pack, Unpack, unpack::{Result, self}};

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

/// A small convenience trait similar to [`Into`] and [`TryInto`].
pub trait UnpackInto<T> {
    /// Decodes to a target type. May panic if the buffer is too small.
    fn unpack_into<B>(self) -> unpack::Result<T> where B: ByteOrder;

    /// Decodes to a target type, erroring if the buffer is too small.
    fn unpack_safely_into<B>(self) -> unpack::Result<T> where B: ByteOrder;
}

impl<T> UnpackInto<T> for &[u8] where T: Unpack {
    fn unpack_into<B>(self) -> unpack::Result<T> where B: ByteOrder {
        T::unpack::<B>(self)
    }

    fn unpack_safely_into<B>(self) -> unpack::Result<T> where B: ByteOrder {
        T::unpack_safely::<B>(self)
    }
}

/// A small convenience trait that combines [`Pack`] and [`Size`].
pub trait PackSize {
    /// Encodes `self` to a binary representation and
    /// returns the encoded size.
    fn pack_size<B>(&self, buffer: &mut [u8]) -> usize where B: ByteOrder;
}

impl<T> PackSize for T where T: Pack {
    fn pack_size<B>(&self, buffer: &mut [u8]) -> usize where B: ByteOrder {
        self.pack::<B>(buffer);
        T::SIZE
    }
}
