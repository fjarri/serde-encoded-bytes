use core::fmt;

use core::marker::PhantomData;

use serde::{Deserializer, Serializer};

use crate::encoding::Encoding;
use crate::low_level::{self, TryFromArray, TryFromSliceRef};

/// A container for array-like data, e.g. Rust stack arrays, or `generic_array::GenericArray<...>`.
///
/// For use in the `#[serde(with)]` field attribute.
///
/// Note that the length of the array will be serialized as well;
/// this is caused by `serde` not being able to communicate to format implementations
/// that the array has a constant size.
/// See <https://github.com/serde-rs/serde/issues/2120> for details.
///
/// Requirements:
/// - serializer requires the field to implement `AsRef<[u8]>`;
/// - deserializer requires the field to implement [`TryFromArray`] or `TryFrom<[u8; N]>`.
pub struct ArrayLike<Enc: Encoding>(PhantomData<Enc>);

impl<Enc: Encoding> ArrayLike<Enc> {
    /// Serializes array-like data.
    pub fn serialize<T, S>(obj: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: AsRef<[u8]>,
        S: Serializer,
    {
        low_level::serialize_slice::<Enc, _>(obj.as_ref(), serializer)
    }

    /// Deserializes into array-like data.
    pub fn deserialize<'de, T, E, D, const N: usize>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: TryFromArray<E, N>,
        E: fmt::Display,
    {
        low_level::deserialize_array::<Enc, N, _, _, _>(deserializer)
    }
}

/// A container for slice-like data, e.g. `Vec<u8>` or `Box<u8>`.
///
/// For use in the `#[serde(with)]` field attribute.
///
/// Requirements:
/// - serializer requires the field to implement `AsRef<[u8]>`;
/// - deserializer requires the field to implement [`TryFromSliceRef`] or `TryFrom<&[u8]>`.
pub struct SliceLike<Enc: Encoding>(PhantomData<Enc>);

impl<Enc: Encoding> SliceLike<Enc> {
    /// Serializes slice-like data.
    pub fn serialize<T, S>(obj: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: AsRef<[u8]>,
        S: Serializer,
    {
        low_level::serialize_slice::<Enc, _>(obj.as_ref(), serializer)
    }

    /// Deserializes into slice-like data.
    pub fn deserialize<'de, T, E, D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: for<'a> TryFromSliceRef<'a, E>,
        E: fmt::Display,
    {
        low_level::deserialize_slice::<Enc, _, _, _>(deserializer)
    }
}

/// A container for boxed array-like data, e.g. `Box<[u8; 4]>`
/// or `Box<generic_array::GenericArray<...>>`.
///
/// For use in the `#[serde(with)]` field attribute.
///
/// Requirements:
/// - serializer requires the field to implement `AsRef<[u8; N]>`;
/// - deserializer requires the field to implement [`TryFromArray`] or `TryFrom<[u8; N]>`.
pub struct BoxedArrayLike<Enc: Encoding>(PhantomData<Enc>);

impl<Enc: Encoding> BoxedArrayLike<Enc> {
    /// Serializes boxed-array-like data.
    pub fn serialize<T, S, const N: usize>(obj: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: AsRef<[u8; N]>,
        S: Serializer,
    {
        low_level::serialize_slice::<Enc, _>(obj.as_ref(), serializer)
    }

    /// Deserializes into boxed-array-like data.
    pub fn deserialize<'de, T, E, D, const N: usize>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: TryFromArray<E, N>,
        E: fmt::Display,
    {
        low_level::deserialize_array::<Enc, N, _, _, _>(deserializer)
    }
}

#[cfg(test)]
mod tests {
    use alloc::{boxed::Box, vec::Vec};

    use serde::{Deserialize, Serialize};

    use crate::{encoding::Hex, ArrayLike, BoxedArrayLike, SliceLike};

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct FixedArrayStruct(#[serde(with = "ArrayLike::<Hex>")] [u8; 4]);

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct VectorStruct(#[serde(with = "SliceLike::<Hex>")] Vec<u8>);

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct BoxedArrayStruct(#[serde(with = "BoxedArrayLike::<Hex>")] Box<[u8; 4]>);

    #[test]
    fn roundtrip_array() {
        let val = FixedArrayStruct([1, 2, 3, 4]);
        let val_bytes = rmp_serde::to_vec(&val).unwrap();
        let val_back = rmp_serde::from_slice::<FixedArrayStruct>(&val_bytes).unwrap();
        assert_eq!(val, val_back);
    }

    #[test]
    fn roundtrip_slice() {
        let val = VectorStruct([1, 2, 3, 4].into());
        let val_bytes = rmp_serde::to_vec(&val).unwrap();
        let val_back = rmp_serde::from_slice::<VectorStruct>(&val_bytes).unwrap();
        assert_eq!(val, val_back);
    }

    #[test]
    fn roundtrip_boxed_array() {
        let val = BoxedArrayStruct([1, 2, 3, 4].into());
        let val_bytes = rmp_serde::to_vec(&val).unwrap();
        let val_back = rmp_serde::from_slice::<BoxedArrayStruct>(&val_bytes).unwrap();
        assert_eq!(val, val_back);
    }
}
