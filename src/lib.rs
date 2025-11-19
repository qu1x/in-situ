//! In Situ Endian-independent Bytes Access
//!
//! # Feature Gates
//!
//!   * `bytes`: For abstracting `Bytes` and `BytesMut` as well.

#[cfg(feature = "bstr")]
pub use bstr;
#[cfg(feature = "bstr")]
use bstr::BStr;
pub use byteorder;
#[cfg(feature = "bytes")]
pub use bytes;

use byteorder::{BE, ByteOrder, LE, NativeEndian};
use std::{fmt::Debug, hash::Hash, mem, str::Utf8Error};

/// Size of `u8`.
pub const U8: usize = 1;
/// Size of `u16`.
pub const U16: usize = 2;
/// Size of `u24`.
pub const U24: usize = 3;
/// Size of `u32`.
pub const U32: usize = 4;
/// Size of `u64`.
pub const U64: usize = 8;
/// Size of `u128`.
pub const U128: usize = 16;
/// Size of `i8`.
pub const I8: usize = 1;
/// Size of `i16`.
pub const I16: usize = 2;
/// Size of `i24`.
pub const I24: usize = 3;
/// Size of `i32`.
pub const I32: usize = 4;
/// Size of `i64`.
pub const I64: usize = 8;
/// Size of `i128`.
pub const I128: usize = 16;
/// Size of `f32`.
pub const F32: usize = 4;
/// Size of `f64`.
pub const F64: usize = 8;

/// Calculates padding of `align`ed `offset` in bytes.
///
/// Leverages two's complement shortcuts instead of branching and modulo operations.
#[must_use]
#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
pub fn padding(offset: usize, align: usize) -> usize {
    let padding = -(offset as isize) as usize & (align - 1);
    debug_assert_eq!(padding, (align - offset % align) % align);
    padding
}

/// Calculates `align`ed `offset` in bytes.
///
/// Leverages two's complement shortcuts instead of branching and modulo operations.
#[must_use]
#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
pub fn aligned(offset: usize, align: usize) -> usize {
    let aligned = (offset + align - 1) & -(align as isize) as usize;
    debug_assert_eq!(aligned, offset + padding(offset, align));
    aligned
}

