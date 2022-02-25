# serde-nothing

[![crates](https://img.shields.io/crates/v/serde-nothing.svg?style=for-the-badge&label=serde-nothing)](https://crates.io/crates/serde-nothing)
[![docs](https://img.shields.io/badge/docs.rs-serde--nothing-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white)](https://docs.rs/serde-nothing)
[![actions](https://img.shields.io/github/workflow/status/zakarumych/serde-nothing/badge/master?style=for-the-badge)](https://github.com/zakarumych/serde-nothing/actions?query=workflow%3ARust)
[![MIT/Apache](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?style=for-the-badge)](COPYING)
![loc](https://img.shields.io/tokei/lines/github/zakarumych/serde-nothing?style=for-the-badge)


This crate defines "nothing" in terms of `serde` data model
and allows checking values and create such values using only `Serialize` and `Deserialize` traits.

# Motivation

This crate is designed to generalize serialization pattern
where struct fields with None/empty/default values are skipped on serialization
and constructed on deserialization when missing.

Usually this pattern is coded using `#[serde(default, skip_serializing_if = "Option::is_none/Vec::is_empty/is_default")]`.
Where `is_default` is a function defined as
```rust
fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}
```

The pattern works very well for field with concrete types and generic wrappers like `Option`, `Vec` and similar.

But using `#[serde(default)]` on field with generic type `T` would require `Default` bound added to the `Deserialize` impl.
And `#[serde(skip_serializing_if = "is_default")]` would require `T: Default + PartialEq` bound added to the `Serialize` impl.

This crate allows to implement this pattern without additional bounds.
Even more, it allows specialize for types that can be skipped.
That is, if field has type `T` that does not even have a value that would be appropriate to skip,
the code will continue work correctly and would simply always serialize the field and require the data for deserialization.

# Magic? No, science!

This crate provides stateless `Nothing` type which is a special kind of `serde::Serializer` or `serde::Deserializer`.
`Nothing` can be used to serialize and deserialize "nothing" values.
"Nothing" values are `None`, empty collections, units,
structs and tuples where all fields are "nothing"

Serializing a "non-nothing" value with `Nothing` always fails.

As deserializer `Nothing` would visit most appropriate `Visitor` method
with matching kind of nothingness, like `None`, empty slice/sequence/map,
single-variant enum with nothing in discriminant and nothing in payload,
`0` numeric value etc.

User would most probably want to use a shortcut and utilize `is_nothing` function for serialization
and `from_nothing` function for deserialization.


## License

Licensed under either of

* Apache License, Version 2.0, ([license/APACHE](license/APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([license/MIT](license/MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
