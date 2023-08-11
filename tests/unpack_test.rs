use std::sync::atomic::{AtomicUsize, Ordering};

use byteorder::{BigEndian, LittleEndian};
use lightpack::{Unpack, Size, unpack::{Error, Result}, extra::{BE, LE, UnpackInto}};

#[test]
fn unsigned_ints() {
    assert_eq!(u8::unpack::<BigEndian>(&[4]), Ok(4));
    assert_eq!(u16::unpack::<BigEndian>(&[1, 0]), Ok(256));
    assert_eq!(u16::unpack::<LittleEndian>(&[0, 1]), Ok(256));
}

#[test]
fn signed_ints() {
    assert_eq!(i8::unpack::<BigEndian>(&[255]), Ok(-1));
}

#[test]
fn basic_structs() {
    #[derive(Size, Unpack, Clone, Copy, Debug, PartialEq, Eq)]
    #[allow(dead_code)]
    struct X {
        x: u8,
        y: u16,
    }

    #[derive(Size, Unpack, Clone, Copy, Debug, PartialEq, Eq)]
    #[allow(dead_code)]
    struct Y {
        x0: X,
        x1: X,
        x2: bool,
    }

    #[derive(Size, Unpack, Clone, Copy, Debug, PartialEq, Eq)]
    struct Tuple(X, Y);

    #[derive(Size, Unpack, Clone, Copy, Debug, PartialEq, Eq)]
    struct Unit;

    let x0 = X { x: 3, y: 4 };
    let x1 = X { x: 2, y: 8 };
    let y = Y { x0, x1, x2: true };

    assert_eq!(X::unpack::<BigEndian>(&[3, 0, 4]), Ok(x0));
    assert_eq!(X::unpack::<LittleEndian>(&[3, 4, 0]), Ok(x0));
    assert_eq!(X::unpack::<BigEndian>(&[2, 0, 8]), Ok(x1));
    assert_eq!(Tuple::unpack::<BigEndian>(&[3, 0, 4, 3, 0, 4, 2, 0, 8, 1]), Ok(Tuple(x0, y)));
    assert_eq!(Unit::unpack::<BigEndian>(&[]), Ok(Unit));
    assert_eq!(Unit::unpack::<BigEndian>(&[1, 2]), Ok(Unit));
}

#[test]
fn generic_structs() {
    #[derive(Size, Unpack, Debug, PartialEq, Eq)]
    #[allow(dead_code)]
    struct Wrap<T>(T);

    #[derive(Size, Unpack, Debug, PartialEq, Eq)]
    #[allow(dead_code)]
    struct Pair<L, R> {
        left: L,
        right: R,
    }

    #[derive(Size, Unpack, Debug, PartialEq, Eq)]
    #[allow(dead_code)]
    struct HasField {
        wrap: Wrap<u32>,
    }

    #[derive(Size, Unpack, Debug, PartialEq, Eq)]
    #[allow(dead_code)]
    struct HasGenericField<T> {
        wrap: Wrap<T>,
        x: bool,
    }

    assert_eq!(Wrap::unpack::<LittleEndian>(&[3, 0, 0, 0]), Ok(Wrap(3i32)));
    assert_eq!(Wrap::unpack::<BigEndian>(&[0, 4]), Ok(Wrap(4u16)));
    assert_eq!(HasField::unpack::<BigEndian>(&[0, 0, 0, 9]), Ok(HasField { wrap: Wrap(9u32) }));
    assert_eq!(HasGenericField::unpack::<BigEndian>(&[1, 1]), Ok(HasGenericField { wrap: Wrap(true), x: true }));

    let buffer = [9, 8, 3];
    assert_eq!(
        <(i16, u8)>::unpack::<BigEndian>(&buffer),
        Pair::<i16, u8>::unpack::<BigEndian>(&buffer).map(|Pair { left, right }| (left, right))
    );
}