/// Provides endian-independent immutable bytes access.
///
/// Requires methods to be implemented detecting or hardcoding the word size and endianness. This
/// trait requires the `AsRef<[u8]>` trait to access slices of generic types. It is not implemented
/// for the `Raw` trait but instead for its wrapper types since each wrapper might implement the
/// endianness detection differently.
pub trait InSitu: AsRef<[u8]> {
    /// The word size of the slice, not to be confused with the various word sizes of how to access
    /// the slice.
    fn swap_size(&self) -> usize;
    /// Whether the underlying bytes are in big-endian (be) or little-endian (le) byte order.
    fn is_be(&self) -> bool;
    /// Inversion of `is_be()`.
    fn is_le(&self) -> bool {
        !self.is_be()
    }
    /// Tests if the underlying byte order has the machine's native endianness.
    fn is_native(&self) -> bool {
        self.is_be() == (NativeEndian::read_u16(&[0, 1]) == 1)
    }
    /// Convert `is_be()`/`is_le()` into `Order`.
    fn order(&self) -> Order {
        if self.is_be() { Order::BE } else { Order::LE }
    }
    /// If `is_le()`, translates big-endian `offset` of word with `word_size` in slice of
    /// `swap_size()` into little-endian via bitwise instead of branching and modulo operations,
    /// otherwise passes through `offset`.
    fn at(&self, offset: usize, word_size: usize) -> usize {
        if self.is_be() || self.swap_size() < word_size {
            offset
        } else {
            offset ^ (self.swap_size() - word_size)
        }
    }
    /// Gets `&str` if UTF-8 in slice of `swap_size()` at big-endian `offset` endian-independently.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the slice is not UTF-8 with a description as to why the provided slice is
    /// not UTF-8.
    fn utf8(&self, offset: usize, length: usize) -> Result<&str, Utf8Error> {
        std::str::from_utf8(&self.as_ref()[offset..][..length])
    }
    /// Gets `BStr` in slice of `swap_size()` at big-endian `offset` endian-independently.
    #[cfg(feature = "bstr")]
    fn bstr(&self, offset: usize, length: usize) -> &BStr {
        BStr::new(&self.as_ref()[offset..][..length])
    }
    /// Gets `bool` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn bool(&self, offset: usize) -> bool {
        self.u8(offset) != 0
    }
    /// Gets `u8` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn u8(&self, offset: usize) -> u8 {
        let offset = self.at(offset, U8);
        self.as_ref()[offset]
    }
    /// Gets `u16` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn u16(&self, offset: usize) -> u16 {
        let offset = self.at(offset, U16);
        if self.is_be() {
            BE::read_u16(&self.as_ref()[offset..])
        } else {
            LE::read_u16(&self.as_ref()[offset..])
        }
    }
    /// Gets `u24` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn u24(&self, offset: usize) -> u32 {
        let offset = self.at(offset, U24);
        if self.is_be() {
            BE::read_u24(&self.as_ref()[offset..])
        } else {
            LE::read_u24(&self.as_ref()[offset..])
        }
    }
    /// Gets `u32` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn u32(&self, offset: usize) -> u32 {
        let offset = self.at(offset, U32);
        if self.is_be() {
            BE::read_u32(&self.as_ref()[offset..])
        } else {
            LE::read_u32(&self.as_ref()[offset..])
        }
    }
    /// Gets `u64` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn u64(&self, offset: usize) -> u64 {
        let offset = self.at(offset, U64);
        if self.is_be() {
            BE::read_u64(&self.as_ref()[offset..])
        } else {
            LE::read_u64(&self.as_ref()[offset..])
        }
    }
    /// Gets `u128` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn u128(&self, offset: usize) -> u128 {
        let offset = self.at(offset, U128);
        if self.is_be() {
            BE::read_u128(&self.as_ref()[offset..])
        } else {
            LE::read_u128(&self.as_ref()[offset..])
        }
    }
    /// Gets unsigned integer of `word_size <= 8` in slice of `swap_size()` at big-endian `offset`
    /// endian-independently.
    fn uint(&self, offset: usize, word_size: usize) -> u64 {
        let offset = self.at(offset, word_size);
        if self.is_be() {
            BE::read_uint(&self.as_ref()[offset..], word_size)
        } else {
            LE::read_uint(&self.as_ref()[offset..], word_size)
        }
    }
    /// Gets unsigned integer of `word_size <= 16` in slice of `swap_size()` at big-endian `offset`
    /// endian-independently.
    fn uint128(&self, offset: usize, word_size: usize) -> u128 {
        let offset = self.at(offset, word_size);
        if self.is_be() {
            BE::read_uint128(&self.as_ref()[offset..], word_size)
        } else {
            LE::read_uint128(&self.as_ref()[offset..], word_size)
        }
    }
    /// Gets `i8` in slice of `swap_size()` at big-endian `offset` endian-independently.
    #[allow(clippy::cast_possible_wrap)]
    fn i8(&self, offset: usize) -> i8 {
        let offset = self.at(offset, I8);
        self.as_ref()[offset] as i8
    }
    /// Gets `i16` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn i16(&self, offset: usize) -> i16 {
        let offset = self.at(offset, I16);
        if self.is_be() {
            BE::read_i16(&self.as_ref()[offset..])
        } else {
            LE::read_i16(&self.as_ref()[offset..])
        }
    }
    /// Gets `i24` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn i24(&self, offset: usize) -> i32 {
        let offset = self.at(offset, I24);
        if self.is_be() {
            BE::read_i24(&self.as_ref()[offset..])
        } else {
            LE::read_i24(&self.as_ref()[offset..])
        }
    }
    /// Gets `i32` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn i32(&self, offset: usize) -> i32 {
        let offset = self.at(offset, I32);
        if self.is_be() {
            BE::read_i32(&self.as_ref()[offset..])
        } else {
            LE::read_i32(&self.as_ref()[offset..])
        }
    }
    /// Gets `i64` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn i64(&self, offset: usize) -> i64 {
        let offset = self.at(offset, I64);
        if self.is_be() {
            BE::read_i64(&self.as_ref()[offset..])
        } else {
            LE::read_i64(&self.as_ref()[offset..])
        }
    }
    /// Gets `u128` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn i128(&self, offset: usize) -> i128 {
        let offset = self.at(offset, I128);
        if self.is_be() {
            BE::read_i128(&self.as_ref()[offset..])
        } else {
            LE::read_i128(&self.as_ref()[offset..])
        }
    }
    /// Gets signed integer of `word_size <= 8` in slice of `swap_size()` at big-endian `offset`
    /// endian-independently.
    fn int(&self, offset: usize, word_size: usize) -> i64 {
        let offset = self.at(offset, word_size);
        if self.is_be() {
            BE::read_int(&self.as_ref()[offset..], word_size)
        } else {
            LE::read_int(&self.as_ref()[offset..], word_size)
        }
    }
    /// Gets signed integer of `word_size <= 16` in slice of `swap_size()` at big-endian `offset`
    /// endian-independently.
    fn int128(&self, offset: usize, word_size: usize) -> i128 {
        let offset = self.at(offset, word_size);
        if self.is_be() {
            BE::read_int128(&self.as_ref()[offset..], word_size)
        } else {
            LE::read_int128(&self.as_ref()[offset..], word_size)
        }
    }
    /// Gets `f32` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn f32(&self, offset: usize) -> f32 {
        let offset = self.at(offset, F32);
        if self.is_be() {
            BE::read_f32(&self.as_ref()[offset..])
        } else {
            LE::read_f32(&self.as_ref()[offset..])
        }
    }
    /// Gets `f64` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn f64(&self, offset: usize) -> f64 {
        let offset = self.at(offset, F64);
        if self.is_be() {
            BE::read_f64(&self.as_ref()[offset..])
        } else {
            LE::read_f64(&self.as_ref()[offset..])
        }
    }
}

