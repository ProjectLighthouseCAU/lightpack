pub trait Size {
    /// The type's encoded size in bytes.
    const SIZE: usize;
}

impl Size for u8 {
    const SIZE: usize = 1;
}

impl Size for u16 {
    const SIZE: usize = 2;
}

impl Size for u32 {
    const SIZE: usize = 4;
}

impl Size for u64 {
    const SIZE: usize = 8;
}

impl Size for i8 {
    const SIZE: usize = 1;
}

impl Size for i16 {
    const SIZE: usize = 2;
}

impl Size for i32 {
    const SIZE: usize = 4;
}

impl Size for i64 {
    const SIZE: usize = 8;
}
