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

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use super::object_data;

    fn get_value_unwrapped(input: &str) -> Value {
        let data = object_data();
        let mut program = crate::jsonata(input).unwrap();

        program.evaluate(data).unwrap().unwrap()
    }

    fn get_value_option(input: &str) -> Option<Value> {
        let data = object_data();
        let mut program = crate::jsonata(input).unwrap();

        program.evaluate(data).unwrap()
    }

    #[test]
    fn path_expression() {
        let input = "name";
        let actual = get_value_unwrapped(input);
        let expected = json!("ACME Corp.");

        assert_eq!(actual, expected);
    }

    #[test]
    fn two_level_path_expression() {
        let input = "address.street";
        let actual = get_value_unwrapped(input);
        let expected = json!("Main St.");

        assert_eq!(actual, expected);
    }

    #[test]
    fn three_level_path_expression() {
        let input = "address.location.latitude";
        let actual = get_value_unwrapped(input);
        let expected = json!(100);

        assert_eq!(actual, expected);
    }

    #[test]
    fn path_expression_missing() {
        let input = "notexist";
        let actual = get_value_option(input);

        assert!(actual.is_none());
    }

    #[test]
    fn path_expression_array_value() {
        let input = "years";
        let actual = get_value_unwrapped(input);
        let expected = json!([1990, 1991]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn path_expression_array_index() {
        let input = "years[1]";
        let actual = get_value_unwrapped(input);
        let expected = json!(1991);

        assert_eq!(actual, expected);
    }

    // FIXME: Test for underscore names
}
