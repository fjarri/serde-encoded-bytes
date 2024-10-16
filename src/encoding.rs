//! Possible encodings for byte sequences when serializing into human-readable formats.

mod traits;

#[cfg(any(feature = "hex", test))]
mod hex;

#[cfg(feature = "base64")]
mod base64;

pub use traits::Encoding;

#[cfg(any(feature = "hex", test))]
pub use self::hex::Hex;

#[cfg(feature = "base64")]
pub use self::base64::Base64;
