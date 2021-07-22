//! Execution of the Abstract Syntax Tree (AST)

use std::{collections::HashMap, fmt::Display};

use serde_json::Value;

use crate::{ast::dyadic::DyadicOpType, builtins::BuiltIns, value::JSONataValue};

pub type EvaluationResult = Result<Option<JSONataValue>, EvaluationError>;

#[derive(PartialEq, Debug)]
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

    /// JSONata Functions cannot be converted to other value types
    FunctionCannotConvertToValue(String),
}

impl EvaluationError {
    pub fn function_incorrect_num_arguments<S>(name: S, expected: usize, actual: usize) -> Self
    where
        S: Into<String>,
    {
        EvaluationError::FunctionIncorrectNumArguments(name.into(), expected, actual)
    }

    pub fn function_invalid_argument<S>(name: S, arg_num: usize, expected_type: S) -> Self
    where
        S: Into<String>,
    {
        EvaluationError::FunctionInvalidArgument(name.into(), arg_num, expected_type.into())
    }
}

impl Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
            EvaluationError::FunctionCannotConvertToValue(func_name) => f.write_fmt(format_args!("Function '{}' cannot be converted to a JSON value", func_name))

        }
    }
}

pub(crate) type JSONataVariables = HashMap<String, Box<Option<JSONataValue>>>;

#[derive(Debug)]
pub struct Context<'a> {
    data: &'a Value,

    variables: JSONataVariables,
}

impl Default for Context<'_> {
    fn default() -> Self {
        Context {
            data: &Value::Null,
            variables: HashMap::new(),
        }
    }
}

impl<'a> Context<'a> {
    pub fn from_data(data: &'a Value) -> Context<'a> {
        let mut context = Context {
            data,
            variables: HashMap::new(),
        };
        context.load_builtins();
        context
    }

    fn load_builtins(&mut self) {
        BuiltIns::populate_context(&mut self.variables)
    }

    pub fn data(&self) -> &Value {
        &self.data
    }

    pub fn set_var<T, V>(&mut self, var_name: T, value: V)
    where
        T: Into<String>,
        V: Into<Box<Option<JSONataValue>>>,
    {
        self.variables.insert(var_name.into(), value.into());
    }

    pub fn set_data(&mut self, data: &'a Value) {
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
