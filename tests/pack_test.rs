use byteorder::{ByteOrder, BigEndian, LittleEndian};
use lightpack_shared::Pack;

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
