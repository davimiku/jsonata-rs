use serde_json::{json, Value};

pub(crate) fn object_data() -> Value {
    json!({
        "name": "ACME Corp.",
        "contact": {
            "name": "John Doe"
        },
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
    use serde_json::Value;

    use crate::jsonata;

    use super::object_data;

    #[test]
    fn literal() {
        let input = "true";
        let mut program = jsonata(input).unwrap();
        let result = program.evaluate(&object_data()).unwrap();
        assert_eq!(result, Some(Value::Bool(true)));
    }
}
