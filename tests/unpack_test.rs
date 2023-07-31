use byteorder::{BigEndian, LittleEndian};
use lightpack::{Unpack, Size};

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
fn derived_structs() {
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


