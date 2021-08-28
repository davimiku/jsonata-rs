mod function;
pub(crate) mod number;
#[cfg(test)]
mod tests;
mod traits;

use std::convert::TryFrom;
use std::fmt;
use std::rc::Rc;

use serde_json::Value;

use crate::ast::dyadic::arithmetic::ArithmeticOpType;
use crate::ast::literal::LiteralValue;
use crate::evaluate::EvaluationError;
use crate::evaluate::EvaluationResult;

use self::function::JSONataFunction;
use self::number::JSONataNumber;
use self::traits::TryNumericOps;

/// Primary data type of JSONata
///
/// This can represent any JSON value or a function
/// Composed of an enum for either:
/// * `Value`
/// * `JSONataFunction`
#[derive(Debug)]
pub enum JSONataValue {
    Value(Value),
    Function(JSONataFunction),
}

impl JSONataValue {
    /// Generates an Option<JSONataValue> from a Option<Value>
    pub fn from_opt_value(val: Option<Value>) -> Option<JSONataValue> {
        val.map(|v| JSONataValue::Value(v))
    }

    /// Generates a JSONataValue that is a function from the given function
    /// and identifier.
    ///
    /// FIXME: 'static lifetime may work here for built-ins but is likely wrong
    /// for user-defined functions.
    pub fn from_func<F, I>(func: F, ident: I) -> Self
    where
        F: 'static + Fn(&[Option<JSONataValue>]) -> EvaluationResult,
        I: Into<String>,
    {
        JSONataFunction {
            func: Rc::new(func),
            ident: ident.into(),
            signature: "".into(),
        }
        .into()
    }

    /// Returns the value as a &Value if possible
    pub fn as_value(&self) -> Option<&Value> {
        match self {
            JSONataValue::Value(val) => Some(val),
            JSONataValue::Function(_) => None,
        }
    }

    /// Checks if the value is a serde_json::Value
    pub fn is_value(&self) -> bool {
        self.as_value().is_some()
    }

    /// Returns the value as a Vec<Value> if possible
    ///
    /// For a Value::Array, returns the internal vec,
    /// otherwise creates a 1-item vec with the internal value.
    pub fn as_vec(&self) -> Option<Vec<Value>> {
        match self.as_value() {
            Some(val) => match val {
                Value::Array(vec) => Some(vec.to_vec()),
                val => Some(vec![val.to_owned()]),
            },
            None => None,
        }
    }

    /// Checks if the value is an instance of Value::Array
    /// with an internal vec
    pub fn is_vec(&self) -> bool {
        self.as_vec().is_some()
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

impl fmt::Display for JSONataValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JSONataValue::Value(val) => write!(f, "{}", val),
            JSONataValue::Function(func) => write!(f, "{}", func),
        }
    }
}

impl From<Value> for JSONataValue {
    fn from(val: Value) -> Self {
        JSONataValue::Value(val)
    }
}

// TODO: See above, may be possible to avoid the clone
impl From<&Value> for JSONataValue {
    fn from(val: &Value) -> Self {
        JSONataValue::Value(val.clone())
    }
}

impl From<&mut Value> for JSONataValue {
    fn from(val: &mut Value) -> Self {
        JSONataValue::Value(val.take())
    }
}

impl From<Vec<Value>> for JSONataValue {
    fn from(val: Vec<Value>) -> Self {
        JSONataValue::Value(val.into())
    }
}

impl From<bool> for JSONataValue {
    fn from(val: bool) -> Self {
        JSONataValue::Value(val.into())
    }
}

impl From<&str> for JSONataValue {
    fn from(s: &str) -> Self {
        JSONataValue::Value(s.into())
    }
}

impl From<String> for JSONataValue {
    fn from(s: String) -> Self {
        JSONataValue::Value(s.into())
    }
}

impl From<usize> for JSONataValue {
    fn from(u: usize) -> Self {
        JSONataValue::Value(u.into())
    }
}

impl From<i32> for JSONataValue {
    fn from(i: i32) -> Self {
        JSONataValue::Value(i.into())
    }
}

impl From<f64> for JSONataValue {
    fn from(f: f64) -> Self {
        JSONataValue::Value(f.into())
    }
}

impl From<LiteralValue> for JSONataValue {
    fn from(val: LiteralValue) -> Self {
        JSONataValue::Value(val.into())
    }
}

