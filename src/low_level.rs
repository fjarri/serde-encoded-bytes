use alloc::format;
use core::{any::type_name, fmt, marker::PhantomData};

use serde::{de, Deserializer, Serializer};

use crate::encoding::Encoding;

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

struct SliceVisitor<Enc, T, E>(PhantomData<(Enc, T, E)>);

impl<Enc, T, E> de::Visitor<'_> for SliceVisitor<Enc, T, E>
where
    Enc: Encoding,
    T: for<'a> TryFrom<&'a [u8], Error = E>,
    E: fmt::Display,
{
    type Value = T;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a bytestring")
    }

    fn visit_str<SE>(self, v: &str) -> Result<Self::Value, SE>
    where
        SE: de::Error,
    {
        let bytes = Enc::decode(v)?;
        let bytes_len = bytes.len();
        AsRef::<[u8]>::as_ref(&bytes).try_into().map_err(|err| {
            de::Error::custom(format!(
                "Failed to instantiate `{}` from a byte slice of length {bytes_len}: {err}",
                type_name::<T>()
            ))
        })
    }

    fn visit_bytes<SE>(self, v: &[u8]) -> Result<Self::Value, SE>
    where
        SE: de::Error,
    {
        let v_len = v.len();
        v.try_into().map_err(|err| {
            de::Error::custom(format!(
                "Failed to instantiate `{}` from a byte slice of length {v_len}: {err}",
                type_name::<T>()
            ))
        })
    }
}

struct BorrowedSliceVisitor<Enc, T, E>(PhantomData<(Enc, T, E)>);

impl<Enc, T, E> de::Visitor<'_> for BorrowedSliceVisitor<Enc, T, E>
where
    Enc: Encoding,
    T: Clone,
    for<'a> &'a T: TryFrom<&'a [u8], Error = E>,
    E: fmt::Display,
{
    type Value = T;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a bytestring")
    }

    fn visit_str<SE>(self, v: &str) -> Result<Self::Value, SE>
    where
        SE: de::Error,
    {
        let bytes = Enc::decode(v)?;
        let bytes_len = bytes.len();
        let result_ref: &T = AsRef::<[u8]>::as_ref(&bytes).try_into().map_err(|err| {
            de::Error::custom(format!(
                "Failed to instantiate `{}` from a byte slice of length {bytes_len}: {err}",
                type_name::<T>()
            ))
        })?;
        Ok(result_ref.clone())
    }

    fn visit_bytes<SE>(self, v: &[u8]) -> Result<Self::Value, SE>
    where
        SE: de::Error,
    {
        let v_len = v.len();
        let result_ref: &T = v.try_into().map_err(|err| {
            de::Error::custom(format!(
                "Failed to instantiate `{}` from a byte slice of length {v_len}: {err}",
                type_name::<T>()
            ))
        })?;
        Ok(result_ref.clone())
    }
}

struct ArrayVisitor<Enc, T, E, const N: usize>(PhantomData<(Enc, T, E)>);

impl<Enc, T, E, const N: usize> de::Visitor<'_> for ArrayVisitor<Enc, T, E, N>
where
    Enc: Encoding,
    T: TryFrom<[u8; N], Error = E>,
    E: fmt::Display,
{
    type Value = T;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a bytestring of length {N}")
    }

    fn visit_str<SE>(self, v: &str) -> Result<Self::Value, SE>
    where
        SE: de::Error,
    {
        let bytes = Enc::decode(v)?;
        let bytes_len = bytes.len();
        let arr = <[u8; N]>::try_from(bytes).map_err(|_| {
            de::Error::custom(format!(
                "Expected a bytestring of length {N}, got {bytes_len}",
            ))
        })?;
        T::try_from(arr).map_err(|err| {
            de::Error::custom(format!(
                "Failed to instantiate `{}` from `[u8; {N}]`: {err}",
                type_name::<T>()
            ))
        })
    }

    fn visit_bytes<SE>(self, v: &[u8]) -> Result<Self::Value, SE>
    where
        SE: de::Error,
    {
        let v_len = v.len();
        let arr = <[u8; N]>::try_from(v).map_err(|_| {
            de::Error::custom(format!("Expected a bytestring of length {N}, got {v_len}",))
        })?;
        T::try_from(arr).map_err(|err| {
            de::Error::custom(format!(
                "Failed to instantiate `{}` from `[u8; {N}]`: {err}",
                type_name::<T>()
            ))
        })
    }
}

#[cfg(feature = "generic-array-014")]
struct GenericArray014Visitor<Enc, L>(PhantomData<(Enc, L)>);

