use core::fmt;

use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};

use crate::Nothing;

/// Error type for `Nothing` serializer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NothingSerializeError;

impl fmt::Display for NothingSerializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Not nothing")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for NothingSerializeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl serde::ser::Error for NothingSerializeError {
    fn custom<T>(_: T) -> Self {
        NothingSerializeError
    }
}

impl Serializer for Nothing {
    type Ok = ();
    type Error = NothingSerializeError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<(), NothingSerializeError> {
        if v {
            Err(NothingSerializeError)
        } else {
            Ok(())
        }
    }
    fn serialize_i8(self, v: i8) -> Result<(), NothingSerializeError> {
        if v != 0 {
            Err(NothingSerializeError)
        } else {
            Ok(())
        }
    }
    fn serialize_i16(self, v: i16) -> Result<(), NothingSerializeError> {
        if v != 0 {
            Err(NothingSerializeError)
        } else {
            Ok(())
        }
    }
    fn serialize_i32(self, v: i32) -> Result<(), NothingSerializeError> {
        if v != 0 {
            Err(NothingSerializeError)
        } else {
            Ok(())
        }
    }
    fn serialize_i64(self, v: i64) -> Result<(), NothingSerializeError> {
        if v != 0 {
            Err(NothingSerializeError)
        } else {
            Ok(())
        }
    }
    fn serialize_u8(self, v: u8) -> Result<(), NothingSerializeError> {
        if v != 0 {
            Err(NothingSerializeError)
        } else {
            Ok(())
        }
    }
    fn serialize_u16(self, v: u16) -> Result<(), NothingSerializeError> {
        if v != 0 {
            Err(NothingSerializeError)
        } else {
            Ok(())
        }
    }
    fn serialize_u32(self, v: u32) -> Result<(), NothingSerializeError> {
        if v != 0 {
            Err(NothingSerializeError)
        } else {
            Ok(())
        }
    }
    fn serialize_u64(self, v: u64) -> Result<(), NothingSerializeError> {
        if v != 0 {
            Err(NothingSerializeError)
        } else {
            Ok(())
        }
    }
    fn serialize_i128(self, v: i128) -> Result<(), NothingSerializeError> {
        if v != 0 {
            Err(NothingSerializeError)
        } else {
            Ok(())
        }
    }
    fn serialize_u128(self, v: u128) -> Result<(), NothingSerializeError> {
        if v != 0 {
            Err(NothingSerializeError)
        } else {
            Ok(())
        }
    }
    fn serialize_f32(self, v: f32) -> Result<(), NothingSerializeError> {
        if v != 0.0 {
            Err(NothingSerializeError)
        } else {
            Ok(())
        }
    }
    fn serialize_f64(self, v: f64) -> Result<(), NothingSerializeError> {
        if v != 0.0 {
            Err(NothingSerializeError)
        } else {
            Ok(())
        }
    }
    fn serialize_char(self, v: char) -> Result<(), NothingSerializeError> {
        if v != '\0' {
            Err(NothingSerializeError)
        } else {
            Ok(())
        }
    }
    fn serialize_str(self, v: &str) -> Result<(), NothingSerializeError> {
        if v.is_empty() {
            Ok(())
        } else {
            Err(NothingSerializeError)
        }
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<(), NothingSerializeError> {
        if v.is_empty() {
            Ok(())
        } else {
            Err(NothingSerializeError)
        }
    }
    fn serialize_none(self) -> Result<(), NothingSerializeError> {
        Ok(())
    }
    fn serialize_some<T: ?Sized>(self, _: &T) -> Result<(), NothingSerializeError> {
        Err(NothingSerializeError)
    }
    fn serialize_unit(self) -> Result<(), NothingSerializeError> {
        Ok(())
    }
    fn serialize_unit_struct(self, _: &'static str) -> Result<(), NothingSerializeError> {
        Ok(())
    }
    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<(), NothingSerializeError> {
        Ok(())
    }
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<(), NothingSerializeError>
    where
        T: Serialize,
    {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        value: &T,
    ) -> Result<(), NothingSerializeError>
    where
        T: Serialize,
    {
        value.serialize(self)
    }
    fn serialize_seq(self, _: Option<usize>) -> Result<Self, NothingSerializeError> {
        Ok(self)
    }
    fn serialize_tuple(self, _: usize) -> Result<Self, NothingSerializeError> {
        Ok(self)
    }
    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self, NothingSerializeError> {
        Ok(self)
    }
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self, NothingSerializeError> {
        Ok(self)
    }
    fn serialize_map(self, _: Option<usize>) -> Result<Self, NothingSerializeError> {
        Ok(self)
    }
    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self, NothingSerializeError> {
        Ok(self)
    }
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self, NothingSerializeError> {
        Ok(self)
    }

    fn collect_seq<I>(self, iter: I) -> Result<(), NothingSerializeError>
    where
        I: IntoIterator,
    {
        if iter.into_iter().next().is_some() {
            Err(NothingSerializeError)
        } else {
            Ok(())
        }
    }

    fn collect_map<K, V, I>(self, iter: I) -> Result<(), NothingSerializeError>
    where
        I: IntoIterator<Item = (K, V)>,
    {
        if iter.into_iter().next().is_some() {
            Err(NothingSerializeError)
        } else {
            Ok(())
        }
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> Result<(), NothingSerializeError>
    where
        T: fmt::Display,
    {
        use core::fmt::Write;

        struct WriteEmpty;

        impl Write for WriteEmpty {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                if s.is_empty() {
                    Ok(())
                } else {
                    Err(fmt::Error)
                }
            }
            fn write_char(&mut self, _: char) -> fmt::Result {
                Err(fmt::Error)
            }
        }

        if write!(&mut WriteEmpty, "{}", value).is_ok() {
            Ok(())
        } else {
            Err(NothingSerializeError)
        }
    }
}

impl SerializeSeq for Nothing {
    type Ok = ();
    type Error = NothingSerializeError;

