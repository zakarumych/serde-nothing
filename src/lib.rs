//!
//! This crate defines "nothing" in terms of `serde` data model
//! and allows checking values and create such values using only `Serialize` and `Deserialize` traits.
//!
//! # Motivation
//!
//! This crate is designed to generalize serialization pattern
//! where struct fields with None/empty/default values are skipped on serialization
//! and constructed on deserialization when missing.
//!
//! Usually this pattern is coded using `#[serde(default, skip_serializing_if = "Option::is_none/Vec::is_empty/is_default")]`.
//! Where `is_default` is a function defined as
//! ```rust
//! fn is_default<T: Default + PartialEq>(value: &T) -> bool {
//!     *value == T::default()
//! }
//! ```
//!
//! The pattern works very well for field with concrete types and generic wrappers like `Option`, `Vec` and similar.
//!
//! But using `#[serde(default)]` on field with generic type `T` would require `Default` bound added to the `Deserialize` impl.
//! And `#[serde(skip_serializing_if = "is_default")]` would require `T: Default + PartialEq` bound added to the `Serialize` impl.
//!
//! This crate allows to implement this pattern without additional bounds.
//! Even more, it allows specialize for types that can be skipped.
//! That is, if field has type `T` that does not even have a value that would be appropriate to skip,
//! the code will continue work correctly and would simply always serialize the field and require the data for deserialization.
//!
//! # Magic? No, science!
//!
//! This crate provides stateless `Nothing` type which is a special kind of `serde::Serializer` or `serde::Deserializer`.
//! `Nothing` can be used to serialize and deserialize "nothing" values.
//! "Nothing" values are `None`, empty collections, units,
//! structs and tuples where all fields are "nothing"
//!
//! Serializing a "non-nothing" value with `Nothing` always fails.
//!
//! As deserializer `Nothing` would visit most appropriate `Visitor` method
//! with matching kind of nothingness, like `None`, empty slice/sequence/map,
//! single-variant enum with nothing in discriminant and nothing in payload,
//! `0` numeric value etc.
//!
//! User would most probably want to use a shortcut and utilize `is_nothing` function for serialization
//! and `from_nothing` function for deserialization.
//!

#![cfg_attr(not(feature = "std"), no_std)]

mod de;
mod ser;

pub use self::{de::NothingDeserializeError, ser::NothingSerializeError};

/// Serializer to serialize values into and from nothing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nothing;

/// Returns true if the value matches definition of "nothing".
/// Typically deserializing from `Nothing` would yield same value.
#[inline]
pub fn is_nothing<T: ?Sized>(value: &T) -> bool
where
    T: serde::ser::Serialize,
{
    value.serialize(Nothing).is_ok()
}

#[cfg(test)]
mod tests {
    use core::fmt::Debug;

    use serde::{Deserialize, Serialize};

    use crate::{is_nothing, Nothing};

    #[derive(Debug, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize)]
    struct Struct {
        number: u32,
        string: &'static str,
    }

    #[cfg(test)]
    fn check_roundtrip<'de, T: ?Sized + Serialize + Deserialize<'de> + PartialEq + Debug>(
        value: &T,
    ) {
        assert!(is_nothing(value));
        let () = value.serialize(Nothing).unwrap();
        assert_eq!(Ok(value), T::deserialize(Nothing).as_ref());
    }

    #[test]
    fn test_unit() {
        check_roundtrip(&());
    }

    #[test]
    fn test_numbers() {
        check_roundtrip(&0);
    }

    #[test]
    #[should_panic]
    fn test_numbers_fail() {
        check_roundtrip(&1);
    }

    #[test]
    fn test_string() {
        check_roundtrip(&"");
    }

    #[test]
    #[should_panic]
    fn test_string_fail() {
        check_roundtrip(&"a");
    }

    #[test]
    fn test_slice() {
        check_roundtrip(&&[0u8][..0]);
    }

    #[test]
    #[should_panic]
    fn test_slice_fail() {
        check_roundtrip(&&[0u8][..]);
    }

    #[test]
    fn test_array() {
        check_roundtrip(&[0, 0, 0]);
    }

    #[test]
    #[should_panic]
    fn test_array_fail() {
        check_roundtrip(&[0, 0, 1]);
    }

    #[test]
    fn test_tuple() {
        check_roundtrip(&(0, ""))
    }

    #[test]
    #[should_panic]
    fn test_tuple_fail() {
        check_roundtrip(&(0, "a"))
    }

    #[test]
    fn test_struct() {
        check_roundtrip(&Struct {
            number: 0,
            string: "",
        })
    }

    #[test]
    #[should_panic]
    fn test_struct_fail() {
        check_roundtrip(&Struct {
            number: 1,
            string: "",
        })
    }
}
