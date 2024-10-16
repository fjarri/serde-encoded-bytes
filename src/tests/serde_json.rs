use alloc::string::{String, ToString};

use serde::{Deserialize, Serialize};

use super::common::{
    BiggerTestArray, SmallerTestArray, TestArray, TestSlice, BIGGER_EXAMPLE_BYTES, EXAMPLE_BYTES,
    SMALLER_EXAMPLE_BYTES,
};

/// Bincode serialization of [`EXAMPLE_BYTES`].
const JSON_REF: &str = "{\"value\":\"0x0001f203f405f607f809fa0bfc0d0eff\"}";

fn json_serialize<T: Serialize>(value: T) -> Result<String, String> {
    serde_json::to_string(&value).map_err(|err| err.to_string())
}

fn json_deserialize<'de, T: Deserialize<'de>>(string: &'de str) -> Result<T, String> {
    serde_json::from_str::<T>(string).map_err(|err| err.to_string())
}

#[test]
fn roundtrip_array() {
    let val = TestArray {
        value: EXAMPLE_BYTES,
    };
    let val_bytes = json_serialize(&val).unwrap();
    assert_eq!(val_bytes, JSON_REF);

    let val_back = json_deserialize::<TestArray>(&val_bytes).unwrap();
    assert_eq!(val, val_back);

    let val_bigger = BiggerTestArray {
        value: BIGGER_EXAMPLE_BYTES,
    };
    let val_bigger_bytes = json_serialize(val_bigger).unwrap();
    assert_eq!(
        json_deserialize::<TestArray>(&val_bigger_bytes).unwrap_err(),
        "Expected byte array of length 16, got 17 at line 1 column 47"
    );

    let val_smaller = SmallerTestArray {
        value: SMALLER_EXAMPLE_BYTES,
    };
    let val_smaller_bytes = json_serialize(val_smaller).unwrap();
    assert_eq!(
        json_deserialize::<TestArray>(&val_smaller_bytes).unwrap_err(),
        "Expected byte array of length 16, got 15 at line 1 column 43"
    );
}

#[test]
fn roundtrip_slice() {
    let val = TestSlice {
        value: EXAMPLE_BYTES,
    };
    let val_bytes = json_serialize(&val).unwrap();
    assert_eq!(val_bytes, JSON_REF);

    let val_back = json_deserialize::<TestSlice>(&val_bytes).unwrap();
    assert_eq!(val, val_back);
}
