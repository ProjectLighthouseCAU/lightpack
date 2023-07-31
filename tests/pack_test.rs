use byteorder::{ByteOrder, BigEndian, LittleEndian};
use lightpack::{Pack, Size};

// TODO: This will likely be much easier once generic_const_exprs
//       stabilizes since we can just use P::SIZE instead of N.
//       We might even want to make this available to library consumers
//       by adding a `PackExt` trait with a `pack_buf` method that returns
//       `[u8; Self::SIZE]`.

fn pack_buf<B, P, const N: usize>(value: P) -> [u8; N] where B: ByteOrder, P: Pack {
    let mut buffer = [0u8; N];
    value.pack::<B>(&mut buffer);
    buffer
}

#[test]
fn unsigned_ints() {
    assert_eq!(pack_buf::<BigEndian, u8, 1>(0), [0]);
    assert_eq!(pack_buf::<BigEndian, u8, 2>(4), [4, 0]);
    assert_eq!(pack_buf::<LittleEndian, u8, 2>(4), [4, 0]);
    assert_eq!(pack_buf::<BigEndian, u16, 2>(258), [1, 2]);
    assert_eq!(pack_buf::<LittleEndian, u16, 2>(258), [2, 1]);
    assert_eq!(pack_buf::<LittleEndian, u32, 4>(1048), [24, 0b100, 0, 0]);
    assert_eq!(pack_buf::<BigEndian, u32, 4>(1048), [0, 0, 0b100, 24]);
}

#[test]
fn signed_ints() {
    assert_eq!(pack_buf::<BigEndian, i8, 1>(0), [0]);
    assert_eq!(pack_buf::<BigEndian, i8, 2>(-1), [255, 0]);
    assert_eq!(pack_buf::<LittleEndian, i8, 2>(-2), [254, 0]);
    assert_eq!(pack_buf::<BigEndian, i16, 2>(256), [1, 0]);
    assert_eq!(pack_buf::<LittleEndian, i16, 2>(256), [0, 1]);
}

#[test]
fn derived_structs() {
    #[derive(Size, Pack)]
    #[allow(dead_code)]
    struct X {
        x: u8,
        y: u16,
    }

    #[derive(Size, Pack)]
    #[allow(dead_code)]
    struct Y {
        x0: X,
        x1: X,
        x2: bool,
    }

    #[derive(Size, Pack)]
    struct Tuple(X, Y);

    #[derive(Size, Pack)]
    struct Unit;

    assert_eq!(pack_buf::<BigEndian, _, 3>(X { x: 3, y: 4 }), [3, 0, 4]);
    assert_eq!(pack_buf::<LittleEndian, _, 3>(X { x: 3, y: 4 }), [3, 4, 0]);
}

