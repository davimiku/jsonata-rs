pub(crate) mod number;
#[cfg(test)]
mod tests;
mod traits;

use serde_json::Value;

use crate::ast::dyadic::DyadicOpType;
use crate::evaluate::EvaluationError;

use self::number::JSONataNumber;
use self::traits::TryNumericOps;

pub struct JSONataValue(Value);

impl JSONataValue {
    pub fn value(&self) -> &Value {
        &self.0
    }

    // TODO: Perhaps use Cell<T> to be able to .take
    // from a Value so that a JSONataValue can take ownership
    // of a Value to avoid the clone
    //
    // pub fn from_val(val: &Value) -> Self {
    //     let mut val = val;
    //     JSONataValue::from_val_internal(val)
    // }

    // fn from_val_internal(val: &mut Value) -> Self {
    //     JSONataValue(val.take())
    // }
}
impl From<Value> for JSONataValue {
    fn from(val: Value) -> Self {
        JSONataValue(val)
    }
}

// TODO: See above, may be possible to avoid the clone
impl From<&Value> for JSONataValue {
    fn from(val: &Value) -> Self {
        JSONataValue(val.clone())
    }
}

impl From<&mut Value> for JSONataValue {
    fn from(val: &mut Value) -> Self {
        JSONataValue(val.take())
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
            (_, _) => Err(EvaluationError::DyadicMustBeNumber(DyadicOpType::Add)),
        }
    }

    fn try_sub(self, rhs: Self) -> Result<Value, EvaluationError> {
        match (self.value(), rhs.value()) {
            (Value::Number(left), Value::Number(right)) => {
                Ok((JSONataNumber::from(left) - JSONataNumber::from(right)).to_value())
            }
            (_, _) => Err(EvaluationError::DyadicMustBeNumber(DyadicOpType::Sub)),
        }
    }

    fn try_mul(self, rhs: Self) -> Result<Value, EvaluationError> {
        match (self.value(), rhs.value()) {
            (Value::Number(left), Value::Number(right)) => {
                Ok((JSONataNumber::from(left) * JSONataNumber::from(right)).to_value())
            }
            (_, _) => Err(EvaluationError::DyadicMustBeNumber(DyadicOpType::Mul)),
        }
    }

    fn try_div(self, rhs: Self) -> Result<Value, EvaluationError> {
        match (self.value(), rhs.value()) {
            (Value::Number(left), Value::Number(right)) => {
                Ok((JSONataNumber::from(left) / JSONataNumber::from(right)).to_value())
            }
            (_, _) => Err(EvaluationError::DyadicMustBeNumber(DyadicOpType::Div)),
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
