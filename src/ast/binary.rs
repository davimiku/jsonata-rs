//! Binary expression AST representation
//!

use serde_json::Value;

use crate::evaluate::{Context, EvaluationResult};

use super::expression::Expression;

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
    Lesser,
    LesserEquals,
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
            CompareType::Lesser => CompareExpression::lesser(lhs, rhs),
            CompareType::LesserEquals => CompareExpression::lesser_equals(lhs, rhs),
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
    ///
    /// The lhs and rhs must both be numbers or both be strings, otherwise a runtime error
    /// is thrown.
    fn greater(lhs: Option<Value>, rhs: Option<Value>) -> EvaluationResult {
        todo!()
    }

    /// Tests if the left-hand side is greater than or equal to right-hand side.
    ///
    /// If either value is None, the return value is None.
    ///
    /// The lhs and rhs must both be numbers or both be strings, otherwise a runtime error
    /// is thrown.
    fn greater_equals(lhs: Option<Value>, rhs: Option<Value>) -> EvaluationResult {
        todo!()
    }

    /// Tests if the left-hand side is lesser than the right-hand side.
    ///
    /// If either value is None, the return value is None.
    ///
    /// The lhs and rhs must both be numbers or both be strings, otherwise a runtime error
    /// is thrown.
    fn lesser(lhs: Option<Value>, rhs: Option<Value>) -> EvaluationResult {
        todo!()
    }

    /// Tests if the left-hand side is lesser than or equal to right-hand side.
    ///
    /// If either value is None, the return value is None.
    ///
    /// The lhs and rhs must both be numbers or both be strings, otherwise a runtime error
    /// is thrown.
    fn lesser_equals(lhs: Option<Value>, rhs: Option<Value>) -> EvaluationResult {
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
    fn equals() {
        let mut context = Context::default();
        let expr = CompareExpression {
            lhs: Box::new(LiteralExpression::from("test").into()),
            rhs: Box::new(LiteralExpression::from("test").into()),
            compare_type: CompareType::Equals,
        };
        assert_eq!(Ok(Some(Value::Bool(true))), expr.evaluate(&mut context));
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
}
