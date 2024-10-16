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
const JSON_REF: &[u8; 46] = b"{\"value\":\"0x0001f203f405f607f809fa0bfc0d0eff\"}";

fn json_serialize<T: Serialize>(value: T) -> Result<Vec<u8>, String> {
    let mut serialized = [0u8; 256];
    let len = serde_json_core::to_slice(&value, &mut serialized).map_err(|err| err.to_string())?;
    #[allow(clippy::indexing_slicing)]
    Ok(serialized[..len].into())
}

fn json_deserialize<'de, T: Deserialize<'de>>(string: &'de [u8]) -> Result<T, String> {
    let (result, _size) =
        serde_json_core::from_slice::<T>(string).map_err(|err| err.to_string())?;
    Ok(result)
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
        "JSON does not match deserializer’s expected format."
    );

    let val_smaller = SmallerTestArray {
        value: SMALLER_EXAMPLE_BYTES,
    };
    let val_smaller_bytes = json_serialize(val_smaller).unwrap();
    assert_eq!(
        json_deserialize::<TestArray>(&val_smaller_bytes).unwrap_err(),
        "JSON does not match deserializer’s expected format."
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
