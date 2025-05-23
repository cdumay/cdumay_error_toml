/// Converts a [`toml::de::Error`] into a [`cdumay_core::Error`].
#[macro_export]
macro_rules! convert_deserialize_result {
    ($result:expr, $context:expr, $text:expr) => {
        $result.map_err(|err| cdumay_error_toml::TomlDeserializeErrorConverter::convert_error(&err, Some($text.to_string()), $context))
    };
    ($result:expr, $context:expr) => {
        $result.map_err(|err| {
            cdumay_error_toml::TomlDeserializeErrorConverter::convert_error(&err,  None, $context)
        })
    };
    ($result:expr) => {
        $result.map_err(|err| cdumay_error_toml::TomlDeserializeErrorConverter::convert_error(&err, None, std::collections::BTreeMap::new()))
    };
}
/// Converts a [`toml::ser::Error`] into a [`cdumay_core::Error`].
#[macro_export]
macro_rules! convert_serialize_result {
    ($result:expr, $context:expr, $text:expr) => {
        $result.map_err(|err| cdumay_error_toml::TomlSerializeErrorConverter::convert_error(&err, Some($text.to_string()), $context))
    };
    ($result:expr, $context:expr) => {
        $result.map_err(|err| {
            cdumay_error_toml::TomlSerializeErrorConverter::convert_error(&err, None, $context)
        })
    };
    ($result:expr) => {
        $result.map_err(|err| cdumay_error_toml::TomlSerializeErrorConverter::convert_error(&err, None, std::collections::BTreeMap::new()))
    };
}
