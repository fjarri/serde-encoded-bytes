use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use serde::{Deserialize, Serialize};

use super::common::{
    BiggerTestArray, SmallerTestArray, TestArray, TestSlice, BIGGER_EXAMPLE_BYTES, EXAMPLE_BYTES,
    SMALLER_EXAMPLE_BYTES,
};

/// Messagepack serialization of [`EXAMPLE_BYTES`].
const MESSAGEPACK_REF: [u8; 19] = [
    0x91, 0xc4, 16, 0, 1, 0xf2, 3, 0xf4, 5, 0xf6, 7, 0xf8, 9, 0xfa, 11, 0xfc, 13, 14, 0xff,
];

fn messagepack_serialize<T: Serialize>(value: T) -> Result<Vec<u8>, String> {
    rmp_serde::encode::to_vec(&value).map_err(|err| err.to_string())
}

fn messagepack_deserialize<'de, T: Deserialize<'de>>(bytes: &'de [u8]) -> Result<T, String> {
    rmp_serde::decode::from_slice(bytes).map_err(|err| err.to_string())
}

#[test]
fn roundtrip_array() {
    let val = TestArray {
        value: EXAMPLE_BYTES,
    };
    let val_bytes = messagepack_serialize(&val).unwrap();
    assert_eq!(val_bytes, MESSAGEPACK_REF);

    let val_back = messagepack_deserialize::<TestArray>(&val_bytes).unwrap();
    assert_eq!(val, val_back);

    let val_bigger = BiggerTestArray {
        value: BIGGER_EXAMPLE_BYTES,
    };
    let val_bigger_bytes = messagepack_serialize(val_bigger).unwrap();
    assert_eq!(
        messagepack_deserialize::<TestArray>(&val_bigger_bytes).unwrap_err(),
        "Expected byte array of length 16, got 17"
    );

    let val_smaller = SmallerTestArray {
        value: SMALLER_EXAMPLE_BYTES,
    };
    let val_smaller_bytes = messagepack_serialize(val_smaller).unwrap();
    assert_eq!(
        messagepack_deserialize::<TestArray>(&val_smaller_bytes).unwrap_err(),
        "Expected byte array of length 16, got 15"
    );
}

#[test]
fn roundtrip_slice() {
    let val = TestSlice {
        value: EXAMPLE_BYTES,
    };
    let val_bytes = messagepack_serialize(&val).unwrap();
    assert_eq!(val_bytes, MESSAGEPACK_REF);

    let val_back = messagepack_deserialize::<TestSlice>(&val_bytes).unwrap();
    assert_eq!(val, val_back);
}
