use serde_json::Value;

use crate::evaluate::{Context, EvaluationResult};

use super::expression::{Expression, ExpressionType};

pub struct LiteralExpression {
    val: LiteralValue,
}

impl LiteralExpression {
    pub fn from_int(i: i64) -> Self {
        LiteralExpression { val: i.into() }
    }

    pub fn from_string(s: String) -> Self {
        LiteralExpression { val: s.into() }
    }

    pub fn val(&self) -> &LiteralValue {
        &self.val
    }
}

impl Expression for LiteralExpression {
    fn etype(&self) -> ExpressionType {
        ExpressionType::Literal
    }

    fn evaluate(&self, _context: &Context) -> EvaluationResult {
        Ok(Some(self.val.into()))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralValue {
    Integer(i64),
    // Float(f64),
    String(String),
    Bool(bool),
}

impl From<LiteralValue> for Value {
    /// Convert value from the lexer into a
    /// serde_json Value.
    fn from(val: LiteralValue) -> Self {
        match val {
            LiteralValue::Integer(i) => Value::Number(i.into()),
            // LiteralValue::Float(f64)
            LiteralValue::String(s) => Value::String(s),
            LiteralValue::Bool(b) => Value::Bool(b),
        }
    }
}

impl From<String> for LiteralValue {
    /// Convert from a String to a LiteralValue
    fn from(s: String) -> Self {
        LiteralValue::String(s)
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
