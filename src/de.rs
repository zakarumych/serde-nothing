use core::fmt;

use serde::{
    de::{
        DeserializeSeed, EnumAccess, Expected, MapAccess, SeqAccess, Unexpected, VariantAccess,
        Visitor,
    },
    Deserialize, Deserializer,
};

use crate::Nothing;

struct FewNothing {
    len: usize,
}

impl<'de> SeqAccess<'de> for FewNothing {
    type Error = NothingDeserializeError;

    #[inline]
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, NothingDeserializeError>
    where
        T: DeserializeSeed<'de>,
    {
        if self.len > 0 {
            self.len -= 1;
            seed.deserialize(Nothing).map(Some)
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn next_element<T>(&mut self) -> Result<Option<T>, NothingDeserializeError>
    where
        T: Deserialize<'de>,
    {
        if self.len > 0 {
            self.len -= 1;
            T::deserialize(Nothing).map(Some)
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

/// Error type for `Nothing` deserializer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NothingDeserializeError;

impl fmt::Display for NothingDeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Something expected")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for NothingDeserializeError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl serde::de::Error for NothingDeserializeError {
    #[inline]
    fn custom<T>(_: T) -> Self {
        NothingDeserializeError
    }

    #[inline]
    fn invalid_type(_: Unexpected, _: &dyn Expected) -> Self {
        NothingDeserializeError
    }

    #[inline]
    fn invalid_value(_: Unexpected, _: &dyn Expected) -> Self {
        NothingDeserializeError
    }

    #[inline]
    fn invalid_length(_: usize, _: &dyn Expected) -> Self {
        NothingDeserializeError
    }

    #[inline]
    fn unknown_variant(_: &str, _: &'static [&'static str]) -> Self {
        NothingDeserializeError
    }

    #[inline]
    fn unknown_field(_: &str, _: &'static [&'static str]) -> Self {
        NothingDeserializeError
    }

    #[inline]
    fn missing_field(_: &'static str) -> Self {
        NothingDeserializeError
    }

    #[inline]
    fn duplicate_field(_: &'static str) -> Self {
        NothingDeserializeError
    }
}

impl<'de> Deserializer<'de> for Nothing {
    type Error = NothingDeserializeError;
    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    #[inline]
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(false)
    }

    #[inline]
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(0)
    }

    #[inline]
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(0)
    }

    #[inline]
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(0)
    }

    #[inline]
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(0)
    }

    #[inline]
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(0)
    }

    #[inline]
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(0)
    }

    #[inline]
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(0)
    }

    #[inline]
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(0)
    }

    #[inline]
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i128(0)
    }

    #[inline]
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u128(0)
    }

    #[inline]
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(0.0)
    }

    #[inline]
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(0.0)
    }

    #[inline]
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_char('\0')
    }

    #[inline]
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str("")
    }

    #[inline]
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str("")
    }

    #[inline]
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(&[])
    }

    #[inline]
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bytes(&[])
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_none()
    }

    #[inline]
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    #[inline]
    fn deserialize_unit_struct<V>(
        self,
        _: &'static str,
        visitor: V,
    ) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    #[inline]
    fn deserialize_newtype_struct<V>(
        self,
        _: &'static str,
        visitor: V,
    ) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    #[inline]
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    #[inline]
    fn deserialize_tuple<V>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(FewNothing { len })
    }

    #[inline]
    fn deserialize_tuple_struct<V>(
        self,
        _: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(FewNothing { len })
    }

    #[inline]
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    #[inline]
    fn deserialize_struct<V>(
        self,
        _: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(FewNothing { len: fields.len() })
    }

    #[inline]
    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    #[inline]
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(0)
    }

    #[inline]
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

impl<'de> SeqAccess<'de> for Nothing {
    type Error = NothingDeserializeError;

    #[inline]
    fn next_element_seed<T>(&mut self, _: T) -> Result<Option<T::Value>, NothingDeserializeError>
    where
        T: DeserializeSeed<'de>,
    {
        Ok(None)
    }

