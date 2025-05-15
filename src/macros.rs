#[macro_export]
macro_rules! convert_deserialize_result {
    ($result:expr, $context:expr, $text:expr) => {
        $result.map_err(|err| cdumay_error_toml::TomlDeserializeErrorConverter::convert_error(&err, Some($text.to_string()), $context))
    };
    ($result:expr, $text:expr) => {
        $result.map_err(|err| {
            cdumay_error_toml::TomlDeserializeErrorConverter::convert_error(&err, Some($text.to_string()), std::collections::BTreeMap::new())
        })
    };
    ($result:expr) => {
        $result.map_err(|err| cdumay_error_toml::TomlDeserializeErrorConverter::convert_error(&err, None, std::collections::BTreeMap::new()))
    };
}
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