impl From<JSONataNumber> for JSONataValue {
    fn from(num: JSONataNumber) -> Self {
        JSONataValue::Value(num.into())
    }
}

impl TryFrom<JSONataValue> for Value {
    type Error = EvaluationError;

    fn try_from(value: JSONataValue) -> Result<Self, Self::Error> {
        match value {
            JSONataValue::Value(val) => Ok(val),
            JSONataValue::Function(func) => Err(EvaluationError::FunctionCannotConvertToValue(
                func.ident().to_string(),
            )),
        }
    }
}

impl PartialEq for JSONataValue {
    fn eq(&self, other: &Self) -> bool {
        match (self.as_value(), other.as_value()) {
            (Some(val1), Some(val2)) => match (val1, val2) {
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
            },

            // In the JSONata exerciser "$sum = $sum" returns true, presumably due to
            // Javascript's equality rules. The JSONata documentation does not define the equality
            // behavior of functions, however, and we'll leave the behavior as an implementation
            // detail until it becomes useful to be able to compare JSONata functions for equality.
            (_, _) => false,
        }
    }
}

impl TryNumericOps for JSONataValue {
    fn try_add(self, rhs: Self) -> Result<Value, EvaluationError> {
        if let (Some(self_val), Some(rhs_val)) = (self.as_value(), rhs.as_value()) {
            match (self_val, rhs_val) {
                (Value::Number(left), Value::Number(right)) => {
                    Ok((JSONataNumber::from(left) + JSONataNumber::from(right)).to_value())
                }
                (_, _) => Err(EvaluationError::DyadicMustBeNumber(
                    ArithmeticOpType::Add.into(),
                )),
            }
        } else {
            Err(EvaluationError::DyadicMustBeNumber(
                ArithmeticOpType::Add.into(),
            ))
        }
    }

    fn try_sub(self, rhs: Self) -> Result<Value, EvaluationError> {
        if let (Some(self_val), Some(rhs_val)) = (self.as_value(), rhs.as_value()) {
            match (self_val, rhs_val) {
                (Value::Number(left), Value::Number(right)) => {
                    Ok((JSONataNumber::from(left) - JSONataNumber::from(right)).to_value())
                }
                (_, _) => Err(EvaluationError::DyadicMustBeNumber(
                    ArithmeticOpType::Sub.into(),
                )),
            }
        } else {
            Err(EvaluationError::DyadicMustBeNumber(
                ArithmeticOpType::Sub.into(),
            ))
        }
    }

    fn try_mul(self, rhs: Self) -> Result<Value, EvaluationError> {
        if let (Some(self_val), Some(rhs_val)) = (self.as_value(), rhs.as_value()) {
            match (self_val, rhs_val) {
                (Value::Number(left), Value::Number(right)) => {
                    Ok((JSONataNumber::from(left) * JSONataNumber::from(right)).to_value())
                }
                (_, _) => Err(EvaluationError::DyadicMustBeNumber(
                    ArithmeticOpType::Mul.into(),
                )),
            }
        } else {
            Err(EvaluationError::DyadicMustBeNumber(
                ArithmeticOpType::Mul.into(),
            ))
        }
    }

    fn try_div(self, rhs: Self) -> Result<Value, EvaluationError> {
        if let (Some(self_val), Some(rhs_val)) = (self.as_value(), rhs.as_value()) {
            match (self_val, rhs_val) {
                (Value::Number(left), Value::Number(right)) => {
                    Ok((JSONataNumber::from(left) / JSONataNumber::from(right)).to_value())
                }
                (_, _) => Err(EvaluationError::DyadicMustBeNumber(
                    ArithmeticOpType::Div.into(),
                )),
            }
        } else {
            Err(EvaluationError::DyadicMustBeNumber(
                ArithmeticOpType::Div.into(),
            ))
        }
    }

    fn try_rem(self, rhs: Self) -> Result<Value, EvaluationError> {
        if let (Some(self_val), Some(rhs_val)) = (self.as_value(), rhs.as_value()) {
            match (self_val, rhs_val) {
                (Value::Number(left), Value::Number(right)) => {
                    Ok((JSONataNumber::from(left) % JSONataNumber::from(right)).to_value())
                }
                (_, _) => Err(EvaluationError::DyadicMustBeNumber(
                    ArithmeticOpType::Div.into(),
                )),
            }
        } else {
            Err(EvaluationError::DyadicMustBeNumber(
                ArithmeticOpType::Div.into(),
            ))
        }
    }
}
