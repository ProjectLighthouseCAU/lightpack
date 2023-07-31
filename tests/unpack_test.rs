use byteorder::{BigEndian, LittleEndian};
use lightpack::{Unpack, Size};

#[test]
fn unsigned_ints() {
    assert_eq!(u8::unpack::<BigEndian>(&[4]), 4);
    assert_eq!(u16::unpack::<BigEndian>(&[1, 0]), 256);
    assert_eq!(u16::unpack::<LittleEndian>(&[0, 1]), 256);
}

#[test]
fn signed_ints() {
    assert_eq!(i8::unpack::<BigEndian>(&[255]), -1);
}
