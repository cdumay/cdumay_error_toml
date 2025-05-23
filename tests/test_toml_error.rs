use cdumay_core::ErrorConverter;
use cdumay_error_toml::{TomlDeserializeErrorConverter, TomlSerializeErrorConverter};
use serde_value::Value;
use std::collections::BTreeMap;

fn sample_context() -> BTreeMap<String, Value> {
    let mut context = BTreeMap::new();
    context.insert("filename".to_string(), Value::String("config.toml".to_string()));
    context.insert("line".to_string(), Value::U64(42));
    context
}

#[test]
fn test_toml_serialize_error_conversion() {
    let error: toml::ser::Error = toml::to_string(&Value::Unit).unwrap_err();

    let context = sample_context();
    let message = "Failed to serialize config".to_string();

    let converted = TomlSerializeErrorConverter::convert(&error, message.clone(), context.clone());

    assert_eq!(converted.message(), message);
    assert_eq!(converted.details(), context);
}

#[test]
fn test_toml_deserialize_error_conversion() {
    // Simulate an error from TOML deserialization
    let raw = "invalid = [1, 2"; // Invalid TOML
    let error: toml::de::Error = toml::from_str::<toml::Value>(raw).unwrap_err();

    let context = sample_context();
    let message = "Failed to parse TOML".to_string();

    let converted = TomlDeserializeErrorConverter::convert(&error, message.clone(), context.clone());

    assert_eq!(converted.message(), message);
    assert_eq!(converted.details(), context);
}
