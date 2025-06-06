use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use serde::{Deserialize, Serialize};

use super::common::{
    BiggerTestArray, SmallerTestArray, TestArray, TestSlice, BIGGER_EXAMPLE_BYTES, EXAMPLE_BYTES,
    SMALLER_EXAMPLE_BYTES,
};

/// ASN.1 serialization of [`EXAMPLE_BYTES`].
const ASN1_REF: [u8; 20] = [
    48, 18, 4, 16, 0, 1, 0xf2, 3, 0xf4, 5, 0xf6, 7, 0xf8, 9, 0xfa, 11, 0xfc, 13, 14, 0xff,
];

fn asn1_serialize<T: Serialize>(value: T) -> Result<Vec<u8>, String> {
    serde_asn1_der::to_vec(&value).map_err(|err| err.to_string())
}

fn asn1_deserialize<'de, T: Deserialize<'de>>(bytes: &'de [u8]) -> Result<T, String> {
    serde_asn1_der::from_bytes(bytes).map_err(|err| err.to_string())
}

#[test]
fn roundtrip_array() {
    let val = TestArray {
        value: EXAMPLE_BYTES,
    };
    let val_bytes = asn1_serialize(&val).unwrap();
    assert_eq!(val_bytes, ASN1_REF);

    let val_back = asn1_deserialize::<TestArray>(&val_bytes).unwrap();
    assert_eq!(val, val_back);

    let val_bigger = BiggerTestArray {
        value: BIGGER_EXAMPLE_BYTES,
    };
    let val_bigger_bytes = asn1_serialize(val_bigger).unwrap();
    assert_eq!(
        asn1_deserialize::<TestArray>(&val_bigger_bytes).unwrap_err(),
        "Serde error: Expected a bytestring of length 16, got 17"
    );

    let val_smaller = SmallerTestArray {
        value: SMALLER_EXAMPLE_BYTES,
    };
    let val_smaller_bytes = asn1_serialize(val_smaller).unwrap();
    assert_eq!(
        asn1_deserialize::<TestArray>(&val_smaller_bytes).unwrap_err(),
        "Serde error: Expected a bytestring of length 16, got 15"
    );
}

#[test]
fn roundtrip_slice() {
    let val = TestSlice {
        value: EXAMPLE_BYTES,
    };
    let val_bytes = asn1_serialize(&val).unwrap();
    assert_eq!(val_bytes, ASN1_REF);

    let val_back = asn1_deserialize::<TestSlice>(&val_bytes).unwrap();
    assert_eq!(val, val_back);
}
