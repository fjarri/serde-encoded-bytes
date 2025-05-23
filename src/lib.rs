#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![warn(
    clippy::mod_module_files,
    clippy::unwrap_used,
    clippy::indexing_slicing,
    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_qualifications
)]

extern crate alloc;

mod containers;
mod encoding;
mod low_level;

#[cfg(test)]
mod tests;

pub use containers::{ArrayLike, BorrowedSliceLike, BoxedArrayLike, SliceLike};
pub use encoding::Encoding;

#[cfg(feature = "generic-array-014")]
pub use containers::GenericArray014;

// Specifically enable `Hex` for tests, since we need some encoding to be specified.
// Should be removed when https://github.com/rust-lang/cargo/issues/2911 is fixed.
#[cfg(any(feature = "hex", test))]
pub use encoding::Hex;

#[cfg(feature = "base64")]
pub use encoding::{Base64, Base64Url};