#[test]
fn primitive_enums() {
    #[derive(Size, Unpack, Clone, Copy, PartialEq, Eq, Debug)]
    #[repr(u8)]
    #[allow(dead_code)]
    enum X {
        A = 1,
        B = 4,
        C = 8,
    }

    #[derive(Size, Unpack, Clone, Copy, PartialEq, Eq, Debug)]
    #[repr(i32)]
    #[allow(dead_code)]
    enum Y {
        A = -9,
    }

    assert_eq!(X::unpack::<BigEndian>(&[1]), Ok(X::A));
    assert_eq!(X::unpack::<BigEndian>(&[4]), Ok(X::B));
    assert_eq!(X::unpack::<LittleEndian>(&[8]), Ok(X::C));
    assert_eq!(X::unpack::<LittleEndian>(&[2]), Err(Error::InvalidEnumValueU8(2)));
    assert_eq!(Y::unpack::<LittleEndian>(&[255, 255, 255, 255]), Err(Error::InvalidEnumValueI32(-1)));
    assert_eq!(Y::unpack::<BigEndian>(&[255, 255, 255, 247]), Ok(Y::A));
    assert_eq!(Y::unpack::<LittleEndian>(&[247, 255, 255, 255]), Ok(Y::A));
}

#[test]
fn arrays() {
    assert_eq!(<[u8; 0]>::unpack::<BigEndian>(&[]), Ok([]));
    assert_eq!(<[_; 1]>::unpack::<BigEndian>(&[255]), Ok([-1i8]));
    assert_eq!(<[_; 3]>::unpack::<BigEndian>(&[0, 0, 0, 16, 0, 0, 0, 16, 0, 0, 0, 16]), Ok([16u32; 3]));
    assert_eq!(<[_; 5]>::unpack::<BigEndian>(&[3, 4, 1, 0, 2]), Ok([3u8, 4, 1, 0, 2]));
    assert_eq!(<[_; 3]>::unpack::<LittleEndian>(&[2, 0, 3, 0, 255, 255]), Ok([2i16, 3, -1]));
    assert_eq!(<[_; 3]>::unpack::<LittleEndian>(&[1, 0, 0, 0, 1, 1]), Ok([(true, false), (false, false), (true, true)]));
}

#[test]
fn array_partial_dropping() {
    // This makes sure that the mechanism to drop already-parsed elements
    // works correctly (in the unsafe array unpack implementation).

    static DROP_COUNTER: AtomicUsize = AtomicUsize::new(0);

    // The test may only be invoked once per execution!
    assert_eq!(DROP_COUNTER.load(Ordering::Relaxed), 0);

    #[derive(PartialEq, Eq, Debug)]
    struct Incrementor;

    impl Drop for Incrementor {
        fn drop(&mut self) {
            DROP_COUNTER.fetch_add(1, Ordering::Relaxed);
        }
    }

    impl Size for Incrementor {
        const SIZE: usize = <(bool, u8)>::SIZE;
    }

    impl Unpack for Incrementor {
        fn unpack<B>(buffer: &[u8]) -> Result<Self> where B: byteorder::ByteOrder, Self: Sized {
            let (succeed, i) = <(bool, u8)>::unpack::<B>(buffer)?;
            if succeed {
                Ok(Incrementor)
            } else {
                Err(Error::Custom(i as u32))
            }
        }
    }

    let buffer = [
        // bool, u8
        1, 0u8,
        1, 1,
        1, 2,
        0, 3,
        1, 4,
    ];

    let result = <[Incrementor; 10]>::unpack::<BigEndian>(&buffer);

    assert_eq!(result, Err(Error::Custom(3)));
    assert_eq!(DROP_COUNTER.load(Ordering::Relaxed), 3);
}

#[test]
fn fixed_endianness() {
    assert_eq!(BE::<u16>::unpack::<BigEndian>(&[0, 4]), Ok(BE(4)));
    assert_eq!(BE::<u16>::unpack::<LittleEndian>(&[0, 4]), Ok(BE(4)));
    assert_eq!(LE::<u16>::unpack::<BigEndian>(&[4, 0]), Ok(LE(4)));
    assert_eq!(LE::<u16>::unpack::<LittleEndian>(&[4, 0]), Ok(LE(4)));
    assert_eq!((&[4, 0]).unpack_into::<BigEndian>(), Ok(LE(4u16)));
}

#[test]
fn unpack_safely() {
    let buffer = [2u8, 0];
    assert_eq!(u16::unpack_safely::<LittleEndian>(&buffer), Ok(2));
    assert_eq!(u32::unpack_safely::<LittleEndian>(&buffer), Err(Error::BufferTooSmall { actual: 2, expected: 4 }));
}
