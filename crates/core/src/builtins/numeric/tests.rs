use serde_json::json;

use crate::builtins::BuiltIns;
use crate::value::number::JSONataNumber;
use crate::value::JSONataValue;

#[test]
fn number() {}

#[test]
fn abs() {
    let cases: Vec<(JSONataNumber, JSONataNumber)> = vec![
        ((-5).into(), 5.into()),
        (5.into(), 5.into()),
        ((-5.3).into(), (5.3).into()),
        ((5.3).into(), (5.3).into()),
    ];
    for (input, expected) in cases {
        let actual = BuiltIns::abs(&input);
        let expected: JSONataValue = expected.into();
        assert_eq!(actual, Ok(Some(expected)));
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
