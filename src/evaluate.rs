//! Execution of the Abstract Syntax Tree (AST)

use std::{collections::HashMap, fmt::Display};

use serde_json::Value;

use crate::ast::dyadic::DyadicOpType;

pub type EvaluationResult = Result<Option<Value>, EvaluationError>;

#[derive(PartialEq, Clone, Debug)]
pub enum EvaluationError {
    /// The values '{}' and '{}' on either side of operator '{}' must be of the same data type
    DyadicInconsistentDataType(Value, Value, DyadicOpType),

    /// The expressions on either side of operator '{}' must evaluate to numeric values
    DyadicMustBeNumber(DyadicOpType),

    /// The expressions on either side of operator '{}' must evaluate to numeric or string values
    DyadicMustBeNumberOrString(DyadicOpType),

    /// Function '{}': argument '{}' must be '{}'
    FunctionInvalidArgument(String, usize, String),

    /// Function '{}': requires '{}' arguments, '{}' were provided
    FunctionIncorrectNumArguments(String, usize, usize),
}

impl Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvaluationError::DyadicInconsistentDataType(val1, val2, op) => {
                f.write_fmt(format_args!("The values '{}' and '{}' on either side of operator '{}' must be of the same data type", val1, val2, op))
            }
            EvaluationError::DyadicMustBeNumber(op) => {
                f.write_fmt(format_args!("The expressions on either side of operator '{}' must evaluate to numeric values", op))
            }
            EvaluationError::DyadicMustBeNumberOrString(op) => {
                f.write_fmt(format_args!("The expressions on either side of operator '{}' must evaluate to numeric or string values", op))
            }
            EvaluationError::FunctionInvalidArgument(func_name, arg_num, expected) => {
                f.write_fmt(format_args!("Function '{}': argument '{}' must be '{}'", func_name, arg_num, expected))
            }
            EvaluationError::FunctionIncorrectNumArguments(func_name, num_expected, num_actual) => {
                f.write_fmt(format_args!("Function '{}': requires '{}' arguments, '{}' were provided", func_name, num_expected, num_actual))
            }
        }
    }
}

#[derive(Debug)]
pub struct Context {
    data: Value,

    variables: HashMap<String, Option<Value>>,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            data: Value::Null,
            variables: HashMap::new(),
        }
    }
}

impl Context {
    pub fn from_data(data: Value) -> Context {
        Context {
            data,
            variables: HashMap::new(),
        }
    }

    pub fn data(&self) -> &Value {
        &self.data
    }

    pub fn set_var(&mut self, var_name: String, value: Option<Value>) {
        self.variables.insert(var_name, value);
    }

    pub fn set_data(&mut self, data: Value) {
        self.data = data
    }
    // pub(crate) fn create_function<P, B>(
    //     &mut self,
    //     params: P,
    //     body: B
    // ) -> Result<(), ()>
    // where
    //     P: Into<Box<[FormalParameter]>>,
    //     B: Into<StatementList>,
    //     {}
}
