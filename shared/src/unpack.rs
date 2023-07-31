pub trait Unpack {
    /// Decodes the type from a binary representation.
    fn unpack<B>(buffer: &[u8]) -> Self;
}
