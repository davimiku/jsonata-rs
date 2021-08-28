pub(crate) mod dyadic;
pub(crate) mod expr;
pub(crate) mod literal;
pub(crate) mod path;

use crate::evaluate::{Context, EvaluationResult};
use serde_json::Value;

use self::expr::Expression;

pub struct Program<'a> {
    /// Contains the evaluation context of the program
    ///
    /// This tracks declared variables and functions, and
    /// TODO the current location for reporting of runtime errors
    ///
    pub(crate) context: Context<'a>,

    /// Top-level expression of the JSONata program
    ///
    /// Expressions can (and usually do) have child
    /// expressions.
    pub(crate) expression: Expression,
}

impl<'a> Program<'a> {
    /// Create a new JSONata program with the provided
    /// top-level expression
    ///
    /// Creates a default Context and a program that
    /// is ready to be evaluated.
    pub(crate) fn new(expression: Expression) -> Self {
        Program {
            context: Context::default(),
            expression,
        }
    }

    /// Evaluate the JSONata program with the given data.
    ///
    /// The data is in Value format as parsed by serde-json.
    pub fn evaluate(&mut self, data: &'a Value) -> EvaluationResult {
        self.set_data(data);

        Ok(self.expression.evaluate(&mut self.context)?)
    }

    /// Sets the JSON data into the program's internal state
    fn set_data(&mut self, data: &'a Value) {
        self.context.set_data(data)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Location {
    line: u32,
    col: u32,
    char: u64,
}
