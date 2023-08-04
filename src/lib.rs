//! Lightpack is a lightweight, no-std binary serialization library that uses a
//! simple, untagged encoding format. This makes it especially suitable for
//! embedded use-cases.
//! 
//! The crate provides three core traits: [`Pack`] for encoding, [`Unpack`] for
//! decoding and [`Size`] providing the encoded size in bytes as associated const.
//! The first two traits are roughly analogouos to serde's `Serialize` and
//! `Deserialize`.
//! 
//! These traits can be derived to make your own structures encodable and decodable:
//! 
//! ```ignore
//! #[derive(Size, Pack, Unpack)]
//! struct Point {
//!     x: i16,
//!     y: i16,
//! }
//! ```
//! 
//! To encode, call `pack` with an endianness (e.g. `lightpack::byteorder::BigEndian`) on a `&mut [u8]` slice:
//! 
//! ```ignore
//! let mut buffer = [0u8; Point::SIZE];
//! Point { x: 3, y: 4 }.pack::<BigEndian>(&mut buffer);
//! // => buffer == [0, 3, 0, 4]
//! ```
//! 
//! To decode, call `unpack` on a `&[u8]` slice:
//! 
//! ```ignore
//! Point::unpack::<BigEndian>(&[0, 3, 0, 4]).unwrap()
//! // => Point { x: 3, y: 4 }
//! ```

#![no_std]

pub mod extra;
pub mod pack;
pub mod size;
pub mod unpack;

pub use pack::Pack;
pub use size::Size;
pub use unpack::Unpack;

pub use byteorder;
pub use lightpack_derive::*;
