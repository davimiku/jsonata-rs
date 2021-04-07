use serde_json::Value;

use crate::evaluate::{Context, EvaluationError};

use super::expression::{Expression, ExpressionType};

pub struct ConcatExpression<'a> {
    pub left: &'a Box<dyn Expression>,
    pub right: &'a Box<dyn Expression>,
}

impl Expression for ConcatExpression<'_> {
    fn etype(&self) -> ExpressionType {
        ExpressionType::StringConcat
    }

    fn evaluate(&self, context: &Context) -> Result<Option<Value>, EvaluationError> {
        let left_val = match self.left.evaluate(context)? {
            Some(Value::String(s)) => s,
            // TODO: Cast numbers, bool, null to string
            Some(val) => "".to_string(),
            // TODO: Cast objects/arrays to string (using serde::fmt::display?)
            //
            None => "".to_string(),
        };
        let right_val = match self.right.evaluate(context)? {
            Some(Value::String(s)) => s,
            // TODO: Cast numbers, bool, null to string
            Some(val) => "".to_string(),
            // TODO: Cast objects/arrays to string (using serde::fmt::display?)
            None => "".to_string(),
        };
        Ok(Some(Value::String(left_val + &right_val)))
    }
}
