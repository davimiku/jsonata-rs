pub(crate) mod concat;
pub(crate) mod expression;
pub(crate) mod literal;
pub(crate) mod path;

use crate::evaluate::{Context, EvaluationResult};
use serde_json::Value;

use self::expression::Expression;

pub(crate) struct Program {
    /// Contains the evaluation context of the program
    ///
    /// This tracks declared variables and functions, and
    /// TODO the current location for reporting of runtime errors
    ///
    pub context: Context,

    /// Expressions in the JSONata program
    ///
    /// Example:
    /// ```
    /// (
    ///    $currency = Product.Price.Currency;
    ///    $amount = Product.Price.Amount;
    ///    $amount & $currency
    /// )
    /// ```
    ///
    /// In this case, there are three expressions, with the final expression
    /// representing the return value.
    ///
    /// Many JSONata programs will only have a single expression.
    pub expressions: Vec<Expression>,
}

impl Program {
    pub fn evaluate(&mut self, data: Value) -> EvaluationResult {
        self.set_data(data);

        let mut result = None;
        for expr in &self.expressions {
            result = expr.evaluate(&mut self.context)?;
        }
        Ok(result)
    }

    fn set_data(&mut self, data: Value) {
        self.context.set_data(data)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Location {
    line: u32,
    col: u32,
    char: u64,
}
