use byteorder::{BigEndian, LittleEndian};
use lightpack::{Unpack, Size};
use lightpack_shared::unpack::Error;

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
