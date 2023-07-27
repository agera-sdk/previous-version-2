//! Work with binary data.
//!
//! This module provides an efficient byte buffer structure
//! ([`Bytes`](struct.Bytes.html)) and traits for working with buffer
//! implementations ([`Buffer`], [`BufferMut`]).
//!
//! # `Bytes`
//!
//! `Bytes` is an efficient container for storing and operating on contiguous
//! slices of memory. It is intended for use primarily in networking code, but
//! could have applications elsewhere as well.
//!
//! `Bytes` values facilitate zero-copy network programming by allowing multiple
//! `Bytes` objects to point to the same underlying memory. This is managed by
//! using a reference count to track when the memory is no longer needed and can
//! be freed.
//!
//! A `Bytes` handle can be created directly from an existing byte store (such as `&[u8]`
//! or `Vec<u8>`), but usually a `BytesMut` is used first and written to. For
//! example:
//!
//! ```rust
//! use rialight_util::bytes::{BytesMut, BufferMut};
//!
//! let mut buffer = BytesMut::with_capacity(1024);
//! buffer.put(&b"hello world"[..]);
//! buffer.put_u16(1234);
//!
//! let a = buffer.split();
//! assert_eq!(a, b"hello world\x04\xD2"[..]);
//!
//! buffer.put(&b"goodbye world"[..]);
//!
//! let b = buffer.split();
//! assert_eq!(b, b"goodbye world"[..]);
//!
//! assert_eq!(buffer.capacity(), 998);
//! ```
//!
//! In the above example, only a single buffer of 1024 is allocated. The handles
//! `a` and `b` will share the underlying buffer and maintain indices tracking
//! the view into the buffer represented by the handle.
//!
//! See the [struct docs] for more details.
//!
//! [struct docs]: struct.Bytes.html
//!
//! # `Buffer`, `BufferMut`
//!
//! These two traits provide read and write access to buffers. The underlying
//! storage may or may not be in contiguous memory. For example, `Bytes` is a
//! buffer that guarantees contiguous memory, but a [rope] stores the bytes in
//! disjoint chunks. `Buffer` and `BufferMut` maintain cursors tracking the current
//! position in the underlying byte storage. When bytes are read or written, the
//! cursor is advanced.
//!
//! [rope]: https://en.wikipedia.org/wiki/Rope_(data_structure)
//!
//! ## Relation with `Read` and `Write`
//!
//! At first glance, it may seem that `Buffer` and `BufferMut` overlap in
//! functionality with `std::io::Read` and `std::io::Write`. However, they
//! serve different purposes. A buffer is the value that is provided as an
//! argument to `Read::read` and `Write::write`. `Read` and `Write` may then
//! perform a syscall, which has the potential of failing. Operations on `Buffer`
//! and `BufferMut` are infallible.

pub use bytes::{Bytes, BytesMut, Buf as Buffer, BufMut as BufferMut};

/// Utilities for working with buffers.
///
/// A buffer is any structure that contains a sequence of bytes. The bytes may
/// or may not be stored in contiguous memory. This module contains traits used
/// to abstract over buffers as well as utilities for working with buffer types.
///
/// # `Buffer`, `BufferMut`
///
/// These are the two foundational traits for abstractly working with buffers.
/// They can be thought as iterators for byte structures. They offer additional
/// performance over `Iterator` by providing an API optimized for byte slices.
///
/// See [`Buffer`] and [`BufferMut`] for more details.
///
/// [rope]: https://en.wikipedia.org/wiki/Rope_(data_structure)
pub mod buffer {
    pub use bytes::buf::{
        Chain,
        IntoIter,
        Limit,
        Reader,
        Take,
        UninitSlice,
        Writer,
        Buf as Buffer,
        BufMut as BufferMut,
    };
}