use serde_json::json;

use crate::{builtins::BuiltIns, value::JSONataValue};

#[test]
fn number() {}

#[test]
fn abs() {
    let cases = vec![
        (
            JSONataValue::Value(json!(-5)),
            JSONataValue::Value(json!(5)),
        ),
        (JSONataValue::Value(json!(5)), JSONataValue::Value(json!(5))),
        (
            JSONataValue::Value(json!(-5.3)),
            JSONataValue::Value(json!(5.3)),
        ),
        (
            JSONataValue::Value(json!(5.3)),
            JSONataValue::Value(json!(5.3)),
        ),
    ];
    for case in cases {
        assert_eq!(BuiltIns::abs(&[Some(case.0)]), Ok(Some(case.1)));
    }
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