    fn serialize_element<T: ?Sized>(&mut self, _: &T) -> Result<(), NothingSerializeError> {
        Err(NothingSerializeError)
    }

    fn end(self) -> Result<(), NothingSerializeError> {
        Ok(())
    }
}

impl SerializeTuple for Nothing {
    type Ok = ();
    type Error = NothingSerializeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), NothingSerializeError>
    where
        T: Serialize,
    {
        value.serialize(*self)
    }

    fn end(self) -> Result<(), NothingSerializeError> {
        Ok(())
    }
}

impl SerializeTupleStruct for Nothing {
    type Ok = ();
    type Error = NothingSerializeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), NothingSerializeError>
    where
        T: Serialize,
    {
        value.serialize(*self)
    }

    fn end(self) -> Result<(), NothingSerializeError> {
        Ok(())
    }
}

impl SerializeTupleVariant for Nothing {
    type Ok = ();
    type Error = NothingSerializeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), NothingSerializeError>
    where
        T: Serialize,
    {
        value.serialize(*self)
    }

    fn end(self) -> Result<(), NothingSerializeError> {
        Ok(())
    }
}

impl SerializeMap for Nothing {
    type Ok = ();
    type Error = NothingSerializeError;

    fn serialize_key<T: ?Sized>(&mut self, _: &T) -> Result<(), NothingSerializeError> {
        Err(NothingSerializeError)
    }

    fn serialize_value<T: ?Sized>(&mut self, _: &T) -> Result<(), NothingSerializeError> {
        Err(NothingSerializeError)
    }

    fn end(self) -> Result<(), NothingSerializeError> {
        Ok(())
    }
}

impl SerializeStruct for Nothing {
    type Ok = ();
    type Error = NothingSerializeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _: &'static str,
        value: &T,
    ) -> Result<(), NothingSerializeError>
    where
        T: Serialize,
    {
        value.serialize(*self)
    }

    fn skip_field(&mut self, _: &'static str) -> Result<(), NothingSerializeError> {
        Ok(())
    }

    fn end(self) -> Result<(), NothingSerializeError> {
        Ok(())
    }
}

impl SerializeStructVariant for Nothing {
    type Ok = ();
    type Error = NothingSerializeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _: &'static str,
        value: &T,
    ) -> Result<(), NothingSerializeError>
    where
        T: Serialize,
    {
        value.serialize(*self)
    }

    fn skip_field(&mut self, _: &'static str) -> Result<(), NothingSerializeError> {
        Ok(())
    }

    fn end(self) -> Result<(), NothingSerializeError> {
        Ok(())
    }
}

#[test]
fn test_unit() {
    assert_eq!(().serialize(Nothing), Ok(()));
}

#[test]
fn test_numbers() {
    assert_eq!(0.serialize(Nothing), Ok(()));
    assert_eq!(1.serialize(Nothing), Err(NothingSerializeError));
}

#[test]
fn test_string() {
    assert_eq!("".serialize(Nothing), Ok(()));
    assert_eq!("a".serialize(Nothing), Err(NothingSerializeError));
}

#[test]
fn test_slice() {
    assert_eq!([0; 0][..].serialize(Nothing), Ok(()));
    assert_eq!([0][..].serialize(Nothing), Err(NothingSerializeError));
}

#[test]
fn test_array() {
    assert_eq!([0, 0, 0].serialize(Nothing), Ok(()));
    assert_eq!([0, 0, 1].serialize(Nothing), Err(NothingSerializeError));
}

#[test]
fn test_tuple() {
    assert_eq!((0, "").serialize(Nothing), Ok(()));
    assert_eq!((1, "").serialize(Nothing), Err(NothingSerializeError));
    assert_eq!((0, "a").serialize(Nothing), Err(NothingSerializeError));
}

#[test]
fn test_struct() {
    #[derive(serde_derive::Serialize)]
    struct Struct {
        number: u32,
        string: &'static str,
    }

    assert_eq!(
        Struct {
            number: 0,
            string: ""
        }
        .serialize(Nothing),
        Ok(())
    );
    assert_eq!(
        Struct {
            number: 1,
            string: ""
        }
        .serialize(Nothing),
        Err(NothingSerializeError)
    );
    assert_eq!(
        Struct {
            number: 0,
            string: "a"
        }
        .serialize(Nothing),
        Err(NothingSerializeError)
    );
}
