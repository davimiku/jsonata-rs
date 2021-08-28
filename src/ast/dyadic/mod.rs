//! Dyadic expression AST representation
//!

pub(crate) mod arithmetic;
pub(crate) mod compare;

use std::fmt::Display;

use serde_json::Value;

use crate::{
    evaluate::{Context, EvaluationResult},
    value::{number::JSONataNumber, JSONataValue},
};

use self::{arithmetic::ArithmeticOpType, compare::CompareOpType};

use super::expr::Expression;

#[derive(PartialEq, Debug)]
pub enum DyadicOpType {
    Compare(CompareOpType),
    Arithmetic(ArithmeticOpType),
}

impl Display for DyadicOpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DyadicOpType::Compare(c) => f.write_fmt(format_args!("{}", c)),
            DyadicOpType::Arithmetic(n) => f.write_fmt(format_args!("{}", n)),
        }
    }
}

enum NumberOrString {
    Number(JSONataNumber),
    String(String),
}

impl From<serde_json::Number> for NumberOrString {
    fn from(n: serde_json::Number) -> Self {
        NumberOrString::Number(n.into())
    }
}

impl From<String> for NumberOrString {
    fn from(s: String) -> Self {
        NumberOrString::String(s)
    }
}

#[derive(PartialEq, Debug)]
pub(crate) struct InclusionExpression {
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

    fn is_included(lhs: Option<JSONataValue>, rhs: Option<JSONataValue>) -> JSONataValue {
        match (lhs, rhs) {
            (Some(lhs), Some(rhs)) => {
                match (lhs, rhs) {
                    (JSONataValue::Value(_), JSONataValue::Value(_)) => todo!(),
                    (JSONataValue::Value(_), JSONataValue::Function(_)) => false.into(),
                    (JSONataValue::Function(_), JSONataValue::Value(_)) => todo!(),
                    (JSONataValue::Function(a), JSONataValue::Function(b)) => {
                        // ex. `$max in $max` is true because the rhs is coerced to array
                        // we can skip the array coercion and check equality directly
                        (a == b).into()
                    }
                }
                // Value::from(match l {
                //     Value::Null => InclusionExpression::value_contains(l, r),
                //     Value::Bool(_) => InclusionExpression::value_contains(l, r),
                //     Value::Number(_) => InclusionExpression::value_contains(l, r),
                //     Value::String(_) => InclusionExpression::value_contains(l, r),

                //     // undocumented, but JSONata exerciser returns false for these
                //     Value::Array(_) => false,
                //     Value::Object(_) => false,
                // })
            }
            (_, _) => false.into(),
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
pub(crate) struct ConcatExpression {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

impl ConcatExpression {
    pub(super) fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        let left = self.lhs.evaluate(context)?;
        let right = self.rhs.evaluate(context)?;
        Ok(Some(
            match (left, right) {
                (None, None) => "".into(),
                (Some(a), None) => format!("{}", a),
                (None, Some(b)) => format!("{}", b),
                (Some(a), Some(b)) => format!("{}{}", a, b),
            }
            .into(),
        ))
    }
}
