use serde_json::Value;

use crate::evaluate::EvaluationError;

use self::number::JSONataNumber;

pub(crate) mod number;

pub struct JSONataValue(Value);

impl JSONataValue {
    pub fn value(&self) -> &Value {
        &self.0
    }
}

impl From<Value> for JSONataValue {
    fn from(val: Value) -> Self {
        JSONataValue(val)
    }
}

impl From<&Value> for JSONataValue {
    /// Convert from &Value to JSONataValue
    ///
    /// TODO: Sprinkle some lifetime magic to remove the need for a clone
    fn from(val: &Value) -> Self {
        JSONataValue(val.clone())
    }
}

impl PartialEq for JSONataValue {
    fn eq(&self, other: &Self) -> bool {
        match (self.value(), other.value()) {
            (Value::Null, Value::Null) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => {
                JSONataNumber::from(a) == JSONataNumber::from(b)
            }
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => {
                if a.len() != b.len() {
                    false
                } else {
                    a.iter()
                        .zip(b)
                        .all(|(l, h)| JSONataValue::from(l) == JSONataValue::from(h))
                }
            }
            (Value::Object(a), Value::Object(b)) => {
                if a.len() != b.len() {
                    false
                } else {
                    a.iter().all(|(key, a_val)| match b.get(key) {
                        Some(b_val) => JSONataValue::from(a_val) == JSONataValue::from(b_val),
                        None => false,
                    })
                }
            }

            (_, _) => false,
        }
    }
}

trait TryNumericOps<Rhs = Self> {
    /// Attempt addition between two values
    ///
    /// Ok if the operands are numeric, Err otherwise
    fn try_add(self, rhs: Rhs) -> Result<Value, EvaluationError>;

    /// Attempt subtraction between two values
    ///
    /// Ok if the operands are numeric, Err otherwise
    fn try_sub(self, rhs: Rhs) -> Result<Value, EvaluationError>;

    /// Attempt multiplication between two values
    ///
    /// Ok if the operands are numeric, Err otherwise
    fn try_mul(self, rhs: Rhs) -> Result<Value, EvaluationError>;

    /// Attempt division between two values
    ///
    /// Ok if the operands are numeric, Err otherwise
    fn try_div(self, rhs: Rhs) -> Result<Value, EvaluationError>;

    /// Attempt remainder operation between two values
    ///
    /// Ok if the operands are numeric, Err otherwise
    fn try_rem(self, rhs: Rhs) -> Result<Value, EvaluationError>;
}

impl TryNumericOps for JSONataValue {
    fn try_add(self, rhs: Self) -> Result<Value, EvaluationError> {
        match (self.value(), rhs.value()) {
            (Value::Number(left), Value::Number(right)) => {
                Ok((JSONataNumber::from(left) + JSONataNumber::from(right)).to_value())
            }
            (_, _) => Err(EvaluationError::BinaryMustBeNumber("+".to_string())),
        }
    }

    fn try_sub(self, rhs: Self) -> Result<Value, EvaluationError> {
        match (self.value(), rhs.value()) {
            (Value::Number(left), Value::Number(right)) => {
                Ok((JSONataNumber::from(left) - JSONataNumber::from(right)).to_value())
            }
            (_, _) => Err(EvaluationError::BinaryMustBeNumber("-".to_string())),
        }
    }

    fn try_mul(self, rhs: Self) -> Result<Value, EvaluationError> {
        match (self.value(), rhs.value()) {
            (Value::Number(left), Value::Number(right)) => {
                Ok((JSONataNumber::from(left) * JSONataNumber::from(right)).to_value())
            }
            (_, _) => Err(EvaluationError::BinaryMustBeNumber("*".to_string())),
        }
    }

    fn try_div(self, rhs: Self) -> Result<Value, EvaluationError> {
        match (self.value(), rhs.value()) {
            (Value::Number(left), Value::Number(right)) => {
                Ok((JSONataNumber::from(left) / JSONataNumber::from(right)).to_value())
            }
            (_, _) => Err(EvaluationError::BinaryMustBeNumber("/".to_string())),
        }
    }

    fn try_rem(self, rhs: Self) -> Result<Value, EvaluationError> {
        // match (self.value(), rhs.value()) {
        //     (Value::Number(left), Value::Number(right)) => {
        //         Ok((JSONataNumber::from(left) % JSONataNumber::from(right)).to_value())
        //     }
        //     (_, _) => Err(EvaluationError::BinaryMustBeNumber("%".to_string())),
        // }
        todo!()
    }
}

#[cfg(test)]
mod tests {
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
                Err(EvaluationError::BinaryMustBeNumber("+".to_string()))
            )
        }
    }

    // TODO: tests for sub, mul, div, rem
}
