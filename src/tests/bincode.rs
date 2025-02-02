use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use serde::{Deserialize, Serialize};

use super::common::{
    BiggerTestArray, SmallerTestArray, TestArray, TestSlice, BIGGER_EXAMPLE_BYTES, EXAMPLE_BYTES,
    SMALLER_EXAMPLE_BYTES,
};

/// Bincode serialization of [`EXAMPLE_BYTES`].
const BINCODE_REF: [u8; 17] = [
    16, 0, 1, 0xf2, 3, 0xf4, 5, 0xf6, 7, 0xf8, 9, 0xfa, 11, 0xfc, 13, 14, 0xff,
];

fn bincode_serialize<T: Serialize>(value: T) -> Result<Vec<u8>, String> {
    bincode::serde::encode_to_vec(value, bincode::config::standard()).map_err(|err| err.to_string())
}

fn bincode_deserialize<'de, T: Deserialize<'de>>(bytes: &'de [u8]) -> Result<T, String> {
    bincode::serde::decode_borrowed_from_slice(bytes, bincode::config::standard())
        .map_err(|err| err.to_string())
}

#[test]
fn roundtrip_array() {
    let val = TestArray {
        value: EXAMPLE_BYTES,
    };
    let val_bytes = bincode_serialize(&val).unwrap();
    assert_eq!(val_bytes, BINCODE_REF);

    let val_back = bincode_deserialize::<TestArray>(&val_bytes).unwrap();
    assert_eq!(val, val_back);

    let val_bigger = BiggerTestArray {
        value: BIGGER_EXAMPLE_BYTES,
    };
    let val_bigger_bytes = bincode_serialize(val_bigger).unwrap();
    assert_eq!(
        bincode_deserialize::<TestArray>(&val_bigger_bytes).unwrap_err(),
        "OtherString(\"Expected a bytestring of length 16, got 17\")"
    );

    let val_smaller = SmallerTestArray {
        value: SMALLER_EXAMPLE_BYTES,
    };
    let val_smaller_bytes = bincode_serialize(val_smaller).unwrap();
    assert_eq!(
        bincode_deserialize::<TestArray>(&val_smaller_bytes).unwrap_err(),
        "OtherString(\"Expected a bytestring of length 16, got 15\")"
    );
}

#[test]
fn roundtrip_slice() {
    let val = TestSlice {
        value: EXAMPLE_BYTES,
    };
    let val_bytes = bincode_serialize(&val).unwrap();
    assert_eq!(val_bytes, BINCODE_REF);

    let val_back = bincode_deserialize::<TestSlice>(&val_bytes).unwrap();
    assert_eq!(val, val_back);
}