    #[inline]
    fn next_element<T>(&mut self) -> Result<Option<T>, NothingDeserializeError>
    where
        T: Deserialize<'de>,
    {
        Ok(None)
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        Some(0)
    }
}

impl<'de> MapAccess<'de> for Nothing {
    type Error = NothingDeserializeError;

    #[inline]
    fn next_key_seed<K>(&mut self, _: K) -> Result<Option<K::Value>, NothingDeserializeError>
    where
        K: DeserializeSeed<'de>,
    {
        Ok(None)
    }

    #[inline]
    fn next_value_seed<V>(&mut self, _: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: DeserializeSeed<'de>,
    {
        Err(NothingDeserializeError)
    }

    #[inline]
    fn next_entry_seed<K, V>(
        &mut self,
        _: K,
        _: V,
    ) -> Result<Option<(K::Value, V::Value)>, NothingDeserializeError>
    where
        K: DeserializeSeed<'de>,
        V: DeserializeSeed<'de>,
    {
        Ok(None)
    }

    #[inline]
    fn next_key<K>(&mut self) -> Result<Option<K>, NothingDeserializeError>
    where
        K: Deserialize<'de>,
    {
        Ok(None)
    }

    #[inline]
    fn next_value<V>(&mut self) -> Result<V, NothingDeserializeError>
    where
        V: Deserialize<'de>,
    {
        Err(NothingDeserializeError)
    }

    #[inline]
    fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, NothingDeserializeError>
    where
        K: Deserialize<'de>,
        V: Deserialize<'de>,
    {
        Ok(None)
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        Some(0)
    }
}

impl<'de> EnumAccess<'de> for Nothing {
    type Error = NothingDeserializeError;

    type Variant = Self;

    #[inline]
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), NothingDeserializeError>
    where
        V: DeserializeSeed<'de>,
    {
        let value = seed.deserialize(self)?;
        Ok((value, self))
    }

    #[inline]
    fn variant<V>(self) -> Result<(V, Self::Variant), NothingDeserializeError>
    where
        V: Deserialize<'de>,
    {
        let value = V::deserialize(self)?;
        Ok((value, self))
    }
}

impl<'de> VariantAccess<'de> for Nothing {
    type Error = NothingDeserializeError;

    #[inline]
    fn unit_variant(self) -> Result<(), NothingDeserializeError> {
        Ok(())
    }

    #[inline]
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, NothingDeserializeError>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }

    #[inline]
    fn newtype_variant<T>(self) -> Result<T, NothingDeserializeError>
    where
        T: Deserialize<'de>,
    {
        T::deserialize(self)
    }

    #[inline]
    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(FewNothing { len })
    }

    #[inline]
    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, NothingDeserializeError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(FewNothing { len: fields.len() })
    }
}

#[test]
fn test_unit() {
    assert_eq!(Deserialize::deserialize(Nothing), Ok(()));
}

#[test]
fn test_numbers() {
    assert_eq!(Deserialize::deserialize(Nothing), Ok(0));
    assert_eq!(Deserialize::deserialize(Nothing), Ok(0.0));
}

#[test]
fn test_string() {
    assert_eq!(Deserialize::deserialize(Nothing), Ok(""));
}

#[test]
fn test_slice() {
    assert_eq!(Deserialize::deserialize(Nothing), Ok(&[0u8; 0][..]));
}

#[test]
fn test_array() {
    assert_eq!(Deserialize::deserialize(Nothing), Ok([0, 0, 0]));
}

#[test]
fn test_tuple() {
    assert_eq!(Deserialize::deserialize(Nothing), Ok((0, "")));
}

#[test]
fn test_struct() {
    #[derive(Debug, PartialEq, Eq, serde_derive::Deserialize)]
    struct Struct {
        number: u32,
        string: &'static str,
    }

    assert_eq!(
        Deserialize::deserialize(Nothing),
        Ok(Struct {
            number: 0,
            string: ""
        })
    );
}
