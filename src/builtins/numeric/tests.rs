use serde_json::json;

use crate::{builtins::BuiltIns, value::JSONataValue};

#[test]
fn number() {}

#[test]
fn abs() {
    let args = &[JSONataValue::Value(json!(-5))];
    let res = BuiltIns::abs(args);
    assert_eq!(res, Ok(Some(JSONataValue::Value(json!(5)))));
}

#[test]
fn floor() {}

#[test]
fn ceil() {}

#[test]
fn round() {}

#[test]
fn power() {}

#[test]
fn sqrt() {}

#[test]
fn random() {}

#[test]
fn format_number() {}

#[test]
fn format_base() {}

#[test]
fn format_integer() {}

#[test]
fn parse_integer() {}
