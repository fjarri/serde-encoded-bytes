use alloc::string::{String, ToString};

use serde::{de::DeserializeOwned, Serialize};

use super::common::{
    BiggerTestArray, SmallerTestArray, TestArray, TestSlice, BIGGER_EXAMPLE_BYTES, EXAMPLE_BYTES,
    SMALLER_EXAMPLE_BYTES,
};

/// Bincode serialization of [`EXAMPLE_BYTES`].
const TOML_REF: &str = "value = \"0x0001f203f405f607f809fa0bfc0d0eff\"\n";

fn toml_serialize<T: Serialize>(value: T) -> Result<String, String> {
    toml::to_string(&value).map_err(|err| err.to_string())
}

fn toml_deserialize<T: DeserializeOwned>(string: &str) -> Result<T, String> {
    toml::from_str::<T>(string).map_err(|err| err.to_string())
}

#[test]
fn roundtrip_array() {
    let val = TestArray {
        value: EXAMPLE_BYTES,
    };
    let val_bytes = toml_serialize(&val).unwrap();
    assert_eq!(val_bytes, TOML_REF);

    let val_back = toml_deserialize::<TestArray>(&val_bytes).unwrap();
    assert_eq!(val, val_back);

    let val_bigger = BiggerTestArray {
        value: BIGGER_EXAMPLE_BYTES,
    };
    let val_bigger_bytes = toml_serialize(val_bigger).unwrap();
    assert!(toml_deserialize::<TestArray>(&val_bigger_bytes).is_err());

    let val_smaller = SmallerTestArray {
        value: SMALLER_EXAMPLE_BYTES,
    };
    let val_smaller_bytes = toml_serialize(val_smaller).unwrap();
    assert!(toml_deserialize::<TestArray>(&val_smaller_bytes).is_err());
}

#[test]
fn roundtrip_slice() {
    let val = TestSlice {
        value: EXAMPLE_BYTES,
    };
    let val_bytes = toml_serialize(&val).unwrap();
    assert_eq!(val_bytes, TOML_REF);

    let val_back = toml_deserialize::<TestSlice>(&val_bytes).unwrap();
    assert_eq!(val, val_back);
}
