use serde_json::Value;

use crate::evaluate::{Context, EvaluationResult};

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralExpression {
    val: LiteralValue,
}

impl LiteralExpression {
    pub fn evaluate(&self, _: &Context) -> EvaluationResult {
        Ok(Some(self.val.clone().into()))
    }
}

impl From<i64> for LiteralExpression {
    fn from(i: i64) -> Self {
        LiteralExpression { val: i.into() }
    }
}

impl From<String> for LiteralExpression {
    fn from(s: String) -> Self {
        LiteralExpression { val: s.into() }
    }
}

impl From<bool> for LiteralExpression {
    fn from(b: bool) -> Self {
        LiteralExpression { val: b.into() }
    }
}

impl From<LiteralValue> for LiteralExpression {
    fn from(val: LiteralValue) -> Self {
        LiteralExpression { val }
    }
}

impl From<()> for LiteralExpression {
    fn from(val: ()) -> Self {
        LiteralExpression { val: val.into() }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralValue {
    Integer(i64),
    // Float(f64),
    String(String),
    Bool(bool),
    Null,
}

impl From<LiteralValue> for Value {
    /// Convert value from the parser into a serde Value.
    fn from(val: LiteralValue) -> Self {
        match val {
            LiteralValue::Integer(i) => Value::Number(i.into()),
            // LiteralValue::Float(f64)
            LiteralValue::String(s) => Value::String(s),
            LiteralValue::Bool(b) => Value::Bool(b),
            LiteralValue::Null => Value::Null,
        }
    }
}

impl From<String> for LiteralValue {
    /// Convert from a String to a LiteralValue
    fn from(s: String) -> Self {
        LiteralValue::String(s)
    }
}

impl From<&str> for LiteralValue {
    /// Convert from a &str to a LiteralValue
    fn from(s: &str) -> Self {
        LiteralValue::String(s.to_string())
    }
}

impl From<i64> for LiteralValue {
    /// Convert from an i64 to a LiteralValue
    fn from(i: i64) -> Self {
        LiteralValue::Integer(i)
    }
}

impl From<bool> for LiteralValue {
    /// Convert from a bool to a LiteralValue
    fn from(b: bool) -> Self {
        LiteralValue::Bool(b)
    }
}

impl From<()> for LiteralValue {
    /// Convert from an empty tuple (representing null)
    fn from(_: ()) -> Self {
        LiteralValue::Null
    }
}
