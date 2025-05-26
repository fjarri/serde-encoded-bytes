use alloc::{string::String, vec::Vec};

use base64::{engine::general_purpose, Engine as _};
use serde::de;

use super::Encoding;

/// Encodes the byte sequence into a base64 representation.
///
/// Uses the standard base64 alphabet (with `+` and `/`) and does not include padding.
pub struct Base64;

impl Encoding for Base64 {
    fn encode(bytes: &[u8]) -> String {
        general_purpose::STANDARD_NO_PAD.encode(bytes)
    }

    fn decode<E: de::Error>(string: &str) -> Result<Vec<u8>, E> {
        general_purpose::STANDARD_NO_PAD
            .decode(string)
            .map_err(de::Error::custom)
    }
}

/// Encodes the byte sequence into a base64-url representation.
///
/// Uses the URL-safe base64 alphabet (with `-` and `_` instead of `+` and `/`)
/// and does not include padding.
pub struct Base64Url;

impl Encoding for Base64Url {
    fn encode(bytes: &[u8]) -> String {
        general_purpose::URL_SAFE_NO_PAD.encode(bytes)
    }

    fn decode<E: de::Error>(string: &str) -> Result<Vec<u8>, E> {
        general_purpose::URL_SAFE_NO_PAD
            .decode(string)
            .map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::{String, ToString};

    use serde::{Deserialize, Serialize};

    use super::{Base64, Base64Url};
    use crate::ArrayLike;

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct ArrayStruct(#[serde(with = "ArrayLike::<Base64>")] [u8; 4]);

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct ArrayStructUrlSafe(#[serde(with = "ArrayLike::<Base64Url>")] [u8; 4]);

    fn hr_serialize<T: Serialize>(value: T) -> Result<String, String> {
        serde_json::to_string(&value).map_err(|err| err.to_string())
    }

    fn hr_deserialize<'de, T: Deserialize<'de>>(string: &'de str) -> Result<T, String> {
        serde_json::from_str::<T>(string).map_err(|err| err.to_string())
    }

    #[test]
    fn roundtrip() {
        let val = ArrayStruct([1, 0xf2, 63, 0xf4]);

        let val_str = hr_serialize(&val).unwrap();
        assert_eq!(val_str, "\"AfI/9A\"");
        let val_back = hr_deserialize::<ArrayStruct>(&val_str).unwrap();
        assert_eq!(val, val_back);
    }

    #[test]
    fn roundtrip_url_safe() {
        let val = ArrayStructUrlSafe([1, 0xf2, 63, 0xf4]);

        let val_str = hr_serialize(&val).unwrap();
        assert_eq!(val_str, "\"AfI_9A\"");
        let val_back = hr_deserialize::<ArrayStructUrlSafe>(&val_str).unwrap();
        assert_eq!(val, val_back);
    }
}
