use lightpack::Size;

#[test]
fn unsigned_ints() {
    assert_eq!(u8::SIZE, 1);
    assert_eq!(u16::SIZE, 2);
    assert_eq!(u32::SIZE, 4);
    assert_eq!(u64::SIZE, 8);
}

#[test]
fn signed_ints() {
    assert_eq!(i8::SIZE, 1);
    assert_eq!(i16::SIZE, 2);
    assert_eq!(i32::SIZE, 4);
    assert_eq!(i64::SIZE, 8);
}

#[test]
fn basic_structs() {
    #[derive(Size)]
    #[allow(dead_code)]
    struct X {
        x: u8,
        y: u16,
    }

    #[derive(Size)]
    #[allow(dead_code)]
    struct Y {
        x0: X,
        x1: X,
        x2: bool,
    }

    #[derive(Size)]
    struct Tuple(X, Y);

    #[derive(Size)]
    struct Unit;

    assert_eq!(X::SIZE, 3);
    assert_eq!(Y::SIZE, 7);
    assert_eq!(Tuple::SIZE, 10);
    assert_eq!(Unit::SIZE, 0);
}

#[test]
fn generic_structs() {
    #[derive(Size)]
    #[allow(dead_code)]
    struct Wrap<T>(T);

    #[derive(Size)]
    #[allow(dead_code)]
    struct Pair<L, R> {
        left: L,
        right: R,
    }

    assert_eq!(Wrap::<i32>::SIZE, i32::SIZE);
    assert_eq!(Wrap::<u8>::SIZE, u8::SIZE);
    assert_eq!(Pair::<i16, u8>::SIZE, i16::SIZE + u8::SIZE);
}

#[test]
fn primitive_enums() {
    #[derive(Size)]
    #[repr(u8)]
    #[allow(dead_code)]
    enum X {
        A = 1,
        B = 4,
        C = 8,
    }

    #[derive(Size)]
    #[repr(i32)]
    #[allow(dead_code)]
    enum Y {
        A = -9,
    }

    assert_eq!(X::SIZE, 1);
    assert_eq!(Y::SIZE, 4);
}