/// Provides endian-independent mutable bytes access.
///
/// Requires `InSitu` trait to know about endianness. This trait is auto-implemented for all
/// `InSitu + AsMut<[u8]>` implementors.
pub trait InSituMut: InSitu + AsMut<[u8]> {
    /// Sets `u8` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn set_u8(&mut self, offset: usize, value: u8) {
        let at = self.at(offset, U8);
        self.as_mut()[at] = value;
    }
    /// Sets `u16` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn set_u16(&mut self, offset: usize, value: u16) {
        let offset = self.at(offset, U16);
        if self.is_be() {
            BE::write_u16(&mut self.as_mut()[offset..], value);
        } else {
            LE::write_u16(&mut self.as_mut()[offset..], value);
        }
    }
    /// Sets `u24` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn set_u24(&mut self, offset: usize, value: u32) {
        let offset = self.at(offset, U24);
        if self.is_be() {
            BE::write_u24(&mut self.as_mut()[offset..], value);
        } else {
            LE::write_u24(&mut self.as_mut()[offset..], value);
        }
    }
    /// Sets `u32` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn set_u32(&mut self, offset: usize, value: u32) {
        let offset = self.at(offset, U32);
        if self.is_be() {
            BE::write_u32(&mut self.as_mut()[offset..], value);
        } else {
            LE::write_u32(&mut self.as_mut()[offset..], value);
        }
    }
    /// Sets `u64` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn set_u64(&mut self, offset: usize, value: u64) {
        let offset = self.at(offset, U64);
        if self.is_be() {
            BE::write_u64(&mut self.as_mut()[offset..], value);
        } else {
            LE::write_u64(&mut self.as_mut()[offset..], value);
        }
    }
    /// Sets `u128` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn set_u128(&mut self, offset: usize, value: u128) {
        let offset = self.at(offset, U128);
        if self.is_be() {
            BE::write_u128(&mut self.as_mut()[offset..], value);
        } else {
            LE::write_u128(&mut self.as_mut()[offset..], value);
        }
    }
    /// Sets unsigned integer of `word_size <= 8` in slice of `swap_size()` at big-endian `offset`
    /// endian-independently.
    fn set_uint(&mut self, offset: usize, value: u64, word_size: usize) {
        let offset = self.at(offset, word_size);
        if self.is_be() {
            BE::write_uint(&mut self.as_mut()[offset..], value, word_size);
        } else {
            LE::write_uint(&mut self.as_mut()[offset..], value, word_size);
        }
    }
    /// Sets unsigned integer of `word_size <= 16` in slice of `swap_size()` at big-endian `offset`
    /// endian-independently.
    fn set_uint128(&mut self, offset: usize, value: u128, word_size: usize) {
        let offset = self.at(offset, word_size);
        if self.is_be() {
            BE::write_uint128(&mut self.as_mut()[offset..], value, word_size);
        } else {
            LE::write_uint128(&mut self.as_mut()[offset..], value, word_size);
        }
    }
    /// Sets `i8` in slice of `swap_size()` at big-endian `offset` endian-independently.
    #[allow(clippy::cast_sign_loss)]
    fn set_i8(&mut self, offset: usize, value: i8) {
        let at = self.at(offset, I8);
        self.as_mut()[at] = value as u8;
    }
    /// Sets `i16` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn set_i16(&mut self, offset: usize, value: i16) {
        let offset = self.at(offset, I16);
        if self.is_be() {
            BE::write_i16(&mut self.as_mut()[offset..], value);
        } else {
            LE::write_i16(&mut self.as_mut()[offset..], value);
        }
    }
    /// Sets `i24` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn set_i24(&mut self, offset: usize, value: i32) {
        let offset = self.at(offset, I24);
        if self.is_be() {
            BE::write_i24(&mut self.as_mut()[offset..], value);
        } else {
            LE::write_i24(&mut self.as_mut()[offset..], value);
        }
    }
    /// Sets `i32` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn set_i32(&mut self, offset: usize, value: i32) {
        let offset = self.at(offset, I32);
        if self.is_be() {
            BE::write_i32(&mut self.as_mut()[offset..], value);
        } else {
            LE::write_i32(&mut self.as_mut()[offset..], value);
        }
    }
    /// Sets `i64` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn set_i64(&mut self, offset: usize, value: i64) {
        let offset = self.at(offset, I64);
        if self.is_be() {
            BE::write_i64(&mut self.as_mut()[offset..], value);
        } else {
            LE::write_i64(&mut self.as_mut()[offset..], value);
        }
    }
    /// Sets `i128` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn set_i128(&mut self, offset: usize, value: i128) {
        let offset = self.at(offset, I128);
        if self.is_be() {
            BE::write_i128(&mut self.as_mut()[offset..], value);
        } else {
            LE::write_i128(&mut self.as_mut()[offset..], value);
        }
    }
    /// Sets signed integer of `word_size <= 8` in slice of `swap_size()` at big-endian `offset`
    /// endian-independently.
    fn set_int(&mut self, offset: usize, value: i64, word_size: usize) {
        let offset = self.at(offset, word_size);
        if self.is_be() {
            BE::write_int(&mut self.as_mut()[offset..], value, word_size);
        } else {
            LE::write_int(&mut self.as_mut()[offset..], value, word_size);
        }
    }
    /// Sets signed integer of `word_size <= 16` in slice of `swap_size()` at big-endian `offset`
    /// endian-independently.
    fn set_int128(&mut self, offset: usize, value: i128, word_size: usize) {
        let offset = self.at(offset, word_size);
        if self.is_be() {
            BE::write_int128(&mut self.as_mut()[offset..], value, word_size);
        } else {
            LE::write_int128(&mut self.as_mut()[offset..], value, word_size);
        }
    }
    /// Sets `f32` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn set_f32(&mut self, offset: usize, value: f32) {
        let offset = self.at(offset, F32);
        if self.is_be() {
            BE::write_f32(&mut self.as_mut()[offset..], value);
        } else {
            LE::write_f32(&mut self.as_mut()[offset..], value);
        }
    }
    /// Sets `f64` in slice of `swap_size()` at big-endian `offset` endian-independently.
    fn set_f64(&mut self, offset: usize, value: f64) {
        let offset = self.at(offset, F64);
        if self.is_be() {
            BE::write_f64(&mut self.as_mut()[offset..], value);
        } else {
            LE::write_f64(&mut self.as_mut()[offset..], value);
        }
    }
}

