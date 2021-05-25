//! This crate contains the definition and implementation
//! of `JSONataValue`, which can be used for some operations
//! that `serde_json::Value` does not support.

use std::collections::BTreeMap;

use serde_json::Value;

use super::number::JSONataNumber;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum JSONataValue {
    /// JSON null value
    ///
    /// ex.
    /// ```json
    /// null
    /// ```
    Null,

    /// JSON boolean value
    ///
    /// Either `true` or `false`
    /// ex.
    /// ```json
    /// true
    /// ```
    Bool(bool),

    /// JSON string value
    ///
    /// JSON strings are delimited by double quotes
    /// The following characters can be escaped inside of
    /// a JSON string by using a backslash.
    /// - \b : backspace
    /// - \f : form feed
    /// - \n : newline
    /// - \r : carriage return
    /// - \t : tab
    /// - \" : literal double quote character
    /// - \\ : literal backslash character
    /// - \uXXXX : Unicode code point represented by hex number XXXX
    String(String),

    /// JSON number value
    ///
    /// Internal represented by an enum over the following Rust literal types:
    /// - i64 : Negative integers including negative zero
    /// - u64 : Positive integers including positive zero
    /// - f64 : Floating point numbers negative or positive
    ///
    /// This is represented as a custom type similar to serde_json::Value, however,
    /// it differs in the implementation of PartialEq and PartialOrd. In serde_json,
    /// the values of `1` and `1.0` would not be considered equal, however, in JSONata
    /// these must be considered equal.
    ///
    /// The JSONataNumber implementation implements the comparison details between floats
    /// and ints.
    Number(JSONataNumber),

    /// JSON array value
    ///
    /// An array of JSON values which can be any type.
    Array(Vec<JSONataValue>),

    /// JSON object value
    ///
    /// Key value pairs with a String key and the value of any
    /// JSON value.
    Object(BTreeMap<String, JSONataValue>),
}

impl From<Value> for JSONataValue {
    /// Converts from a serde_json::Value enum variant to
    /// a Value (JSONata value)
    fn from(v: Value) -> Self {
        match v {
            Value::Null => JSONataValue::Null,
            Value::Bool(b) => JSONataValue::Bool(b),
            Value::Number(n) => JSONataValue::Number(n.into()),
            Value::String(s) => JSONataValue::String(s),
            Value::Array(v) => JSONataValue::Array(v.iter().map(|val| val.into()).collect()),
            Value::Object(o) => {
                JSONataValue::Object(o.into_iter().map(|(key, val)| (key, val.into())).collect())
            }
        }
    }
}

impl From<&Value> for JSONataValue {
    /// Converts from a &serde_json::Value enum variant to
    /// a Value (JSONata value)
    fn from(v: &Value) -> Self {
        match v {
            Value::Null => JSONataValue::Null,
            Value::Bool(b) => JSONataValue::Bool(*b),
            Value::Number(n) => JSONataValue::Number(n.into()),
            Value::String(s) => JSONataValue::String(s.to_string()),
            Value::Array(v) => JSONataValue::Array(v.iter().map(|val| val.into()).collect()),
            Value::Object(o) => JSONataValue::Object(
                o.into_iter()
                    .map(|(key, val)| (key.to_string(), val.into()))
                    .collect(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn convert_serde_to_jsonata() {
        assert_eq!(JSONataValue::Null, json!(null).into());
        assert_eq!(JSONataValue::Bool(true), json!(true).into());
        assert_eq!(JSONataValue::Bool(false), json!(false).into());
        assert_eq!(JSONataValue::String("foo".to_string()), json!("foo").into());

        assert_eq!(JSONataValue::Number(10_u64.into()), json!(10).into());
        assert_eq!(
            JSONataValue::Number(JSONataNumber::NegInt(-10)),
            json!(-10).into()
        );
        assert_eq!(JSONataValue::Number(10.5.into()), json!(10.5).into());
    }
}
