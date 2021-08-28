//! Dyadic expression AST representation
//!

use std::{
    convert::TryInto,
    fmt::{Display, Write},
};

use serde_json::Value;

use crate::{
    evaluate::{Context, EvaluationError, EvaluationResult},
    value::{number::JSONataNumber, JSONataValue},
};

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

impl From<CompareOpType> for DyadicOpType {
    fn from(c: CompareOpType) -> Self {
        DyadicOpType::Compare(c)
    }
}

impl From<ArithmeticOpType> for DyadicOpType {
    fn from(n: ArithmeticOpType) -> Self {
        DyadicOpType::Arithmetic(n)
    }
}

#[derive(PartialEq, Debug)]
pub enum ArithmeticOpType {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

impl From<&str> for ArithmeticOpType {
    fn from(s: &str) -> Self {
        match s {
            "+" => ArithmeticOpType::Add,
            "-" => ArithmeticOpType::Sub,
            "*" => ArithmeticOpType::Mul,
            "/" => ArithmeticOpType::Div,
            "%" => ArithmeticOpType::Rem,
            _ => unreachable!(),
        }
    }
}

impl Display for ArithmeticOpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArithmeticOpType::Add => f.write_char('+'),
            ArithmeticOpType::Sub => f.write_char('-'),
            ArithmeticOpType::Mul => f.write_char('*'),
            ArithmeticOpType::Div => f.write_char('/'),
            ArithmeticOpType::Rem => f.write_char('%'),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ArithmeticExpression {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
    pub arithmetic_type: ArithmeticOpType,
}

impl ArithmeticExpression {
    /// Evaluate a arithmetic expression
    pub(super) fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        let lhs = self.lhs.evaluate(context)?;
        let rhs = self.rhs.evaluate(context)?;
        match (lhs, rhs) {
            (Some(lhs), Some(rhs)) => ArithmeticExpression::jsonata_value_arithmetic(
                lhs.try_into()?,
                rhs.try_into()?,
                &self.arithmetic_type,
            ),
            (_, _) => Ok(None),
        }
    }

    fn jsonata_value_arithmetic(
        lhs: JSONataNumber,
        rhs: JSONataNumber,
        op: &ArithmeticOpType,
    ) -> EvaluationResult {
        Ok(Some(
            match op {
                ArithmeticOpType::Add => lhs + rhs,
                ArithmeticOpType::Sub => lhs - rhs,
                ArithmeticOpType::Mul => lhs * rhs,
                ArithmeticOpType::Div => lhs / rhs,
                ArithmeticOpType::Rem => lhs % rhs,
            }
            .into(),
        ))
    }
}

#[derive(PartialEq, Debug)]
pub enum CompareOpType {
    Equals,
    NotEquals,
    Greater,
    GreaterEquals,
    Less,
    LessEquals,
}

impl From<&str> for CompareOpType {
    fn from(s: &str) -> Self {
        match s {
            "=" => CompareOpType::Equals,
            "!=" => CompareOpType::NotEquals,
            ">" => CompareOpType::Greater,
            ">=" => CompareOpType::GreaterEquals,
            "<" => CompareOpType::Less,
            "<=" => CompareOpType::LessEquals,
            _ => unreachable!(),
        }
    }
}

impl Display for CompareOpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompareOpType::Equals => f.write_char('='),
            CompareOpType::NotEquals => f.write_str("!="),
            CompareOpType::Greater => f.write_char('>'),
            CompareOpType::GreaterEquals => f.write_str(">="),
            CompareOpType::Less => f.write_char('<'),
            CompareOpType::LessEquals => f.write_str("<="),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct CompareExpression {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
    pub compare_type: CompareOpType,
}

