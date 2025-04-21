#[cfg(test)]
mod tests {
    use serde_value::Value;
    use std::collections::BTreeMap;
    use cdumay_error::ErrorConverter;
    use cdumay_error_toml::{TomlDeserializeError, TomlSerializeError};

    fn sample_context() -> BTreeMap<String, Value> {
        let mut context = BTreeMap::new();
        context.insert("filename".to_string(), Value::String("config.toml".to_string()));
        context.insert("line".to_string(), Value::U64(42));
        context
    }

    #[test]
    fn test_toml_serialize_error_conversion() {
        let error: toml::ser::Error =toml::to_string(&Value::Unit).unwrap_err();

        let context = sample_context();
        let message = "Failed to serialize config".to_string();

        let converted = TomlSerializeError::convert(&error, message.clone(), context.clone());

        assert_eq!(converted.message, message);
        assert_eq!(converted.kind.message_id(), "TOML-00001");
        assert_eq!(converted.details.unwrap(), context);
    }

    #[test]
    fn test_toml_deserialize_error_conversion() {
        // Simulate an error from TOML deserialization
        let raw = "invalid = [1, 2"; // Invalid TOML
        let error: toml::de::Error = toml::from_str::<toml::Value>(raw).unwrap_err();

        let context = sample_context();
        let message = "Failed to parse TOML".to_string();

        let converted = TomlDeserializeError::convert(&error, message.clone(), context.clone());

        assert_eq!(converted.message, message);
        assert_eq!(converted.kind.message_id(), "TOML-00001");
        assert_eq!(converted.details.unwrap(), context);
    }
}
