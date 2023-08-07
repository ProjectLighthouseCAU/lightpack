use byteorder::{ByteOrder, BigEndian, LittleEndian};
use lightpack::{Pack, Size, extra::{BE, LE, PackSize}};

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
fn generic_structs() {
    #[derive(Size, Pack)]
    #[allow(dead_code)]
    struct Wrap<T>(T);

    #[derive(Size, Pack)]
    #[allow(dead_code)]
    struct Pair<L, R> {
        left: L,
        right: R,
    }

    #[derive(Size, Pack)]
    #[allow(dead_code)]
    struct HasField {
        wrap: Wrap<u32>,
    }

    #[derive(Size, Pack)]
    #[allow(dead_code)]
    struct HasGenericField<T> {
        wrap: Wrap<T>,
        x: bool,
    }

    assert_eq!(pack_vec::<LittleEndian, _>(Wrap(3i32)), vec![3, 0, 0, 0]);
    assert_eq!(pack_vec::<BigEndian, _>(Wrap(4u16)), vec![0, 4]);
    assert_eq!(pack_vec::<BigEndian, _>(Pair { left: -4i16, right: 3u8 }), pack_vec::<BigEndian, _>((-4i16, 3u8)));
    assert_eq!(pack_vec::<BigEndian, _>(HasField { wrap: Wrap(4u32) }), vec![0, 0, 0, 4]);
    assert_eq!(pack_vec::<LittleEndian, _>(HasGenericField { wrap: Wrap(false), x: true }), vec![0, 1]);
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

#[test]
fn arrays() {
    assert_eq!(pack_vec::<BigEndian, _>([0u32; 0]), vec![]);
    assert_eq!(pack_vec::<BigEndian, _>([-1i8]), vec![255]);
    assert_eq!(pack_vec::<BigEndian, _>([16u32; 3]), vec![0, 0, 0, 16, 0, 0, 0, 16, 0, 0, 0, 16]);
    assert_eq!(pack_vec::<BigEndian, _>([3u8, 4, 1, 0, 2]), vec![3, 4, 1, 0, 2]);
    assert_eq!(pack_vec::<LittleEndian, _>([2i16, 3, -1]), vec![2, 0, 3, 0, 255, 255]);
    assert_eq!(pack_vec::<LittleEndian, _>([(true, false), (false, false), (true, true)]), vec![1, 0, 0, 0, 1, 1]);
}

#[test]
fn fixed_endianness() {
    assert_eq!(pack_vec::<BigEndian, _>(BE(45u16)), pack_vec::<BigEndian, u16>(45));
    assert_eq!(pack_vec::<LittleEndian, _>(BE(45u16)), pack_vec::<BigEndian, u16>(45));
    assert_eq!(pack_vec::<BigEndian, _>(LE(45u16)), pack_vec::<LittleEndian, u16>(45));
    assert_eq!(pack_vec::<LittleEndian, _>(LE(45u16)), pack_vec::<LittleEndian, u16>(45));
}

#[test]
fn mixed_endianness() {
    #[derive(Size, Pack)]
    struct Mixed {
        le: LE<u16>,
        be: BE<u16>,
    }

    assert_eq!(pack_vec::<BigEndian, _>(Mixed { le: LE(256), be: BE(256) }), vec![0, 1, 1, 0]);
    assert_eq!(pack_vec::<LittleEndian, _>(Mixed { le: LE(256), be: BE(256) }), vec![0, 1, 1, 0]);
}

#[test]
fn test_pack_size() {
    let mut buffer = [0u8; 4];
    assert_eq!(3u16.pack_size::<BigEndian>(&mut buffer), u16::SIZE);
    assert_eq!(2u32.pack_size::<BigEndian>(&mut buffer), u32::SIZE);
}
