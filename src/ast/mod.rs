pub(crate) mod concat;
pub(crate) mod expression;
pub(crate) mod literal;
pub(crate) mod path;

use crate::evaluate::{Context, Evaluatable, EvaluationResult};
use serde_json::Value;

pub struct Program {
    /// Contains the evaluation context of the program
    ///
    /// This tracks declared variables and functions, and
    /// TODO the current location for reporting of runtime errors
    ///
    pub context: Context,

    // charRange: (u32, u32)  // from 0 to number of characters (do we need this?)
    /// Declarations of variables and functions that
    /// occur before the return expression.
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
    /// In the example above, the variable
    /// declarations for `$currency` and `$amount` are stored in this Vec.
    ///
    /// These declarations modify the internal state of the execution context while
    /// it is running but do not return a value.
    pub declarations: Vec<Declaration>,

    /// The final expression in the JSONata program
    /// and represents the returned value of the program.
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
    /// In this case, the `$amount & $currency` is the return expression.
    /// This particular expression is a Concatenation Binary Expression with
    /// a 'left' expression of the Path expression stored in `$amount` and a
    /// 'right' expression of the Path expression in `$currency`.
    pub return_expression: Option<Box<dyn Evaluatable>>,
}

impl Default for Program {
    fn default() -> Self {
        Program {
            declarations: Default::default(),
            return_expression: None,
            context: Default::default(),
        }
    }
}

impl Program {
    pub fn evaluate(&mut self, data: Value) -> EvaluationResult {
        self.set_data(data);
        //
        // TODO: Iterate through declarations to mutate context
        //
        if let Some(expr) = &self.return_expression {
            expr.evaluate(&mut self.context)
        } else {
            Ok(None)
        }
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

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Declaration {
    Variable,
    Function,
}

// #[derive(PartialEq, Eq, Debug, Clone)]
// pub enum BinaryExpression {
//     Numeric,
//     Equality,
//     Comparison,
//     StringConcat,
//     Range,
//     Includes,
// }
