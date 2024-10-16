use serde::{Deserialize, Serialize};

use crate::{ArrayLike, Hex, SliceLike};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TestArray {
    #[serde(with = "ArrayLike::<Hex>")]
    pub(crate) value: [u8; 16],
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct BiggerTestArray {
    #[serde(with = "ArrayLike::<Hex>")]
    pub(crate) value: [u8; 17],
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SmallerTestArray {
    #[serde(with = "ArrayLike::<Hex>")]
    pub(crate) value: [u8; 15],
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TestSlice {
    #[serde(with = "SliceLike::<Hex>")]
    pub(crate) value: [u8; 16],
}

pub(crate) const EXAMPLE_BYTES: [u8; 16] = [
    0, 1, 0xf2, 3, 0xf4, 5, 0xf6, 7, 0xf8, 9, 0xfa, 11, 0xfc, 13, 14, 0xff,
];

pub(crate) const BIGGER_EXAMPLE_BYTES: [u8; 17] = [
    0, 1, 0xf2, 3, 0xf4, 5, 0xf6, 7, 0xf8, 9, 0xfa, 11, 0xfc, 13, 14, 0xff, 0xfe,
];

pub(crate) const SMALLER_EXAMPLE_BYTES: [u8; 15] = [
    1, 0xf2, 3, 0xf4, 5, 0xf6, 7, 0xf8, 9, 0xfa, 11, 0xfc, 13, 14, 0xff,
];
