# Lightpack

[![Build](https://github.com/ProjectLighthouseCAU/lightpack/actions/workflows/build.yml/badge.svg)](https://github.com/ProjectLighthouseCAU/lightpack/actions/workflows/build.yml)

A lighweight, no-std binary serialization library.

## Example

Lightpack provides derivable traits for encoding (`Pack`) and decoding (`Unpack`), as well as for determining a type's encoded size (`Size`):

```rust
#[derive(Size, Pack, Unpack, Debug)]
struct Point {
    x: i16,
    y: i16,
}
```

To encode, call `pack` with an endianness (e.g. `lightpack::byteorder::BigEndian`) on a `&mut [u8]` slice:

```rust
let mut buffer = [0u8; Point::SIZE];
Point { x: 3, y: 4 }.pack::<BigEndian>(&mut buffer);
// => buffer == [0, 3, 0, 4]
```

To decode, call `unpack` on a `&[u8]` slice:

```rust
Point::unpack::<BigEndian>(&[0, 3, 0, 4]).unwrap();
// => Point { x: 3, y: 4 }
```

For a complete example, check out [`examples/point.rs`](examples/point.rs).
