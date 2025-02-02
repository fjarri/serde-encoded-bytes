use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use serde::{de::DeserializeOwned, Serialize};

use super::common::{
    BiggerTestArray, SmallerTestArray, TestArray, TestSlice, BIGGER_EXAMPLE_BYTES, EXAMPLE_BYTES,
    SMALLER_EXAMPLE_BYTES,
};

/// CBOR serialization of [`EXAMPLE_BYTES`].
/// (in "0x80" first three bits, `0b010`, denote a byte string,
/// and the last five, `0b10000`, denote the length)
#[rustfmt::skip]
const CBOR_REF: [u8; 24] = [
    0xa1, 0x65,
    // "value"
    0x76, 0x61, 0x6c, 0x75, 0x65,
    // first three bits, `0b010`, denote a byte string,
    // and the last five, `0b10000`, denote the length
    0x50,
    // The payload
    0, 1, 0xf2, 3, 0xf4, 5, 0xf6, 7, 0xf8, 9, 0xfa, 11, 0xfc, 13, 14, 0xff,
];

fn cbor_serialize<T: Serialize>(value: T) -> Result<Vec<u8>, String> {
    let mut serialized = Vec::new();
    ciborium::ser::into_writer(&value, &mut serialized).map_err(|err| err.to_string())?;
    Ok(serialized)
}

fn cbor_deserialize<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, String> {
    ciborium::de::from_reader::<T, _>(bytes).map_err(|err| err.to_string())
}

#[test]
fn roundtrip_array() {
    let val = TestArray {
        value: EXAMPLE_BYTES,
    };
    let val_bytes = cbor_serialize(&val).unwrap();
    assert_eq!(val_bytes, CBOR_REF);

    let val_back = cbor_deserialize::<TestArray>(&val_bytes).unwrap();
    assert_eq!(val, val_back);

    let val_bigger = BiggerTestArray {
        value: BIGGER_EXAMPLE_BYTES,
    };
    let val_bigger_bytes = cbor_serialize(val_bigger).unwrap();
    assert_eq!(
        cbor_deserialize::<TestArray>(&val_bigger_bytes).unwrap_err(),
        "Semantic(None, \"Expected a bytestring of length 16, got 17\")"
    );

    let val_smaller = SmallerTestArray {
        value: SMALLER_EXAMPLE_BYTES,
    };
    let val_smaller_bytes = cbor_serialize(val_smaller).unwrap();
    assert_eq!(
        cbor_deserialize::<TestArray>(&val_smaller_bytes).unwrap_err(),
        "Semantic(None, \"Expected a bytestring of length 16, got 15\")"
    );
}

#[test]
fn roundtrip_slice() {
    let val = TestSlice {
        value: EXAMPLE_BYTES,
    };
    let val_bytes = cbor_serialize(&val).unwrap();
    assert_eq!(val_bytes, CBOR_REF);

    let val_back = cbor_deserialize::<TestSlice>(&val_bytes).unwrap();
    assert_eq!(val, val_back);
}
