mod function;
pub(crate) mod number;
mod traits;

use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::evaluate::error::EvaluationError;
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
pub(crate) enum JSONataValue {
    JSONValue(JSONValue),
    Function(JSONataFunction),
}

pub(crate) type JSONataVariables = HashMap<String, JSONataValue>;

impl JSONataValue {
    /// Generates an Option<JSONataValue> from a Option<serde_json::Value>
    pub fn from_opt_value(val: Option<serde_json::Value>) -> Option<JSONataValue> {
        val.map(|val| JSONataValue::JSONValue(val.into()))
    }

    /// Generates a JSONataValue that is a function from the given function
    /// and identifier.
    pub fn from_builtin<F, I>(func: F, ident: I) -> Self
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
}

impl fmt::Debug for JSONataValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JSONataValue::JSONValue(val) => write!(f, "{}", val),
            JSONataValue::Function(func) => write!(f, "{}", func),
        }
    }
}

impl fmt::Display for JSONataValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JSONataValue::JSONValue(val) => write!(f, "{}", val),
            JSONataValue::Function(func) => write!(f, "{}", func),
        }
    }
}

impl From<JSONValue> for JSONataValue {
    fn from(val: JSONValue) -> Self {
        JSONataValue::JSONValue(val)
    }
}

impl From<serde_json::Value> for JSONataValue {
    fn from(val: serde_json::Value) -> Self {
        JSONataValue::JSONValue(val.into())
    }
}

impl From<&serde_json::Value> for JSONataValue {
    fn from(val: &serde_json::Value) -> Self {
        JSONataValue::JSONValue(val.clone().into())
    }
}

impl From<Vec<serde_json::Value>> for JSONataValue {
    fn from(val: Vec<serde_json::Value>) -> Self {
        JSONataValue::JSONValue(val.into())
    }
}

impl From<bool> for JSONataValue {
    fn from(val: bool) -> Self {
        JSONataValue::JSONValue(val.into())
    }
}

impl From<&str> for JSONataValue {
    fn from(s: &str) -> Self {
        JSONataValue::JSONValue(s.into())
    }
}

impl From<String> for JSONataValue {
    fn from(s: String) -> Self {
        JSONataValue::JSONValue(s.into())
    }
}

impl From<usize> for JSONataValue {
    fn from(u: usize) -> Self {
        JSONataValue::JSONValue(u.into())
    }
}

impl From<i32> for JSONataValue {
    fn from(i: i32) -> Self {
        JSONataValue::JSONValue(i.into())
    }
}

impl From<f64> for JSONataValue {
    fn from(f: f64) -> Self {
        JSONataValue::JSONValue(f.into())
    }
}

impl From<JSONataNumber> for JSONataValue {
    fn from(num: JSONataNumber) -> Self {
        JSONataValue::JSONValue(num.into())
    }
}

impl PartialEq for JSONataValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (JSONataValue::JSONValue(s), JSONataValue::JSONValue(o)) => {
                match (s.0.clone(), o.0.clone()) {
                    (serde_json::Value::Null, serde_json::Value::Null) => true,
                    (serde_json::Value::Bool(a), serde_json::Value::Bool(b)) => a == b,
                    (serde_json::Value::String(a), serde_json::Value::String(b)) => a == b,

                    (serde_json::Value::Number(a), serde_json::Value::Number(b)) => {
                        JSONataNumber::from(a) == JSONataNumber::from(b)
                    }

                    (serde_json::Value::Array(a), serde_json::Value::Array(b)) => {
                        if a.len() != b.len() {
                            false
                        } else {
                            a.iter()
                                .zip(b)
                                .all(|(l, h)| JSONataValue::from(l) == JSONataValue::from(h))
                        }
                    }

                    (serde_json::Value::Object(a), serde_json::Value::Object(b)) => {
                        if a.len() != b.len() {
                            false
                        } else {
                            a.iter().all(|(key, a_val)| match b.get(key) {
                                Some(b_val) => {
                                    JSONataValue::from(a_val) == JSONataValue::from(b_val)
                                }
                                None => false,
                            })
                        }
                    }

                    (_, _) => false,
                }
            }

            // TODO: define equality behavior for functions
            (_, _) => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

fn parse_numbers<S>(
    lhs: &JSONataValue,
    rhs: &JSONataValue,
    op: S,
) -> Result<(JSONataNumber, JSONataNumber), EvaluationError>
where
    S: Into<String>,
{
    match (lhs, rhs) {
        (JSONataValue::JSONValue(lhs), JSONataValue::JSONValue(rhs)) => {
            match (lhs.0.clone(), rhs.0.clone()) {
                (serde_json::Value::Number(lhs), serde_json::Value::Number(rhs)) => {
                    return Ok((lhs.into(), rhs.into()))
                }
                (_, _) => {}
            }
        }
        (_, _) => {}
    }
    Err(EvaluationError::OperandsMustBeNumbers {
        op: op.into(),
        lhs: lhs.into(),
        rhs: rhs.into(),
    })
}

