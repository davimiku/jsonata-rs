use super::*;
use serde_json::json;

#[test]
fn eq() {
    let cases: Vec<(Value, Value)> = vec![
        (json!(null), json!(null)),
        (json!(true), json!(true)),
        (json!(false), json!(false)),
        (json!("yes"), json!("yes")),
        (json!(1000), json!(1000)),
        (json!(10.0), json!(10)),
        (json!([1, 2]), json!([1.0, 2.0])),
        (json!([1, [2.0]]), json!([1.0, [2]])),
        (
            json!({ "a": true, "b": false }),
            json!({ "b": false, "a": true }),
        ),
    ];
    for (a, b) in cases {
        assert!(JSONataValue(a) == JSONataValue(b));
    }
}

#[test]
fn add() {
    // (lhs, rhs, expected)
    let ok_cases: Vec<(Value, Value, Value)> = vec![
        (json!(1), json!(2), json!(3)),
        (json!(1.5), json!(2.5), json!(4.0)),
        (json!(-1.5), json!(2.5), json!(1.0)),
        (json!(1), json!(2.5), json!(3.5)),
        (json!(100), json!(-250), json!(-150)),
    ];
    for (lhs, rhs, expected) in ok_cases {
        assert_eq!(JSONataValue(lhs).try_add(JSONataValue(rhs)), Ok(expected))
    }

    // (lhs, rhs)
    let err_cases: Vec<(Value, Value)> = vec![
        (json!("hello"), json!("world")),
        (json!(1), json!("1")),
        (json!("1"), json!(1)),
    ];
    for (lhs, rhs) in err_cases {
        assert_eq!(
            JSONataValue(lhs).try_add(JSONataValue(rhs)),
            Err(EvaluationError::DyadicMustBeNumber(DyadicOpType::Add))
        )
    }
}

// TODO: tests for sub, mul, div, rem
