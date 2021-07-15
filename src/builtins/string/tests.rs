use serde_json::json;

use crate::{builtins::BuiltIns, tests::make_val, value::JSONataValue};

#[test]
fn string() {
    let cases = vec![
        (make_val(json!(null)), "null"),
        (make_val(json!(true)), "true"),
        (make_val(json!(false)), "false"),
    ];
    for (input, expected) in cases {
        let actual = BuiltIns::string(&[input]);
        assert_eq!(actual, Ok(Some(expected.into())));
    }
}

#[test]
fn length() {
    let cases = vec![
        (make_val(json!("hello")), 5),
        (make_val(json!("test test")), 9),
        // TODO: test unicode
    ];
    for (input, expected) in cases {
        let actual = BuiltIns::length(&[input]);
        assert_eq!(actual, Ok(Some(expected.into())));
    }
}
