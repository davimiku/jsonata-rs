use serde_json::Value;

use crate::{evaluate::EvaluationResult, value::JSONataValue};

use super::BuiltIns;

#[cfg(test)]
mod tests;

impl BuiltIns {
    /// Casts the argument to a Boolean using the following rules:
    ///
    /// Boolean                     --> unchanged
    /// string: empty               --> false
    /// string: non-empty           --> true
    /// number: 0                   --> false
    /// number: non-zero            --> true
    /// null                        --> false
    /// array:
    /// - empty                     --> false
    /// - >= 1 member casts to true --> true
    /// - all members cast to false --> false
    /// object: empty               --> false
    /// object: non-empty           --> true
    /// function                    --> false
    ///
    /// **Signature**: `$boolean(arg)`
    pub fn boolean(arg: &JSONataValue) -> EvaluationResult {
        Ok(Some(match arg {
            JSONataValue::JSONValue(val) => BuiltIns::boolean_coerce(val).into(),
            JSONataValue::Function(_) => false.into(),
        }))
    }

    fn boolean_coerce(val: &Value) -> bool {
        match val {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Number(n) => {
                if n.is_u64() {
                    n.as_u64() == Some(0_u64)
                } else if n.is_i64() {
                    n.as_i64() == Some(0_i64)
                } else if n.is_f64() {
                    n.as_f64() == Some(0.0)
                } else {
                    false
                }
            }
            Value::String(s) => s.is_empty(),
            Value::Array(v) => {
                if v.is_empty() {
                    false
                } else {
                    v.iter().all(|val| BuiltIns::boolean_coerce(val))
                }
            }
            Value::Object(o) => o.is_empty(),
        }
    }

    /// Returns Boolean NOT on the argument. arg is first cast to a boolean
    ///
    /// **Signature**: `$not(arg)`
    pub fn not(arg: &JSONataValue) -> EvaluationResult {
        Ok(Some(match arg {
            JSONataValue::JSONValue(val) => (!BuiltIns::boolean_coerce(val)).into(),
            JSONataValue::Function(_) => true.into(),
        }))
    }

    /// Returns Boolean true if the arg expression evaluates to a value,
    /// or false if the expression does not match anything
    /// (e.g. a path to a non-existent field reference).
    ///
    /// **Signature**: `$exists(arg)`
    pub fn exists(arg: &Option<JSONataValue>) -> EvaluationResult {
        Ok(Some(match arg {
            Some(_) => true.into(),
            None => false.into(),
        }))
    }
}
