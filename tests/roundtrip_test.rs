use byteorder::{ByteOrder, BigEndian, LittleEndian};
use lightpack::{Pack, Unpack, Size};

fn roundtrip<B, P>(value: P) -> P where B: ByteOrder, P: Pack + Unpack {
    let mut buffer = vec![0u8; P::SIZE];
    value.pack::<B>(&mut buffer);
    P::unpack::<B>(&buffer).unwrap()
}

macro_rules! assert_roundtrips {
    ($x:expr) => {
        {
            assert_eq!(roundtrip::<BigEndian, _>($x), $x);
            assert_eq!(roundtrip::<LittleEndian, _>($x), $x);
        }
    };
}

#[test]
fn unsigned_ints() {
    assert_roundtrips!(0u8);
    assert_roundtrips!(2u8);
    assert_roundtrips!(24u16);
    assert_roundtrips!(1024u16);
    assert_roundtrips!(3000u32);
    assert_roundtrips!(u32::MIN);
    assert_roundtrips!(u32::MAX);
}

#[test]
fn signed_ints() {
    assert_roundtrips!(0i8);
    assert_roundtrips!(2i8);
    assert_roundtrips!(-2i8);
    assert_roundtrips!(24i16);
    assert_roundtrips!(-1024i16);
    assert_roundtrips!(3000i32);
    assert_roundtrips!(i32::MIN);
    assert_roundtrips!(i32::MAX);
}

#[test]
fn basic_structs() {
    #[derive(Size, Pack, Unpack, Clone, Copy, Debug, PartialEq, Eq)]
    #[allow(dead_code)]
    struct X {
        x: u8,
        y: u16,
    }

    #[derive(Size, Pack, Unpack, Clone, Copy, Debug, PartialEq, Eq)]
    #[allow(dead_code)]
    struct Y {
        x0: X,
        x1: X,
        x2: bool,
    }

    #[derive(Size, Pack, Unpack, Clone, Copy, Debug, PartialEq, Eq)]
    struct Tuple(X, Y);

    #[derive(Size, Pack, Unpack, Clone, Copy, Debug, PartialEq, Eq)]
    struct Unit;

    let x0 = X { x: 3, y: 4 };
    let x1 = X { x: 2, y: 8 };
    let y = Y { x0, x1, x2: true };

    assert_roundtrips!(x0);
    assert_roundtrips!(x1);
    assert_roundtrips!(y);
    assert_roundtrips!(Tuple(x0, y));
    assert_roundtrips!(Unit);
}

#[test]
fn generic_structs() {
    #[derive(Size, Pack, Unpack, Debug, PartialEq, Eq)]
    #[allow(dead_code)]
    struct Wrap<T>(T);

    #[derive(Size, Pack, Unpack, Debug, PartialEq, Eq)]
    #[allow(dead_code)]
    struct Pair<L, R> {
        left: L,
        right: R,
    }

    assert_roundtrips!(Wrap(-1i8));
    assert_roundtrips!(Wrap(13u16));
    assert_roundtrips!(Pair { left: Wrap(23i64), right: Wrap(274u32) });
}

#[test]
fn primitive_enums() {
    #[derive(Size, Pack, Unpack, Clone, Copy, PartialEq, Eq, Debug)]
    #[repr(u8)]
    #[allow(dead_code)]
    enum X {
        A = 1,
        B = 4,
        C = 8,
    }

    #[derive(Size, Pack, Unpack, Clone, Copy, PartialEq, Eq, Debug)]
    #[repr(i32)]
    #[allow(dead_code)]
    enum Y {
        A = -9,
        B = 32,
        C = 1,
    }

    assert_roundtrips!(X::A);
    assert_roundtrips!(X::B);
    assert_roundtrips!(X::C);
    assert_roundtrips!(Y::A);
    assert_roundtrips!(Y::B);
    assert_roundtrips!(Y::C);
}

#[test]
fn arrays() {
    assert_roundtrips!([0; 0]);
    assert_roundtrips!([3u16, 4, 23, 128, 9]);
    assert_roundtrips!([true, false]);
    assert_roundtrips!([(false, true), (true, true)]);
    assert_roundtrips!(['H', 'e', 'l', 'l', 'o']);
}
