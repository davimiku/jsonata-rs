//! Dyadic expression AST representation
//!

use std::fmt::{Display, Write};

use serde_json::Value;

use crate::{
    evaluate::{Context, EvaluationError, EvaluationResult},
    value::{number::JSONataNumber, JSONataValue},
};

use super::expr::Expression;
#[derive(PartialEq, Debug)]
enum CompareType {
    Equals,
    NotEquals,
    Greater,
    GreaterEquals,
    Less,
    LessEquals,
}

#[derive(PartialEq, Debug, Clone)]
pub enum DyadicOpType {
    Equals,
    NotEquals,
    Greater,
    GreaterEquals,
    Less,
    LessEquals,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

#[derive(PartialEq, Debug)]
pub struct CompareExpression {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
    compare_type: CompareType,
}

impl Display for DyadicOpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DyadicOpType::Equals => f.write_char('='),
            DyadicOpType::NotEquals => f.write_str("!="),
            DyadicOpType::Greater => f.write_char('>'),
            DyadicOpType::GreaterEquals => f.write_str(">="),
            DyadicOpType::Less => f.write_char('<'),
            DyadicOpType::LessEquals => f.write_str("<="),
            DyadicOpType::Add => f.write_char('+'),
            DyadicOpType::Sub => f.write_char('-'),
            DyadicOpType::Mul => f.write_char('*'),
            DyadicOpType::Div => f.write_char('/'),
            DyadicOpType::Rem => f.write_char('%'),
        }
    }
}