// Auto-implement `InSituMut` for `InSitu + AsMut<[u8]>` implementors.
impl<T: InSitu + AsMut<[u8]>> InSituMut for T {}

/// Abstracts immutable as well as mutable generic bytes view types like `[u8]` and `mut [u8]` as
/// immutable views.
///
/// With `bytes` feature, abstacts `Bytes` and `BytesMut` as well.
///
/// Requires some standard nice-to-have but easily-to-get traits, so the wrapper can just derive
/// them. Requires methods to be implemented to split views into subviews.
pub trait Raw: AsRef<[u8]> + Default + PartialEq + Eq + PartialOrd + Ord + Debug + Hash {
    /// Splits the bytes into two at the given index.
    ///
    /// Afterwards `self` contains elements `[0, at)`, and the returned `Self` contains elements
    /// `[at, len)`.
    #[must_use]
    fn split_off(&mut self, at: usize) -> Self;
    /// Splits the bytes into two at the given index.
    ///
    /// Afterwards `self` contains elements `[at, len)`, and the returned `Self` contains elements
    /// `[0, at)`.
    #[must_use]
    fn split_to(&mut self, at: usize) -> Self;
}

/// Abstracts mutable generic bytes view types like `mut [u8]` as mutable view.
///
/// With `bytes` feature, abstacts `BytesMut` as well.
///
/// This trait is auto-implemented for `Raw + AsMut<[u8]>` implementors extending the immutable
/// views with mutable ones.
pub trait RawMut: Raw + AsMut<[u8]> {}

