use serde_json::Value;

use crate::evaluate::{Context, EvaluationError, EvaluationResult};

use super::expression::Expression;

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

// impl LegacyExpressionTrait for ConcatExpression<'_> {
//     fn etype(&self) -> ExpressionType {
//         ExpressionType::StringConcat
//     }

//     fn evaluate(&self, context: &Context) -> Result<Option<Value>, EvaluationError> {
//         let left_val = match self.left.evaluate(context)? {
//             Some(Value::String(s)) => s,
//             // TODO: Cast numbers, bool, null to string
//             Some(val) => "".to_string(),
//             // TODO: Cast objects/arrays to string (using serde::fmt::display?)
//             //
//             None => "".to_string(),
//         };
//         let right_val = match self.right.evaluate(context)? {
//             Some(Value::String(s)) => s,
//             // TODO: Cast numbers, bool, null to string
//             Some(val) => "".to_string(),
//             // TODO: Cast objects/arrays to string (using serde::fmt::display?)
//             None => "".to_string(),
//         };
//         Ok(Some(Value::String(left_val + &right_val)))
//     }
// }
