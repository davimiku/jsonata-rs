use serde_json::json;

use super::*;

#[ignore]
#[test]
fn test_append() {
    // Nones
    assert_eq!(BuiltIns::append(&[None, None]), Ok(None));
    assert_eq!(
        BuiltIns::append(&[None, Some(vec![Value::Null].into())]),
        Ok(Some(vec![Value::Null].into())),
    );
    assert_eq!(
        BuiltIns::append(&[Some(vec![Value::Null].into()), None]),
        Ok(Some(vec![Value::Null].into())),
    );

    // Somes
    assert_eq!(
        BuiltIns::append(&[
            Some(vec![Value::Bool(true)].into()),
            Some(vec![Value::Bool(false)].into())
        ]),
        Ok(Some(vec![Value::Bool(true), Value::Bool(false)].into())),
    );
}