// Auto-implement `RawMut` for `Raw + AsMut<[u8]>` implementors.
impl<T: Raw + AsMut<[u8]>> RawMut for T {}

impl Raw for &[u8] {
    fn split_off(&mut self, at: usize) -> Self {
        let (l, r) = self.split_at(at);
        *self = l;
        r
    }
    fn split_to(&mut self, at: usize) -> Self {
        let (l, r) = self.split_at(at);
        *self = r;
        l
    }
}

impl Raw for &mut [u8] {
    fn split_off(&mut self, at: usize) -> Self {
        let slice = mem::take(self);
        let (l, r) = slice.split_at_mut(at);
        *self = l;
        r
    }
    fn split_to(&mut self, at: usize) -> Self {
        let slice = mem::take(self);
        let (l, r) = slice.split_at_mut(at);
        *self = r;
        l
    }
}

#[cfg(feature = "bytes")]
impl Raw for bytes::Bytes {
    fn split_off(&mut self, at: usize) -> Self {
        self.split_off(at)
    }
    fn split_to(&mut self, at: usize) -> Self {
        self.split_to(at)
    }
}

#[cfg(feature = "bytes")]
impl Raw for bytes::BytesMut {
    fn split_off(&mut self, at: usize) -> Self {
        self.split_off(at)
    }
    fn split_to(&mut self, at: usize) -> Self {
        self.split_to(at)
    }
}

/// Type describing the underlying byte order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    /// Big-endian byte order.
    BE,
    /// Little-endian byte order.
    LE,
}

/// Type of `from` methods' argument specifying to take the bytes of either the header only or the
/// whole packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Take {
    /// Take bytes of header only.
    Header,
    /// Take bytes of whole packet.
    Packet,
}
