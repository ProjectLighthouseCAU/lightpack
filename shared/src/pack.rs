pub trait Pack {
    fn pack<B>(&self, buffer: &mut [u8]);
}
