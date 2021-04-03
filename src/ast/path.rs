use serde_json::Value;

use crate::evaluate::{Context, Evaluatable, EvaluatableResult};

/// PathExpression is a way to get a Value from the JSON data
///
/// Example:
/// ```json
/// {
///    "name": "ACME Corp.",
///    "address": {
///      "street": "Main St."
///    }
/// }
/// ```
/// The following raw expressions yield these values:
/// * `name` --> `ACME Corp.`
/// * `address.street` --> `Main St.`
///
/// The PathExpression struct holds the identifier to get from the
/// JSON data and recursively nested PathExpression structs for any
/// amount of nested members.
///
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PathExpression {
    pub ident: String,
    pub member: Option<Box<PathExpression>>,
}

/// Evaluates a Path expression
impl Evaluatable for PathExpression {
    /// Evaluate a Path expression
    fn evaluate(&self, context: &mut Context) -> EvaluatableResult {
        let result = self.get_member(context.data());
        Ok(result)
    }
}

impl PathExpression {
    fn get_member(&self, data: &Value) -> Option<Value> {
        if data.is_object() {
            let value = data.get(self.ident.clone())?;
            if let Some(m) = &self.member {
                return m.get_member(value);
            } else {
                return Some(value.clone());
            }
        } else if let Some(arr) = data.as_array() {
            let mut values: Vec<Value> = Vec::new();
            for value in arr {
                if let Some(member) = self.get_member(value) {
                    values.push(member);
                }
            }
            if values.len() > 0 {
                return Some(Value::Array(values));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {

    use serde_json::json;

    use crate::tests::object_data;

    use super::*;

    #[test]
    fn path_get_member() {
        let data = object_data();
        let path = PathExpression {
            ident: "name".to_string(),
            member: None,
        };

        let actual = path.get_member(&data).unwrap();
        let expected = json!("ACME Corp.");

        assert_eq!(actual, expected);
    }

    #[test]
    fn path_get_nested_member() {
        let data = object_data();
        let path = PathExpression {
            ident: "address".to_string(),
            member: Some(Box::new(PathExpression {
                ident: "street".to_string(),
                member: None,
            })),
        };

        let actual = path.get_member(&data).unwrap();
        let expected = json!("Main St.");

        assert_eq!(actual, expected);
    }

    #[test]
    fn path_get_multiple_members_from_array() {
        let data = object_data();
        let path = PathExpression {
            ident: "orders".to_string(),
            member: Some(Box::new(PathExpression {
                ident: "id".to_string(),
                member: None,
            })),
        };

        let actual = path.get_member(&data).unwrap();
        let expected = json!([1, 2]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn path_get_nonexisting_from_array() {
        let data = object_data();
        let path = PathExpression {
            ident: "orders".to_string(),
            member: Some(Box::new(PathExpression {
                ident: "notexist".to_string(),
                member: None,
            })),
        };

        let actual = path.get_member(&data);
        assert!(actual.is_none());
    }
}
