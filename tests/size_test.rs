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
fn derived_struct_size() {
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
