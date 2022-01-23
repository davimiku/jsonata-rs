use serde_json::json;

use crate::builtins::BuiltIns;
use crate::value::JSONataValue;

#[test]
fn string() {
    let cases: Vec<(serde_json::Value, &'static str)> = vec![
        (json!(null), "null"),
        (json!(true), "true"),
        (json!(false), "false"),
    ];
    for (input, expected) in cases {
        let input: JSONataValue = input.into();
        let actual = BuiltIns::string(&input);
        assert_eq!(actual, Ok(Some(expected.into())));
    }
}

#[test]
fn length() {
    let cases = vec![
        (String::from("hello"), 5),
        (String::from("test test"), 9),
        // TODO: test unicode
    ];
    for (input, expected) in cases {
        let actual = BuiltIns::length(input);
        assert_eq!(actual, Ok(Some(expected.into())));
    }
}