impl CompareExpression {
    /// Evaluate a comparison expression
    pub(super) fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        let lhs = self.lhs.evaluate(context)?;
        let rhs = self.rhs.evaluate(context)?;
        match self.compare_type {
            CompareOpType::Equals => CompareExpression::equals(lhs, rhs),
            CompareOpType::NotEquals => CompareExpression::not_equals(lhs, rhs),
            CompareOpType::Greater => CompareExpression::greater(lhs, rhs),
            CompareOpType::GreaterEquals => CompareExpression::greater_equals(lhs, rhs),
            CompareOpType::Less => CompareExpression::less(lhs, rhs),
            CompareOpType::LessEquals => CompareExpression::less_equals(lhs, rhs),
        }
    }

    /// Tests for equality of the left-hand side (lhs) and right-hand side (rhs)
    ///
    /// Two None values in Rust are considered equal to each other, however,
    /// in the JSONata representation None values are considered not equal.
    ///
    /// Otherwise, deep (recursive) equality is tested. Object variants need
    /// not have the same ordering of keys to be considered equal.
    fn equals(lhs: Option<JSONataValue>, rhs: Option<JSONataValue>) -> EvaluationResult {
        Ok(Some(CompareExpression::equals_raw(lhs, rhs).into()))
    }

    /// Checks for equality and returns the result as a raw bool
    fn equals_raw(lhs: Option<JSONataValue>, rhs: Option<JSONataValue>) -> bool {
        match (lhs, rhs) {
            (None, None) => false,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(lhs), Some(rhs)) => lhs == rhs,
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
    /// Potentially this is "documented" in a test case?
    ///
    /// If both sides are Some, deep non-equality is tested.
    fn not_equals(lhs: Option<JSONataValue>, rhs: Option<JSONataValue>) -> EvaluationResult {
        Ok(Some((!CompareExpression::equals_raw(lhs, rhs)).into()))
    }

    /// Tests if the left-hand side is greater than the right-hand side.
    ///
    /// If either value is None, the return value is None.
    /// TODO: Would we rather have an error here?
    ///
    /// The lhs and rhs must both be numbers or both be strings, otherwise a runtime error
    /// is thrown.
    fn greater(lhs: Option<JSONataValue>, rhs: Option<JSONataValue>) -> EvaluationResult {
        match CompareExpression::unwrap_for_compare(lhs, rhs, CompareOpType::LessEquals)? {
            Some((left_val, right_val)) => match (left_val, right_val) {
                (NumberOrString::Number(a), NumberOrString::Number(b)) => Ok(Some((a > b).into())),
                (NumberOrString::String(a), NumberOrString::String(b)) => Ok(Some((a > b).into())),
                (_, _) => unreachable!(),
            },
            None => Ok(None),
        }
    }

    /// Tests if the left-hand side is greater than or equal to right-hand side.
    ///
    /// If either value is None, the return value is None.
    /// TODO: Would we rather have an error here?
    ///
    /// The lhs and rhs must both be numbers or both be strings, otherwise a runtime error
    /// is thrown.
    fn greater_equals(lhs: Option<JSONataValue>, rhs: Option<JSONataValue>) -> EvaluationResult {
        match CompareExpression::unwrap_for_compare(lhs, rhs, CompareOpType::LessEquals)? {
            Some((left_val, right_val)) => match (left_val, right_val) {
                (NumberOrString::Number(a), NumberOrString::Number(b)) => Ok(Some((a >= b).into())),
                (NumberOrString::String(a), NumberOrString::String(b)) => Ok(Some((a >= b).into())),
                (_, _) => unreachable!(),
            },
            None => Ok(None),
        }
    }

    /// Tests if the left-hand side is lesser than the right-hand side.
    ///
    /// If either value is None, the return value is None.
    /// TODO: This is the behavior in the JSONata Exerciser, but it's not documented.
    /// Would we rather have an error here?
    ///
    /// The lhs and rhs must both be numbers or both be strings, otherwise a runtime error
    /// is thrown.
    fn less(lhs: Option<JSONataValue>, rhs: Option<JSONataValue>) -> EvaluationResult {
        match CompareExpression::unwrap_for_compare(lhs, rhs, CompareOpType::LessEquals)? {
            Some((left_val, right_val)) => match (left_val, right_val) {
                (NumberOrString::Number(a), NumberOrString::Number(b)) => Ok(Some((a < b).into())),
                (NumberOrString::String(a), NumberOrString::String(b)) => Ok(Some((a < b).into())),
                (_, _) => unreachable!(),
            },
            None => Ok(None),
        }
    }

    /// Tests if the left-hand side is lesser than or equal to right-hand side.
    ///
    /// If either value is None, the return value is None.
    /// TODO: This is the behavior in the JSONata Exerciser, but it's not documented.
    /// Would we rather have an error here?
    ///
    /// The lhs and rhs must both be numbers or both be strings, otherwise a runtime error
    /// is thrown.
    fn less_equals(lhs: Option<JSONataValue>, rhs: Option<JSONataValue>) -> EvaluationResult {
        match CompareExpression::unwrap_for_compare(lhs, rhs, CompareOpType::LessEquals)? {
            Some((left_val, right_val)) => match (left_val, right_val) {
                (NumberOrString::Number(a), NumberOrString::Number(b)) => Ok(Some((a <= b).into())),
                (NumberOrString::String(a), NumberOrString::String(b)) => Ok(Some((a <= b).into())),
                (_, _) => unreachable!(),
            },
            None => Ok(None),
        }
    }

    /// Performs unwrapping values that will be passed into comparison functions
    ///
    /// Unwraps complex values to return Ok if the unwrapped values can be
    /// compared, and Err if the unwrapped values cannot be compared.
    fn unwrap_for_compare(
        lhs: Option<JSONataValue>,
        rhs: Option<JSONataValue>,
        op_type: CompareOpType,
    ) -> Result<Option<(NumberOrString, NumberOrString)>, EvaluationError> {
        match (lhs, rhs) {
            (None, None) => Ok(None),
            (None, Some(val)) => match val {
                JSONataValue::Value(_) => Ok(None),
                JSONataValue::Function(_) => {
                    Err(EvaluationError::DyadicMustBeNumberOrString(op_type.into()))
                }
            },
            (Some(val), None) => match val {
                JSONataValue::Value(_) => Ok(None),
                JSONataValue::Function(_) => {
                    Err(EvaluationError::DyadicMustBeNumberOrString(op_type.into()))
                }
            },
            (Some(l), Some(r)) => match (l, r) {
                (JSONataValue::Value(left_val), JSONataValue::Value(right_val)) => {
                    match (left_val, right_val) {
                        (Value::Number(l), Value::Number(r)) => Ok(Some((l.into(), r.into()))),
                        (Value::String(l), Value::String(r)) => Ok(Some((l.into(), r.into()))),
                        (_, _) => Err(EvaluationError::DyadicMustBeNumberOrString(op_type.into())),
                    }
                }
                (_, _) => Err(EvaluationError::DyadicMustBeNumberOrString(op_type.into())),
            },
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
pub struct ConcatExpression {
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

#[cfg(test)]
mod tests {

    use serde_json::json;

    use crate::tests::make_val;

    use super::*;

    #[test]
    fn equals() {
        assert_eq!(
            Ok(Some(true.into())),
            CompareExpression::equals(Some(make_val("test")), Some(json!("test").into())),
        );
        assert_eq!(
            Ok(Some(true.into())),
            CompareExpression::equals(Some(make_val(1)), Some(make_val(1))),
        );
        assert_eq!(
            Ok(Some(true.into())),
            CompareExpression::equals(Some(make_val(1.0)), Some(make_val(1))),
        );
        assert_eq!(
            Ok(Some(true.into())),
            CompareExpression::equals(Some(make_val(-2)), Some(make_val(-2))),
        );
    }

    #[test]
    fn array_equals() {
        let lhs = Some(json!([1, 2, 3]).into());
        let rhs = Some(json!([1, 2, 3]).into());
        assert_eq!(Ok(Some(true.into())), CompareExpression::equals(lhs, rhs));
    }

    #[test]
    fn object_equals() {
        let lhs = Some(json!({ "key": "value", "key2": "value2"}).into());
        let rhs = Some(json!({ "key2": "value2", "key": "value"}).into());
        assert_eq!(Ok(Some(true.into())), CompareExpression::equals(lhs, rhs));
    }

    #[test]
    fn none_does_not_equal() {
        let lhs = JSONataValue::from_opt_value(None);
        let rhs = JSONataValue::from_opt_value(None);
        assert_eq!(Ok(Some(false.into())), CompareExpression::equals(lhs, rhs));
    }

    #[test]
    fn greater_num() {
        assert_eq!(
            Ok(Some(make_val(true))),
            CompareExpression::greater(Some(make_val(4)), Some(make_val(3))),
        );
        assert_eq!(
            Ok(Some(make_val(true))),
            CompareExpression::greater(Some(make_val(4)), Some(make_val(-3))),
        );
        assert_eq!(
            Ok(Some(make_val(true))),
            CompareExpression::greater(Some(make_val(4.2)), Some(make_val(4.1))),
        );
        assert_eq!(
            Ok(Some(make_val(false))),
            CompareExpression::greater(Some(make_val(3)), Some(make_val(4))),
        );
        assert_eq!(
            Ok(Some(make_val(false))),
            CompareExpression::greater(Some(make_val(-3)), Some(make_val(4))),
        );
        assert_eq!(
            Ok(Some(make_val(false))),
            CompareExpression::greater(Some(make_val(4.1)), Some(make_val(4.2))),
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
