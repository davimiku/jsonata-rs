//! Binary expression AST representation
//!

use serde_json::{Number, Value};

use crate::evaluate::{Context, EvaluationError, EvaluationResult};

use super::{expression::Expression, number::JSONataNumber, value::JSONataValue};

#[derive(PartialEq, Debug, Clone)]
pub struct CompareExpression {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
    compare_type: CompareType,
}

#[derive(PartialEq, Debug, Clone)]
enum CompareType {
    Equals,
    NotEquals,
    Greater,
    GreaterEquals,
    Less,
    LessEquals,
}

impl CompareExpression {
    /// Evaluate a Equals expression for whether the two expressions are equal
    pub fn evaluate(&self, context: &mut Context) -> EvaluationResult {
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
    /// Otherwise, deep equality is tested.
    ///
    /// This operation cannot error at runtime TODO: I think?
    /// FIXME: use JSONataNumber for numeric comparisons
    fn equals(lhs: Option<Value>, rhs: Option<Value>) -> EvaluationResult {
        if lhs.is_none() || rhs.is_none() {
            Ok(Some(false.into()))
        } else {
            Ok(Some((lhs.unwrap() == rhs.unwrap()).into()))
        }
    }

    /// Tests for non-equality of the left-hand side (lhs) and right-hand side (rhs)
    ///
    /// In the JSONata representation is either side is None, then `not_equals` evaluates
    /// to false. In this regard, it is not the inverse of `equals`.
    /// TODO: Decide if this is the behavior we want to implement, it's not documented in the JSONata docs.
    ///
    /// If both sides are Some, deep non-equality is tested.
    ///
    /// This operation cannot error at runtime TODO: I think?
    fn not_equals(lhs: Option<Value>, rhs: Option<Value>) -> EvaluationResult {
        if lhs.is_none() || rhs.is_none() {
            Ok(Some(false.into()))
        } else {
            Ok(Some((lhs.unwrap() == rhs.unwrap()).into()))
        }
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
                (_, _) => Err(EvaluationError::BinaryInvalidDataType(">".into())),
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
        todo!()
    }

    /// Tests if the left-hand side is lesser than the right-hand side.
    ///
    /// If either value is None, the return value is None.
    /// TODO: Would we rather have an error here?
    ///
    /// The lhs and rhs must both be numbers or both be strings, otherwise a runtime error
    /// is thrown.
    fn less(lhs: Option<Value>, rhs: Option<Value>) -> EvaluationResult {
        todo!()
    }

    /// Tests if the left-hand side is lesser than or equal to right-hand side.
    ///
    /// If either value is None, the return value is None.
    /// TODO: Would we rather have an error here?
    ///
    /// The lhs and rhs must both be numbers or both be strings, otherwise a runtime error
    /// is thrown.
    fn less_equals(lhs: Option<Value>, rhs: Option<Value>) -> EvaluationResult {
        todo!()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct InclusionExpression {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

impl InclusionExpression {
    /// Evaluate whether the lhs value is included in the rhs value
    pub fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        let res = InclusionExpression::is_included(
            self.lhs.evaluate(context)?,
            self.rhs.evaluate(context)?,
        );
        Ok(Some(Value::Bool(res)))
    }

    fn is_included(lhs: Option<Value>, rhs: Option<Value>) -> bool {
        if lhs.is_none() || rhs.is_none() {
            return false;
        }
        let l = lhs.unwrap();
        let r = rhs.unwrap();
        match l {
            Value::Null => InclusionExpression::value_contains(l, r),
            Value::Bool(_) => InclusionExpression::value_contains(l, r),
            Value::Number(_) => InclusionExpression::value_contains(l, r),
            Value::String(_) => InclusionExpression::value_contains(l, r),

            // undocumented, but JSONata exerciser returns false for these
            Value::Array(_) => false,
            Value::Object(_) => false,
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

#[cfg(test)]
mod tests {

    use serde_json::json;

    use crate::ast::literal::LiteralExpression;

    use super::*;

    #[test]
    fn test() {
        let a = Value::Number(5.into());
        println!("{}", a.is_i64());
        println!("{}", a.is_u64());
        println!("{}", a.is_number());
    }

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
