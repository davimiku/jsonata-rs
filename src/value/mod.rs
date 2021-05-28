pub(crate) mod number;
#[cfg(test)]
mod tests;
mod traits;

use serde_json::Value;

use crate::evaluate::EvaluationError;

use self::number::JSONataNumber;
use self::traits::TryNumericOps;

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
