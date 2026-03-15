//! The [`Pack`] trait and primitive implementations.

use byteorder::ByteOrder;
use pastey::paste;

use crate::Size;

/// Types that can be encoded to a binary representation.
pub trait Pack: Size {
    /// Encodes `self` to a binary representation.
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder;
}

macro_rules! impl_pack_number {
    ($t:ty) => {
        impl Pack for $t {
            fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
                paste!{ B::[<write_$t>](buffer, *self) }
            }
        }
    };
}

impl Pack for u8 {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        buffer[0] = *self;
    }
}
impl_pack_number!(u16);
impl_pack_number!(u32);
impl_pack_number!(u64);

impl Pack for i8 {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        buffer[0] = *self as u8;
    }
}

impl_pack_number!(i16);
impl_pack_number!(i32);
impl_pack_number!(i64);

impl_pack_number!(f32);
impl_pack_number!(f64);

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

macro_rules! impl_pack_tuple {
    ($($idx:tt),*) => {
        paste! {
            impl<$([<T$idx>],)*> Pack for ($([<T$idx>],)*)
            where
                $([<T$idx>]: Pack,)*
            {
                #[allow(unused)] // because we generate one more "let buffer = ..." statement than we need
                fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
                    $(
                        self.$idx.pack::<B>(buffer);
                        let buffer = &mut buffer[[<T$idx>]::SIZE..];
                    )*
                }
            }
        }
    };
}

impl_pack_tuple!();
impl_pack_tuple!(0);
impl_pack_tuple!(0, 1);
impl_pack_tuple!(0, 1, 2);
impl_pack_tuple!(0, 1, 2, 3);

impl<T> Pack for &T where T: Pack {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        T::pack::<B>(self, buffer)
    }
}

impl<T> Pack for &mut T where T: Pack {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        T::pack::<B>(self, buffer)
    }
}

impl<T> Pack for Option<T> where T: Pack {
    fn pack<B>(&self, buffer: &mut [u8]) where B: ByteOrder {
        match self {
            Some(x) => (true, x).pack::<B>(buffer),
            None => false.pack::<B>(buffer),
        }
    }
}

impl<T, const N: usize> Pack for [T; N] where T: Pack {
    fn pack<B>(&self, mut buffer: &mut [u8]) where B: ByteOrder {
        for value in self {
            value.pack::<B>(buffer);
            buffer = &mut buffer[T::SIZE..];
        }
    }
}
