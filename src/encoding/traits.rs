use alloc::{string::String, vec::Vec};

use serde::de;

/// A trait for encoding bytes into strings.
pub trait Encoding {
    /// Encodes the byte sequence.
    fn encode(bytes: &[u8]) -> String;

    /// Decodes the byte sequence.
    fn decode<E: de::Error>(string: &str) -> Result<Vec<u8>, E>;
}