#[cfg(feature = "generic-array-014")]
impl<Enc, L> de::Visitor<'_> for GenericArray014Visitor<Enc, L>
where
    Enc: Encoding,
    L: generic_array_014::ArrayLength<u8>,
{
    type Value = generic_array_014::GenericArray<u8, L>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a bytestring of length {}", L::to_usize())
    }

    fn visit_str<SE>(self, v: &str) -> Result<Self::Value, SE>
    where
        SE: de::Error,
    {
        let bytes = Enc::decode(v)?;
        let bytes_len = bytes.len();
        Self::Value::from_exact_iter(bytes).ok_or_else(|| {
            de::Error::custom(format!(
                "Expected a bytestring of length {}, got {bytes_len}",
                L::to_usize()
            ))
        })
    }

    fn visit_bytes<SE>(self, v: &[u8]) -> Result<Self::Value, SE>
    where
        SE: de::Error,
    {
        let v_len = v.len();
        Self::Value::from_exact_iter(v.iter().copied()).ok_or_else(|| {
            de::Error::custom(format!(
                "Expected a bytestring of length {}, got {v_len}",
                L::to_usize()
            ))
        })
    }
}

pub(crate) fn deserialize_slice<'de, Enc: Encoding, T, E, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: for<'a> TryFrom<&'a [u8], Error = E>,
    E: fmt::Display,
{
    if deserializer.is_human_readable() {
        deserializer.deserialize_str(SliceVisitor::<Enc, T, E>(PhantomData))
    } else {
        deserializer.deserialize_bytes(SliceVisitor::<Enc, T, E>(PhantomData))
    }
}

pub(crate) fn deserialize_borrowed_slice<'de, Enc: Encoding, T, E, D>(
    deserializer: D,
) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Clone,
    for<'a> &'a T: TryFrom<&'a [u8], Error = E>,
    E: fmt::Display,
{
    if deserializer.is_human_readable() {
        deserializer.deserialize_str(BorrowedSliceVisitor::<Enc, T, E>(PhantomData))
    } else {
        deserializer.deserialize_bytes(BorrowedSliceVisitor::<Enc, T, E>(PhantomData))
    }
}

pub(crate) fn deserialize_array<'de, Enc: Encoding, const N: usize, T, E, D>(
    deserializer: D,
) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: TryFrom<[u8; N], Error = E>,
    E: fmt::Display,
{
    if deserializer.is_human_readable() {
        deserializer.deserialize_str(ArrayVisitor::<Enc, T, E, N>(PhantomData))
    } else {
        deserializer.deserialize_bytes(ArrayVisitor::<Enc, T, E, N>(PhantomData))
    }
}

