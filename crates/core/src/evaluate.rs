use crate::value::JSONataValue;
use thiserror::Error;

pub(crate) type EvaluationResult = Result<Option<JSONataValue>, EvaluationError>;

#[derive(Error, Debug, PartialEq)]
pub(crate) enum EvaluationError {
    #[error("cannot convert value to a number")]
    CannotConvertToNumber,

    #[error("cannot convert value")]
    FunctionCannotConvertToValue,

    #[error("invalid argument to function")]
    FunctionInvalidArgument,

    #[error("incorrect number of arguments for function ‘{func_name}’: expected {expected} arguments, received {actual} arguments")]
    FunctionIncorrectNumberArguments {
        func_name: String,
        expected: usize,
        actual: usize,
    },

    #[error(
        "both operands of mathematical operation ‘{op}’ must be numbers: found ‘{lhs}’ and ‘{rhs}’"
    )]
    OperandsMustBeNumbers {
        op: String,
        lhs: String,
        rhs: String,
    },
}
