use byteorder::{ByteOrder, BigEndian, LittleEndian};
use lightpack::{Pack, Unpack, Size};

fn roundtrip<B, P>(value: P) -> P where B: ByteOrder, P: Pack + Unpack {
    let mut buffer = vec![0u8; P::SIZE];
    value.pack::<B>(&mut buffer);
    P::unpack::<B>(&buffer)
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


