use alloc::format;
use core::{fmt, marker::PhantomData};

use serde::{de, Deserializer, Serializer};

use crate::encoding::Encoding;

/// A type of a trait alias, to work around <https://github.com/rust-lang/rust/issues/113517>.
/// If not for that issue, we could just use `TryFrom<&'a [u8]>` directly in the bounds.
pub trait TryFromSliceRef<'a, E>: TryFrom<&'a [u8], Error = E> {}

impl<'a, T> TryFromSliceRef<'a, T::Error> for T where T: TryFrom<&'a [u8]> {}

/// A type of a trait alias, to work around <https://github.com/rust-lang/rust/issues/113517>.
/// If not for that issue, we could just use `TryFrom<[u8; N]>` directly in the bounds.
pub trait TryFromArray<E, const N: usize>: TryFrom<[u8; N], Error = E> {}

impl<T, const N: usize> TryFromArray<T::Error, N> for T where T: TryFrom<[u8; N]> {}

pub(crate) fn serialize_slice<Enc, S>(value: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    Enc: Encoding,
{
    if serializer.is_human_readable() {
        serializer.serialize_str(&Enc::encode(value))
    } else {
        serializer.serialize_bytes(value)
    }
}

struct SliceVisitor<Enc, T, V>(PhantomData<Enc>, PhantomData<T>, PhantomData<V>);

impl<'de, Enc, T, V> de::Visitor<'de> for SliceVisitor<Enc, T, V>
where
    Enc: Encoding,
    T: for<'a> TryFromSliceRef<'a, V>,
    V: fmt::Display,
{
    type Value = T;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a bytestring")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let bytes = Enc::decode(v)?;
        AsRef::<[u8]>::as_ref(&bytes)
            .try_into()
            .map_err(de::Error::custom)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.try_into().map_err(de::Error::custom)
    }
}

struct ArrayVisitor<Enc, T, V, const N: usize>(PhantomData<Enc>, PhantomData<T>, PhantomData<V>);

impl<'de, Enc, T, V, const N: usize> de::Visitor<'de> for ArrayVisitor<Enc, T, V, N>
where
    Enc: Encoding,
    T: TryFromArray<V, N>,
    V: fmt::Display,
{
    type Value = T;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a bytestring of length {N}")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let bytes = Enc::decode(v)?;
        let bytes_len = bytes.len();
        let arr = <[u8; N]>::try_from(bytes).map_err(|_| {
            de::Error::custom(format!(
                "Expected byte array of length {N}, got {bytes_len}",
            ))
        })?;
        T::try_from(arr).map_err(de::Error::custom)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let v_len = v.len();
        let arr = <[u8; N]>::try_from(v).map_err(|_| {
            de::Error::custom(format!("Expected byte array of length {N}, got {v_len}",))
        })?;
        T::try_from(arr).map_err(de::Error::custom)
    }
}

pub(crate) fn deserialize_slice<'de, Enc: Encoding, T, V, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: for<'a> TryFromSliceRef<'a, V>,
    V: fmt::Display,
{
    if deserializer.is_human_readable() {
        deserializer.deserialize_str(SliceVisitor::<Enc, T, V>(
            PhantomData,
            PhantomData,
            PhantomData,
        ))
    } else {
        deserializer.deserialize_bytes(SliceVisitor::<Enc, T, V>(
            PhantomData,
            PhantomData,
            PhantomData,
        ))
    }
}

pub(crate) fn deserialize_array<'de, Enc: Encoding, const N: usize, T, V, D>(
    deserializer: D,
) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: TryFromArray<V, N>,
    V: fmt::Display,
{
    if deserializer.is_human_readable() {
        deserializer.deserialize_str(ArrayVisitor::<Enc, T, V, N>(
            PhantomData,
            PhantomData,
            PhantomData,
        ))
    } else {
        deserializer.deserialize_bytes(ArrayVisitor::<Enc, T, V, N>(
            PhantomData,
            PhantomData,
            PhantomData,
        ))
    }
}

#[cfg(test)]
mod tests {
    use alloc::{
        string::{String, ToString},
        vec::Vec,
    };

    use serde::{Deserialize, Serialize};

