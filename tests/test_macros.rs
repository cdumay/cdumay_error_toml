use cdumay_core::ErrorConverter;
use cdumay_error_toml::{convert_deserialize_result, convert_serialize_result};
use serde_value::Value;
use std::collections::BTreeMap;

#[test]
fn test_serialize_macro() {
    let mut context = BTreeMap::new();
    context.insert("filename".to_string(), Value::String("config.toml".to_string()));
    context.insert("line".to_string(), Value::U64(42));
    let converted = convert_serialize_result!(toml::to_string(&Value::Unit), context, "Failed to serialize config");
    assert!(converted.is_err());

    let err = converted.unwrap_err();
    assert!(err.message().contains("Failed to serialize config"));
}

#[test]
fn test_toml_deserialize_error_conversion() {
    let result = toml::from_str::<toml::Value>("invalid = [1, 2");
    let converted = convert_deserialize_result!(result);
    assert!(converted.is_err());

    let err = converted.unwrap_err();
    assert!(err.message().contains("invalid array"));
}