impl CompareExpression {
    /// Evaluate a Equals expression for whether the two expressions are equal
    pub(super) fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        let lhs = self.lhs.evaluate(context)?;
        let rhs = self.rhs.evaluate(context)?;
        match self.compare_type {
            CompareType::Equals => CompareExpression::equals(lhs, rhs),
            CompareType::NotEquals => CompareExpression::not_equals(lhs, rhs),
            CompareType::Greater => CompareExpression::greater(lhs, rhs),
            CompareType::GreaterEquals => CompareExpression::greater_equals(lhs, rhs),
            CompareType::Less => CompareExpression::less(lhs, rhs),
            CompareType::LessEquals => CompareExpression::less_equals(lhs, rhs),
        }
    }

    /// Tests for equality of the left-hand side (lhs) and right-hand side (rhs)
    ///
    /// Two None values in Rust are considered equal to each other, however,
    /// in the JSONata representation None values are considered not equal.
    ///
    /// Otherwise, deep (recursive) equality is tested. Object variants need
    /// not have the same ordering of keys to be considered equal.
    fn equals(lhs: Option<Value>, rhs: Option<Value>) -> EvaluationResult {
        Ok(Some(CompareExpression::equals_raw(lhs, rhs).into()))
    }

    /// Checks for equality and returns the result as a raw bool
    fn equals_raw(lhs: Option<Value>, rhs: Option<Value>) -> bool {
        match (lhs, rhs) {
            (None, None) => false,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(lhs_val), Some(rhs_val)) => {
                JSONataValue::from(lhs_val) == JSONataValue::from(rhs_val)
            }
        }
    }

    /// Tests for non-equality of the left-hand side (lhs) and right-hand side (rhs)
    ///
    /// In the JSONata representation is either side is None, then `not_equals` evaluates
    /// to false. In this regard, it **is not the inverse** of `equals`!
    ///
    /// FIXME: Decide if this is the behavior we want to implement, it's not documented in the JSONata docs.
    /// Intuitively, one would expect that equals is the opposite of not_equals, as
    /// currently implemented below.
    ///
    /// If both sides are Some, deep non-equality is tested.
    ///
    /// This operation cannot error at runtime TODO: I think?
    fn not_equals(lhs: Option<Value>, rhs: Option<Value>) -> EvaluationResult {
        Ok(Some((!CompareExpression::equals_raw(lhs, rhs)).into()))
    }

    /// Tests if the left-hand side is greater than the right-hand side.
    ///
    /// If either value is None, the return value is None.
    /// TODO: Would we rather have an error here?
    ///
    /// The lhs and rhs must both be numbers or both be strings, otherwise a runtime error
    /// is thrown.
    fn greater(lhs: Option<Value>, rhs: Option<Value>) -> EvaluationResult {
        match (lhs, rhs) {
            (None, None) => Ok(None),
            (None, Some(_)) => Ok(None),
            (Some(_), None) => Ok(None),
            (Some(l), Some(r)) => match (l, r) {
                (Value::Number(a), Value::Number(b)) => {
                    let j_num_lhs: JSONataNumber = a.into();
                    let j_num_rhs: JSONataNumber = b.into();
                    Ok(Some((j_num_lhs > j_num_rhs).into()))
                }
                (Value::String(a), Value::String(b)) => Ok(Some((a > b).into())),
                (_, _) => Err(EvaluationError::DyadicMustBeNumberOrString(
                    DyadicOpType::Greater,
                )),
            },
        }
    }

    /// Tests if the left-hand side is greater than or equal to right-hand side.
    ///
    /// If either value is None, the return value is None.
    /// TODO: Would we rather have an error here?
    ///
    /// The lhs and rhs must both be numbers or both be strings, otherwise a runtime error
    /// is thrown.
    fn greater_equals(lhs: Option<Value>, rhs: Option<Value>) -> EvaluationResult {
        match (lhs, rhs) {
            (None, None) => Ok(None),
            (None, Some(_)) => Ok(None),
            (Some(_), None) => Ok(None),
            (Some(l), Some(r)) => match (l, r) {
                (Value::Number(a), Value::Number(b)) => {
                    let j_num_lhs: JSONataNumber = a.into();
                    let j_num_rhs: JSONataNumber = b.into();
                    Ok(Some((j_num_lhs >= j_num_rhs).into()))
                }
                (Value::String(a), Value::String(b)) => Ok(Some((a >= b).into())),
                (_, _) => Err(EvaluationError::DyadicMustBeNumberOrString(
                    DyadicOpType::GreaterEquals,
                )),
            },
        }
    }

    /// Tests if the left-hand side is lesser than the right-hand side.
    ///
    /// If either value is None, the return value is None.
    /// TODO: Would we rather have an error here?
    ///
    /// The lhs and rhs must both be numbers or both be strings, otherwise a runtime error
    /// is thrown.
    fn less(lhs: Option<Value>, rhs: Option<Value>) -> EvaluationResult {
        match (lhs, rhs) {
            (None, None) => Ok(None),
            (None, Some(_)) => Ok(None),
            (Some(_), None) => Ok(None),
            (Some(l), Some(r)) => match (l, r) {
                (Value::Number(a), Value::Number(b)) => {
                    let j_num_lhs: JSONataNumber = a.into();
                    let j_num_rhs: JSONataNumber = b.into();
                    Ok(Some((j_num_lhs < j_num_rhs).into()))
                }
                (Value::String(a), Value::String(b)) => Ok(Some((a < b).into())),
                (_, _) => Err(EvaluationError::DyadicMustBeNumberOrString(
                    DyadicOpType::Less,
                )),
            },
        }
    }

    /// Tests if the left-hand side is lesser than or equal to right-hand side.
    ///
    /// If either value is None, the return value is None.
    /// TODO: Would we rather have an error here?
    ///
    /// The lhs and rhs must both be numbers or both be strings, otherwise a runtime error
    /// is thrown.
    fn less_equals(lhs: Option<Value>, rhs: Option<Value>) -> EvaluationResult {
        match (lhs, rhs) {
            (None, None) => Ok(None),
            (None, Some(_)) => Ok(None),
            (Some(_), None) => Ok(None),
            (Some(l), Some(r)) => match (l, r) {
                (Value::Number(a), Value::Number(b)) => {
                    let j_num_lhs: JSONataNumber = a.into();
                    let j_num_rhs: JSONataNumber = b.into();
                    Ok(Some((j_num_lhs <= j_num_rhs).into()))
                }
                (Value::String(a), Value::String(b)) => Ok(Some((a <= b).into())),
                (_, _) => Err(EvaluationError::DyadicMustBeNumberOrString(
                    DyadicOpType::LessEquals,
                )),
            },
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct InclusionExpression {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

impl InclusionExpression {
    /// Evaluate whether the lhs value is included in the rhs value
    pub(super) fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        let res = InclusionExpression::is_included(
            self.lhs.evaluate(context)?,
            self.rhs.evaluate(context)?,
        );
        Ok(Some(res))
    }

    fn is_included(lhs: Option<Value>, rhs: Option<Value>) -> Value {
        match (lhs, rhs) {
            (Some(l), Some(r)) => {
                Value::from(match l {
                    Value::Null => InclusionExpression::value_contains(l, r),
                    Value::Bool(_) => InclusionExpression::value_contains(l, r),
                    Value::Number(_) => InclusionExpression::value_contains(l, r),
                    Value::String(_) => InclusionExpression::value_contains(l, r),

                    // undocumented, but JSONata exerciser returns false for these
                    Value::Array(_) => false,
                    Value::Object(_) => false,
                })
            }
            (_, _) => Value::from(false),
        }
    }

    fn value_contains(needle: Value, haystack: Value) -> bool {
        match haystack {
            Value::Null => needle.is_null(),
            Value::Bool(_) => needle == haystack,
            Value::Number(_) => needle == haystack,
            Value::String(_) => needle == haystack,
            Value::Array(arr) => arr.contains(&needle),
            Value::Object(_) => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ConcatExpression {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

impl ConcatExpression {
    pub(super) fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        let left = self.lhs.evaluate(context)?;
        let right = self.rhs.evaluate(context)?;
        Ok(Some(Value::String(match (left, right) {
            (None, None) => "".into(),
            (None, Some(b)) => b.to_string(),
            (Some(a), None) => a.to_string(),
            (Some(a), Some(b)) => a.to_string() + &b.to_string(),
        })))
    }
}

/// TODO: Remove after checking why this was tested.
/// Value::Number(5.into()).is_i64()  is true!!

#[cfg(test)]
mod tests {

    use serde_json::json;

    use super::*;

    #[test]
    fn equals() {
        assert_eq!(
            Ok(Some(true.into())),
            CompareExpression::equals(Some(json!("test")), Some(json!("test"))),
        );
        assert_eq!(
            Ok(Some(true.into())),
            CompareExpression::equals(Some(json!(1)), Some(json!(1))),
        );
        assert_eq!(
            Ok(Some(true.into())),
            CompareExpression::equals(Some(json!(1.0)), Some(json!(1))),
        );
        assert_eq!(
            Ok(Some(true.into())),
            CompareExpression::equals(Some(json!(-2)), Some(json!(-2))),
        );
    }

    #[test]
    fn array_equals() {
        let lhs = Some(json!([1, 2, 3]));
        let rhs = Some(json!([1, 2, 3]));
        assert_eq!(Ok(Some(true.into())), CompareExpression::equals(lhs, rhs));
    }

    #[test]
    fn object_equals() {
        let lhs = Some(json!({ "key": "value", "key2": "value2"}));
        let rhs = Some(json!({ "key2": "value2", "key": "value"}));
        assert_eq!(Ok(Some(true.into())), CompareExpression::equals(lhs, rhs));
    }

    #[test]
    fn none_does_not_equal() {
        let lhs: Option<Value> = None;
        let rhs: Option<Value> = None;
        assert_eq!(Ok(Some(false.into())), CompareExpression::equals(lhs, rhs));
    }

    #[test]
    fn greater_num() {
        assert_eq!(
            Ok(Some(json!(true))),
            CompareExpression::greater(Some(json!(4)), Some(json!(3))),
        );
        assert_eq!(
            Ok(Some(json!(true))),
            CompareExpression::greater(Some(json!(4)), Some(json!(-3))),
        );
        assert_eq!(
            Ok(Some(json!(true))),
            CompareExpression::greater(Some(json!(4.2)), Some(json!(4.1))),
        );
        assert_eq!(
            Ok(Some(json!(false))),
            CompareExpression::greater(Some(json!(3)), Some(json!(4))),
        );
        assert_eq!(
            Ok(Some(json!(false))),
            CompareExpression::greater(Some(json!(-3)), Some(json!(4))),
        );
        assert_eq!(
            Ok(Some(json!(false))),
            CompareExpression::greater(Some(json!(4.1)), Some(json!(4.2))),
        );
    }

    #[test]
    fn greater_string() {
        assert!(true);
        // TODO:
    }

    #[test]
    fn greater_invalid_datatype() {
        assert!(true);
        // TODO:
    }
}
