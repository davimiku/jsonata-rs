use serde_json::{json, Value};

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4)
}

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
