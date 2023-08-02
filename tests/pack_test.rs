use byteorder::{ByteOrder, BigEndian, LittleEndian};
use lightpack::{Pack, Size};

fn pack_vec<B, P>(value: P) -> Vec<u8> where B: ByteOrder, P: Pack {
    let mut buffer = vec![0u8; P::SIZE];
    value.pack::<B>(&mut buffer);
    buffer
}

#[test]
fn unsigned_ints() {
    assert_eq!(pack_vec::<BigEndian, u8>(0), vec![0]);
    assert_eq!(pack_vec::<BigEndian, u16>(4), vec![0, 4]);
    assert_eq!(pack_vec::<LittleEndian, u16>(4), vec![4, 0]);
    assert_eq!(pack_vec::<BigEndian, u16>(258), vec![1, 2]);
    assert_eq!(pack_vec::<LittleEndian, u16>(258), vec![2, 1]);
    assert_eq!(pack_vec::<LittleEndian, u32>(1048), vec![24, 0b100, 0, 0]);
    assert_eq!(pack_vec::<BigEndian, u32>(1048), vec![0, 0, 0b100, 24]);
}

#[test]
fn signed_ints() {
    assert_eq!(pack_vec::<BigEndian, i8>(0), vec![0]);
    assert_eq!(pack_vec::<BigEndian, i8>(-1), vec![255]);
    assert_eq!(pack_vec::<LittleEndian, i8>(-2), vec![254]);
    assert_eq!(pack_vec::<BigEndian, i16>(256), vec![1, 0]);
    assert_eq!(pack_vec::<LittleEndian, i16>(256), vec![0, 1]);
}

#[test]
fn basic_structs() {
    #[derive(Size, Pack, Clone, Copy)]
    #[allow(dead_code)]
    struct X {
        x: u8,
        y: u16,
    }

    #[derive(Size, Pack, Clone, Copy)]
    #[allow(dead_code)]
    struct Y {
        x0: X,
        x1: X,
        x2: bool,
    }

    #[derive(Size, Pack, Clone, Copy)]
    struct Tuple(X, Y);

    #[derive(Size, Pack, Clone, Copy)]
    struct Unit;

    let x0 = X { x: 3, y: 4 };
    let x1 = X { x: 2, y: 8 };
    let y = Y { x0, x1, x2: true };

    assert_eq!(pack_vec::<BigEndian, _>(x0), vec![3, 0, 4]);
    assert_eq!(pack_vec::<LittleEndian, _>(x1), vec![2, 8, 0]);
    assert_eq!(pack_vec::<BigEndian, _>(y), vec![3, 0, 4, 2, 0, 8, 1]);
    assert_eq!(pack_vec::<BigEndian, _>(Tuple(x0, y)), vec![3, 0, 4, 3, 0, 4, 2, 0, 8, 1]);
    assert_eq!(pack_vec::<BigEndian, _>(Unit), vec![]);
}

#[test]
fn primitive_enums() {
    #[derive(Size, Pack, Clone, Copy)]
    #[repr(u8)]
    #[allow(dead_code)]
    enum X {
        A = 1,
        B = 4,
        C = 8,
    }

    #[derive(Size, Pack, Clone, Copy)]
    #[repr(i32)]
    #[allow(dead_code)]
    enum Y {
        A = -9,
    }

    assert_eq!(pack_vec::<BigEndian, _>(X::A), vec![1]);
    assert_eq!(pack_vec::<BigEndian, _>(X::B), vec![4]);
    assert_eq!(pack_vec::<BigEndian, _>(X::C), vec![8]);
}
