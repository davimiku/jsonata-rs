#[cfg(test)]
use crate::jsonata;
use serde_json::{json, Value};

pub(crate) fn object_data() -> Value {
    json!({
        "name": "ACME Corp.",
        "address": {
            "street": "Main St.",
            "location": {
                "latitude": 100,
                "longitude": 100
            }
        },
        "years": [1990, 1991],
        "orders": [
            {
                "id": 1,
                "products": [
                    {
                        "id": "p1",
                        "name": "piano",
                        "price": 500
                    },
                    {
                        "id": "p3",
                        "name": "boulder",
                        "price": 0
                    }
                ]
            },
            {
                "id": 2,
                "products": [
                    {
                        "id": "p2",
                        "name": "anvil",
                        "price": 460.01
                    }
                ]
            }
        ]
    })
}

#[test]
fn path_expression() {
    let data = object_data();
    let mut program = jsonata("name").unwrap();

    let actual = program.evaluate(data).unwrap().unwrap();
    let expected = json!("ACME Corp.");

    assert_eq!(actual, expected);
}

#[test]
fn two_level_path_expression() {
    let data = object_data();
    let mut program = jsonata("address.street").unwrap();

    let actual = program.evaluate(data).unwrap().unwrap();
    let expected = json!("Main St.");

    assert_eq!(actual, expected);
}

#[test]
fn three_level_path_expression() {
    let data = object_data();
    let mut program = jsonata("address.location.latitude").unwrap();

    let actual = program.evaluate(data).unwrap().unwrap();
    let expected = json!(100);

    assert_eq!(actual, expected);
}

#[test]
fn path_expression_missing() {
    let data = object_data();
    let mut program = jsonata("notexist").unwrap();

    let actual = program.evaluate(data).unwrap();

    assert!(actual.is_none());
}

#[test]
fn path_expression_array_value() {
    let data = object_data();
    let mut program = jsonata("years").unwrap();

    let actual = program.evaluate(data).unwrap().unwrap();
    let expected = json!([1990, 1991]);

    assert_eq!(actual, expected);
}
    assert_eq!(expected, actual);
}

// FIXME: Test for underscore names
