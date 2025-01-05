use alloc::{format, string::String, vec::Vec};

use serde::de;

use super::Encoding;

/// Encodes the byte sequence into a `0x`-prefixed hexadecimal representation.
pub struct Hex;

impl Encoding for Hex {
    fn encode(bytes: &[u8]) -> String {
        format!("0x{}", hex::encode(bytes))
    }

    fn decode<E: de::Error>(string: &str) -> Result<Vec<u8>, E> {
        let digits = string.strip_prefix("0x").ok_or_else(|| {
            de::Error::invalid_value(
                de::Unexpected::Str(string),
                &"0x-prefixed hex-encoded bytes",
            )
        })?;
        hex::decode(digits).map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::{String, ToString};

    use serde::{Deserialize, Serialize};

    use super::Hex;
    use crate::ArrayLike;

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct ArrayStruct(#[serde(with = "ArrayLike::<Hex>")] [u8; 4]);

    fn hr_serialize<T: Serialize>(value: T) -> Result<String, String> {
        serde_json::to_string(&value).map_err(|err| err.to_string())
    }

    fn hr_deserialize<'de, T: Deserialize<'de>>(string: &'de str) -> Result<T, String> {
        serde_json::from_str::<T>(string).map_err(|err| err.to_string())
    }

    #[test]
    fn roundtrip() {
        let val = ArrayStruct([1, 0xf2, 3, 0xf4]);

        let val_str = hr_serialize(&val).unwrap();
        assert_eq!(val_str, "\"0x01f203f4\"");
        let val_back = hr_deserialize::<ArrayStruct>(&val_str).unwrap();
        assert_eq!(val, val_back);
    }

    #[test]
    fn errors() {
        assert_eq!(
            hr_deserialize::<ArrayStruct>("\"01f203f4\"").unwrap_err(),
            concat![
                "invalid value: string \"01f203f4\", expected 0x-prefixed ",
                "hex-encoded bytes at line 1 column 10"
            ]
        );
        assert_eq!(
            hr_deserialize::<ArrayStruct>("\"0\"").unwrap_err(),
            "invalid value: string \"0\", expected 0x-prefixed hex-encoded bytes at line 1 column 3"
        );
    }

    #[test]
    fn multi_byte_character() {
        // A regression test for a bug in validating a possible hex string
        // that could lead to a panic.
        // `str::get()` takes an index in bytes, but requires it to be aligned
        // to the end of a character, so it can fail for multi-byte characters
        // even if the index is technically lower than `str::len()`.

        // This is a unicode "AE" with a 3-byte UTF-8 encoding (0xE1 0xB4 0x81)
        assert_eq!(
            hr_deserialize::<ArrayStruct>("\"\u{1D01}\"").unwrap_err(),
            concat![
                "invalid value: string \"\u{1D01}\", expected 0x-prefixed ",
                "hex-encoded bytes at line 1 column 5"
            ]
        );
    }
}