#[cfg(feature = "generic-array-014")]
pub(crate) fn deserialize_generic_array_014<'de, Enc: Encoding, L, D>(
    deserializer: D,
) -> Result<generic_array_014::GenericArray<u8, L>, D::Error>
where
    D: Deserializer<'de>,
    L: generic_array_014::ArrayLength<u8>,
{
    if deserializer.is_human_readable() {
        deserializer.deserialize_str(GenericArray014Visitor::<Enc, L>(PhantomData))
    } else {
        deserializer.deserialize_bytes(GenericArray014Visitor::<Enc, L>(PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use alloc::{
        format,
        string::{String, ToString},
        vec::Vec,
    };

    use serde::{Deserialize, Serialize};

    use crate::{encoding::Hex, ArrayLike, BorrowedSliceLike, SliceLike};

    #[cfg(feature = "generic-array-014")]
    use crate::GenericArray014;

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct ArrayStruct(#[serde(with = "ArrayLike::<Hex>")] [u8; 4]);

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct VectorStruct(#[serde(with = "SliceLike::<Hex>")] Vec<u8>);

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct BorrowStruct<const N: usize>(#[serde(with = "BorrowedSliceLike::<Hex>")] Borrow<N>);

    #[cfg(feature = "generic-array-014")]
    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct GenericArray014Struct<L: generic_array_014::ArrayLength<u8>>(
        #[serde(with = "GenericArray014::<Hex>")] generic_array_014::GenericArray<u8, L>,
    );

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct WrongLength(#[serde(with = "ArrayLike::<Hex>")] [u8; 5]);

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct WrongValue(u32);

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Borrow<const N: usize>([u8; N]);

    impl<const N: usize> AsRef<[u8]> for Borrow<N> {
        fn as_ref(&self) -> &[u8] {
            self.0.as_ref()
        }
    }

    impl<'a, const N: usize> TryFrom<&'a [u8]> for &'a Borrow<N> {
        type Error = String;
        fn try_from(source: &'a [u8]) -> Result<Self, Self::Error> {
            let source_len = source.len();
            if source_len != N {
                return Err(format!("The slice must have length {N}"));
            }

            Ok(unsafe { &*(source.as_ptr() as *const Borrow<N>) })
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct BadType([u8; 4]);

    impl AsRef<[u8]> for BadType {
        fn as_ref(&self) -> &[u8] {
            self.0.as_ref()
        }
    }

    impl TryFrom<[u8; 4]> for BadType {
        type Error = String;
        fn try_from(_source: [u8; 4]) -> Result<Self, Self::Error> {
            Err("BadType cannot deserialize from `[u8; 4]`".into())
        }
    }

    impl TryFrom<&[u8]> for BadType {
        type Error = String;
        fn try_from(source: &[u8]) -> Result<Self, Self::Error> {
            let source_len = source.len();
            Err(format!(
                "BadType cannot deserialize from `&[u8]` of length {source_len}"
            ))
        }
    }

    impl<'a> TryFrom<&'a [u8]> for &'a BadType {
        type Error = String;
        fn try_from(source: &'a [u8]) -> Result<Self, Self::Error> {
            let source_len = source.len();
            Err(format!(
                "BadType cannot deserialize from `&[u8]` of length {source_len}"
            ))
        }
    }

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct BadArrayStruct(#[serde(with = "ArrayLike::<Hex>")] BadType);

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct BadSliceStruct(#[serde(with = "SliceLike::<Hex>")] BadType);

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct BadBorrowSliceStruct(#[serde(with = "BorrowedSliceLike::<Hex>")] BadType);

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
            "Expected a bytestring of length 4, got 5 at line 1 column 14"
        );

        // Unexpected value type
        assert_eq!(
            hr_deserialize::<ArrayStruct>("1").unwrap_err(),
            "invalid type: integer `1`, expected a bytestring of length 4 at line 1 column 1"
        );

        // A struct that always fails on deserialization
        let bad_struct_str = hr_serialize(BadArrayStruct(BadType([1, 2, 3, 4]))).unwrap();
        assert_eq!(
            hr_deserialize::<BadArrayStruct>(&bad_struct_str).unwrap_err(),
            concat![
                "Failed to instantiate `serde_encoded_bytes::low_level::tests::BadType` ",
                "from `[u8; 4]`: BadType cannot deserialize from `[u8; 4]` at line 1 column 12"
            ]
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
            "Expected a bytestring of length 4, got 5"
        );

        // Unexpected value type
        let wrong_val_bytes = bin_serialize(WrongValue(0x01020304)).unwrap();
        assert_eq!(
            bin_deserialize::<ArrayStruct>(&wrong_val_bytes).unwrap_err(),
            "invalid type: integer `16909060`, expected a bytestring of length 4"
        );

        // A struct that always fails on deserialization
        let bad_struct_bytes = bin_serialize(BadArrayStruct(BadType([1, 2, 3, 4]))).unwrap();
        assert_eq!(
            bin_deserialize::<BadArrayStruct>(&bad_struct_bytes).unwrap_err(),
            concat![
                "Failed to instantiate `serde_encoded_bytes::low_level::tests::BadType` ",
                "from `[u8; 4]`: BadType cannot deserialize from `[u8; 4]`"
            ]
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

        // A struct that always fails on deserialization
        let bad_struct_str = hr_serialize(BadSliceStruct(BadType([1, 2, 3, 4]))).unwrap();
        assert_eq!(
            hr_deserialize::<BadSliceStruct>(&bad_struct_str).unwrap_err(),
            concat![
                "Failed to instantiate `serde_encoded_bytes::low_level::tests::BadType` ",
                "from a byte slice of length 4: ",
                "BadType cannot deserialize from `&[u8]` of length 4 at line 1 column 12"
            ]
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

        // A struct that always fails on deserialization
        let bad_struct_bytes = bin_serialize(BadSliceStruct(BadType([1, 2, 3, 4]))).unwrap();
        assert_eq!(
            bin_deserialize::<BadSliceStruct>(&bad_struct_bytes).unwrap_err(),
            concat![
                "Failed to instantiate `serde_encoded_bytes::low_level::tests::BadType` ",
                "from a byte slice of length 4: ",
                "BadType cannot deserialize from `&[u8]` of length 4"
            ]
        );
    }

    #[test]
    fn borrow_slice_visitor_human_readable() {
        let val = BorrowStruct(Borrow([1, 2, 3, 4]));

        // Normal operation
        let val_str = hr_serialize(&val).unwrap();
        let val_back = hr_deserialize::<BorrowStruct<4>>(&val_str).unwrap();
        assert_eq!(val, val_back);

        // Failed to decode
        assert_eq!(
            hr_deserialize::<BorrowStruct<4>>("\"0x0102030\"").unwrap_err(),
            "Odd number of digits at line 1 column 11"
        );

        // Unexpected value type
        assert_eq!(
            hr_deserialize::<BorrowStruct<4>>("1").unwrap_err(),
            "invalid type: integer `1`, expected a bytestring at line 1 column 1"
        );

        // A struct that always fails on deserialization
        let bad_struct_str = hr_serialize(BadBorrowSliceStruct(BadType([1, 2, 3, 4]))).unwrap();
        assert_eq!(
            hr_deserialize::<BadBorrowSliceStruct>(&bad_struct_str).unwrap_err(),
            concat![
                "Failed to instantiate `serde_encoded_bytes::low_level::tests::BadType` ",
                "from a byte slice of length 4: ",
                "BadType cannot deserialize from `&[u8]` of length 4 at line 1 column 12"
            ]
        );
    }

    #[test]
    fn borrow_slice_visitor_binary() {
        let val = BorrowStruct(Borrow([1, 2, 3, 4]));

        // Normal operation
        let val_bytes = bin_serialize(&val).unwrap();
        let val_back = bin_deserialize::<BorrowStruct<4>>(&val_bytes).unwrap();
        assert_eq!(val, val_back);

        // Unexpected value type
        let wrong_val_bytes = bin_serialize(WrongValue(0x01020304)).unwrap();
        assert_eq!(
            bin_deserialize::<BorrowStruct<4>>(&wrong_val_bytes).unwrap_err(),
            "invalid type: integer `16909060`, expected a bytestring"
        );

        // A struct that always fails on deserialization
        let bad_struct_bytes = bin_serialize(BadBorrowSliceStruct(BadType([1, 2, 3, 4]))).unwrap();
        assert_eq!(
            bin_deserialize::<BadBorrowSliceStruct>(&bad_struct_bytes).unwrap_err(),
            concat![
                "Failed to instantiate `serde_encoded_bytes::low_level::tests::BadType` ",
                "from a byte slice of length 4: ",
                "BadType cannot deserialize from `&[u8]` of length 4"
            ]
        );
    }

    #[cfg(feature = "generic-array-014")]
    #[test]
    fn ga014_visitor_human_readable() {
        use generic_array_014::typenum::U4;

        let val = GenericArray014Struct([1, 2, 3, 4].into());

        // Normal operation
        let val_str = hr_serialize(&val).unwrap();
        let val_back = hr_deserialize::<GenericArray014Struct<U4>>(&val_str).unwrap();
        assert_eq!(val, val_back);

        // Failed to decode
        assert_eq!(
            hr_deserialize::<GenericArray014Struct<U4>>("\"0x0102030\"").unwrap_err(),
            "Odd number of digits at line 1 column 11"
        );

        // Unexpected value type
        assert_eq!(
            hr_deserialize::<GenericArray014Struct<U4>>("1").unwrap_err(),
            "invalid type: integer `1`, expected a bytestring of length 4 at line 1 column 1"
        );

        // Length mismatch
        let bad_struct_str = hr_serialize(GenericArray014Struct([1, 2, 3].into())).unwrap();
        assert_eq!(
            hr_deserialize::<GenericArray014Struct<U4>>(&bad_struct_str).unwrap_err(),
            "Expected a bytestring of length 4, got 3 at line 1 column 10"
        );
    }

    #[cfg(feature = "generic-array-014")]
    #[test]
    fn ga014_visitor_binary() {
        use generic_array_014::typenum::U4;

        let val = GenericArray014Struct([1, 2, 3, 4].into());

        // Normal operation
        let val_bytes = bin_serialize(&val).unwrap();
        let val_back = bin_deserialize::<GenericArray014Struct<U4>>(&val_bytes).unwrap();
        assert_eq!(val, val_back);

        // Unexpected value type
        let wrong_val_bytes = bin_serialize(WrongValue(0x01020304)).unwrap();
        assert_eq!(
            bin_deserialize::<GenericArray014Struct<U4>>(&wrong_val_bytes).unwrap_err(),
            "invalid type: integer `16909060`, expected a bytestring of length 4"
        );

        // Length mismatch
        let bad_struct_bytes = bin_serialize(GenericArray014Struct([1, 2, 3].into())).unwrap();
        assert_eq!(
            bin_deserialize::<GenericArray014Struct<U4>>(&bad_struct_bytes).unwrap_err(),
            "Expected a bytestring of length 4, got 3"
        );
    }
}