    use crate::{encoding::Hex, ArrayLike, SliceLike};

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct ArrayStruct(#[serde(with = "ArrayLike::<Hex>")] [u8; 4]);

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct VectorStruct(#[serde(with = "SliceLike::<Hex>")] Vec<u8>);

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct WrongLength(#[serde(with = "ArrayLike::<Hex>")] [u8; 5]);

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct WrongValue(u32);

    fn bin_serialize<T: Serialize>(value: T) -> Result<Vec<u8>, String> {
        rmp_serde::encode::to_vec(&value).map_err(|err| err.to_string())
    }

    fn bin_deserialize<'de, T: Deserialize<'de>>(bytes: &'de [u8]) -> Result<T, String> {
        rmp_serde::decode::from_slice(bytes).map_err(|err| err.to_string())
    }

    fn hr_serialize<T: Serialize>(value: T) -> Result<String, String> {
        serde_json::to_string(&value).map_err(|err| err.to_string())
    }

    fn hr_deserialize<'de, T: Deserialize<'de>>(string: &'de str) -> Result<T, String> {
        serde_json::from_str::<T>(string).map_err(|err| err.to_string())
    }

    #[test]
    fn array_visitor_human_readable() {
        let val = ArrayStruct([1, 2, 3, 4]);

        // Normal operation
        let val_str = hr_serialize(&val).unwrap();
        let val_back = hr_deserialize::<ArrayStruct>(&val_str).unwrap();
        assert_eq!(val, val_back);

        // Failed to decode
        assert_eq!(
            hr_deserialize::<ArrayStruct>("\"0x0102030\"").unwrap_err(),
            "Odd number of digits at line 1 column 11"
        );

        // Wrong length
        assert_eq!(
            hr_deserialize::<ArrayStruct>("\"0x0102030405\"").unwrap_err(),
            "Expected byte array of length 4, got 5 at line 1 column 14"
        );

        // Unexpected value type
        assert_eq!(
            hr_deserialize::<ArrayStruct>("1").unwrap_err(),
            "invalid type: integer `1`, expected a bytestring of length 4 at line 1 column 1"
        );
    }

    #[test]
    fn array_visitor_binary() {
        let val = ArrayStruct([1, 2, 3, 4]);

        // Normal operation
        let val_bytes = bin_serialize(&val).unwrap();
        let val_back = bin_deserialize::<ArrayStruct>(&val_bytes).unwrap();
        assert_eq!(val, val_back);

        // Wrong length
        let wrong_len_bytes = bin_serialize(WrongLength([1, 2, 3, 4, 5])).unwrap();
        assert_eq!(
            bin_deserialize::<ArrayStruct>(&wrong_len_bytes).unwrap_err(),
            "Expected byte array of length 4, got 5"
        );

        // Unexpected value type
        let wrong_val_bytes = bin_serialize(WrongValue(0x01020304)).unwrap();
        assert_eq!(
            bin_deserialize::<ArrayStruct>(&wrong_val_bytes).unwrap_err(),
            "invalid type: integer `16909060`, expected a bytestring of length 4"
        );
    }

    #[test]
    fn slice_visitor_human_readable() {
        let val = VectorStruct([1, 2, 3, 4].into());

        // Normal operation
        let val_str = hr_serialize(&val).unwrap();
        let val_back = hr_deserialize::<VectorStruct>(&val_str).unwrap();
        assert_eq!(val, val_back);

        // Failed to decode
        assert_eq!(
            hr_deserialize::<VectorStruct>("\"0x0102030\"").unwrap_err(),
            "Odd number of digits at line 1 column 11"
        );

        // Unexpected value type
        assert_eq!(
            hr_deserialize::<VectorStruct>("1").unwrap_err(),
            "invalid type: integer `1`, expected a bytestring at line 1 column 1"
        );
    }

    #[test]
    fn slice_visitor_binary() {
        let val = VectorStruct([1, 2, 3, 4].into());

        // Normal operation
        let val_bytes = bin_serialize(&val).unwrap();
        let val_back = bin_deserialize::<VectorStruct>(&val_bytes).unwrap();
        assert_eq!(val, val_back);

        // Unexpected value type
        let wrong_val_bytes = bin_serialize(WrongValue(0x01020304)).unwrap();
        assert_eq!(
            bin_deserialize::<VectorStruct>(&wrong_val_bytes).unwrap_err(),
            "invalid type: integer `16909060`, expected a bytestring"
        );
    }
}
