//! Binary expression AST representation
//!

use serde_json::Value;

use crate::evaluate::{Context, EvaluationResult};

use super::expression::Expression;

#[derive(PartialEq, Debug, Clone)]
pub struct EqualsExpression {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

impl EqualsExpression {
    /// Evaluate a Equals expression for whether the two expressions are equal
    pub fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        let is_equal =
            EqualsExpression::equals(self.lhs.evaluate(context)?, self.rhs.evaluate(context)?);
        Ok(Some(Value::Bool(is_equal)))
    }

    /// Tests for equality of the left-hand side (lhs) and right-hand side (rhs)
    ///
    /// Two None values in Rust are considered equal to each other, however,
    /// in this representation None values are considered not equal.
    ///
    /// This is important for comparing two path expressions that do not
    /// find any value in the JSON, we cannot say that they are equal.
    ///
    /// Otherwise,
    fn equals(lhs: Option<Value>, rhs: Option<Value>) -> bool {
        if lhs.is_none() || rhs.is_none() {
            return false;
        }
        lhs.unwrap() == rhs.unwrap()
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
        let expr = EqualsExpression {
            lhs: Box::new(LiteralExpression::from("test").into()),
            rhs: Box::new(LiteralExpression::from("test").into()),
        };
        assert_eq!(Ok(Some(Value::Bool(true))), expr.evaluate(&mut context));
    }

    #[test]
    fn array_equals() {
        let lhs = Some(json!([1, 2, 3]));
        let rhs = Some(json!([1, 2, 3]));
        assert!(EqualsExpression::equals(lhs, rhs));
    }

    #[test]
    fn object_equals() {
        let lhs = Some(json!({ "key": "value", "key2": "value2"}));
        let rhs = Some(json!({ "key2": "value2", "key": "value"}));
        assert!(EqualsExpression::equals(lhs, rhs));
    }

    #[test]
    fn none_does_not_equal() {
        let lhs: Option<Value> = None;
        let rhs: Option<Value> = None;
        assert!(!EqualsExpression::equals(lhs, rhs));
    }
}
