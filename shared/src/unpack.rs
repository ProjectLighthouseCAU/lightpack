pub trait Unpack {
    fn unpack<B>(buffer: &[u8]) -> Self;
}
