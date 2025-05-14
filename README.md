# cdumay_error_toml

[![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
[![cdumay_error_toml on crates.io](https://img.shields.io/crates/v/cdumay_error_toml)](https://crates.io/crates/cdumay_error_toml)
[![cdumay_error_toml on docs.rs](https://docs.rs/cdumay_error_toml/badge.svg)](https://docs.rs/cdumay_error_toml)
[![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error_toml)

A lightweight utility crate that wraps TOML serialization and deserialization errors (`toml::ser::Error`, `toml::de::Error`) and converts them into structured, typed errors using the [`cdumay_error`](https://!docs.rs/cdumay-error/) framework.

This helps standardize error handling in Rust applications that process TOML configuration or data files, while enriching error details with structured context.

### Features

- Categorizes TOML-related errors into `Serialization` and `Deserialization`
- Provides unique error codes, HTTP status codes, and descriptions
- Supports rich contextual error metadata via `BTreeMap`
- Uses the `cdumay_error::ErrorConverter` trait for easy integration
- Provides a convenient macros for error conversion

### Usage Example

#### Dependencies

```toml
[dependencies]
cdumay_error = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde-value = "0.7"
toml = "0.8"
```

#### Code sample

Using the `TomlDeserializeErrorConverter` and `TomlSerializeErrorConverter` directly:
```rust
use cdumay_error::ErrorConverter;
use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use cdumay_error_toml::{TomlDeserializeErrorConverter, TomlSerializeErrorConverter};

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    debug: bool,
}

fn serialize_config(config: &Config) -> Result<String, cdumay_error::Error> {
    toml::to_string(config).map_err(|e| {
        let mut ctx = BTreeMap::new();
        ctx.insert("config_name".into(), serde_value::Value::String(config.name.clone()));
        TomlSerializeErrorConverter::convert(&e, "Failed to serialize TOML config".into(), ctx)
    })
}

fn deserialize_config(input: &str) -> Result<Config, cdumay_error::Error> {
    toml::from_str::<Config>(input).map_err(|e| {
        let mut ctx = BTreeMap::new();
        ctx.insert("input".into(), serde_value::Value::String(input.to_string()));
        TomlDeserializeErrorConverter::convert(&e, "Failed to deserialize TOML config".into(), ctx)
    })
}
```

#### Example Output

```json
{
  "code": "TOML-00001",
  "status": 400,
  "kind": "Invalid Toml data",
  "message": "Failed to deserialize TOML config",
  "context": {
    "input": "[invalid toml]"
  }
}
```

Using the macros:
```rust
use cdumay_error::ErrorConverter;
use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use cdumay_error_toml::{convert_deserialize_result, convert_serialize_result};

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    debug: bool,
}

fn serialize_config(config: &Config) -> Result<String, cdumay_error::Error> {
    let mut ctx = BTreeMap::new();
    ctx.insert("config_name".into(), serde_value::Value::String(config.name.clone()));
    convert_serialize_result!(toml::to_string(config), ctx, "Failed to serialize TOML config")
}

fn deserialize_config(input: &str) -> Result<Config, cdumay_error::Error> {
    let mut ctx = BTreeMap::new();
    ctx.insert("input".into(), serde_value::Value::String(input.to_string()));
    convert_deserialize_result!(toml::from_str::<Config>(input), ctx, "Failed to deserialize TOML config")
}
```