impl TryNumericOps for JSONataValue {
    fn try_add(self, rhs: Self) -> Result<Self, EvaluationError> {
        let (lhs, rhs) = parse_numbers(&self, &rhs, "+")?;
        Ok((lhs + rhs).into())
    }

    fn try_sub(self, rhs: Self) -> Result<Self, EvaluationError> {
        let (lhs, rhs) = parse_numbers(&self, &rhs, "-")?;
        Ok((lhs - rhs).into())
    }

    fn try_mul(self, rhs: Self) -> Result<Self, EvaluationError> {
        let (lhs, rhs) = parse_numbers(&self, &rhs, "*")?;
        Ok((lhs * rhs).into())
    }

    fn try_div(self, rhs: Self) -> Result<Self, EvaluationError> {
        let (lhs, rhs) = parse_numbers(&self, &rhs, "/")?;
        Ok((lhs / rhs).into())
    }

    fn try_rem(self, rhs: Self) -> Result<Self, EvaluationError> {
        let (lhs, rhs) = parse_numbers(&self, &rhs, "%")?;
        Ok((lhs % rhs).into())
    }
}

impl From<&JSONataValue> for String {
    fn from(val: &JSONataValue) -> Self {
        match val {
            JSONataValue::JSONValue(val) => val.to_string(),
            JSONataValue::Function(_) => String::from(""),
        }
    }
}

impl From<&JSONataValue> for bool {
    fn from(val: &JSONataValue) -> Self {
        match val {
            JSONataValue::JSONValue(val) => val.into(),
            JSONataValue::Function(_) => false,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct JSONValue(pub(crate) serde_json::Value);

impl JSONValue {
    /// Index into a JSON array or map. A string index can be used to access a value
    /// in a map, and a usize index can be used to access an element of an array.
    ///
    /// Returns None if the type of self does not match the type of the index,
    /// for example if the index is a string and self is an array or a number.
    /// Also returns None if the given key does not exist in the map or the given
    /// index is not within the bounds of the array.
    pub(crate) fn get<I: serde_json::value::Index>(&self, index: I) -> Option<JSONValue> {
        self.0.get(index).map(|v| v.clone().into())
    }
}

impl fmt::Display for JSONValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> From<T> for JSONValue
where
    T: Into<serde_json::Value>,
{
    fn from(val: T) -> Self {
        JSONValue(val.into())
    }
}

impl From<&JSONValue> for bool {
    fn from(val: &JSONValue) -> Self {
        match &val.0 {
            serde_json::Value::Null => false,
            serde_json::Value::Bool(b) => *b,
            serde_json::Value::Number(n) => {
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
            serde_json::Value::String(s) => s.is_empty(),
            serde_json::Value::Array(v) => {
                if v.is_empty() {
                    false
                } else {
                    v.iter().all(|val| (&JSONValue(val.clone())).into())
                }
            }
            serde_json::Value::Object(o) => o.is_empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::evaluate::error::EvaluationError;
    use crate::value::{traits::TryNumericOps, JSONataValue};

    #[test]
    fn eq() {
        let cases: Vec<(serde_json::Value, serde_json::Value)> = vec![
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
            assert!(JSONataValue::JSONValue(a.into()) == JSONataValue::JSONValue(b.into()));
        }
    }

    #[test]
    fn add() {
        // (lhs, rhs, expected)
        let ok_cases: Vec<(serde_json::Value, serde_json::Value, serde_json::Value)> = vec![
            (json!(1), json!(2), json!(3)),
            (json!(1.5), json!(2.5), json!(4.0)),
            (json!(-1.5), json!(2.5), json!(1.0)),
            (json!(1), json!(2.5), json!(3.5)),
            (json!(100), json!(-250), json!(-150)),
        ];
        for (lhs, rhs, expected) in ok_cases {
            let lhs = JSONataValue::JSONValue(lhs.into());
            let rhs = JSONataValue::JSONValue(rhs.into());
            assert_eq!(lhs.try_add(rhs).unwrap(), expected.into())
        }

        // (lhs, rhs)
        let err_cases: Vec<(serde_json::Value, serde_json::Value)> = vec![
            (json!("hello"), json!("world")),
            (json!(1), json!("1")),
            (json!("1"), json!(1)),
        ];
        for (lhs, rhs) in err_cases {
            let lhs_string = lhs.to_string();
            let rhs_string = rhs.to_string();
            let lhs = JSONataValue::JSONValue(lhs.into());
            let rhs = JSONataValue::JSONValue(rhs.into());
            assert_eq!(
                lhs.try_add(rhs).unwrap_err(),
                EvaluationError::OperandsMustBeNumbers {
                    op: '+'.to_string(),
                    lhs: lhs_string,
                    rhs: rhs_string,
                }
            )
        }
    }

    // TODO: tests for sub, mul, div, rem
}
