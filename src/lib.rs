//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay_error_toml on crates.io](https://img.shields.io/crates/v/cdumay_error_toml)](https://crates.io/crates/cdumay_error_toml)
//! [![cdumay_error_toml on docs.rs](https://docs.rs/cdumay_error_toml/badge.svg)](https://docs.rs/cdumay_error_toml)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error_toml)
//!
//! This crate provides structured error types for handling
//! Toml errors by categorizing them into descriptive,
//! typed error variants with associated codes and messages.

use cdumay_error::{AsError, Error, ErrorConverter, define_errors, define_kinds};
use std::collections::BTreeMap;

/// Defines a custom error kind for TOML-related operations.
///
/// - Code: `TOML-00001`
/// - HTTP Status: `400 Bad Request`
/// - Description: `Invalid Toml data`
define_kinds! {
    TomlData = ("TOML-00001", 400, "Invalid Toml data"),
}

/// Defines typed error structs for TOML serialization and deserialization,
/// both using the shared `TomlData` error kind.
define_errors! {
    TomlDeserialize = TomlData,
    TomlSerialize = TomlData,
}

/// Wrapper type for converting `toml::ser::Error`
/// into a structured `cdumay_error::Error`.
pub struct TomlSerializeError;

impl ErrorConverter for TomlSerializeError {
    type Error = toml::ser::Error;

    /// Converts a `toml::ser::Error` into a `cdumay_error::Error`.
    ///
    /// # Arguments
    /// - `error`: The original TOML serialization error (not used here).
    /// - `text`: The error message to associate with the failure.
    /// - `context`: Additional structured metadata to provide insight into the error.
    ///
    /// # Returns
    /// A typed `Error` with kind `TomlSerialize`, enriched with message and context.
    fn convert(_: &Self::Error, text: String, context: BTreeMap<String, serde_value::Value>) -> Error {
        TomlSerialize::new().set_message(text).set_details(context).into()
    }
}

/// Wrapper type for converting `toml::de::Error`
/// into a structured `cdumay_error::Error`.
pub struct TomlDeserializeError;

impl ErrorConverter for TomlDeserializeError {
    type Error = toml::de::Error;

    /// Converts a `toml::de::Error` into a `cdumay_error::Error`.
    ///
    /// # Arguments
    /// - `error`: The original TOML deserialization error (not used here).
    /// - `text`: The error message to associate with the failure.
    /// - `context`: Additional structured metadata to provide insight into the error.
    ///
    /// # Returns
    /// A typed `Error` with kind `TomlDeserialize`, enriched with message and context.
    fn convert(_: &Self::Error, text: String, context: BTreeMap<String, serde_value::Value>) -> Error {
        TomlDeserialize::new().set_message(text).set_details(context).into()
    }
}
