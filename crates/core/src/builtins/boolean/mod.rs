use crate::evaluate::EvaluationResult;
use crate::value::JSONataValue;

use super::BuiltIns;

#[cfg(test)]
mod tests;

impl BuiltIns {
    /// Casts the argument to a Boolean using the following rules:
    ///
    /// Boolean                     --> unchanged
    /// string: empty               --> false
    /// string: non-empty           --> true
    /// number: 0                   --> false
    /// number: non-zero            --> true
    /// null                        --> false
    /// array:
    /// - empty                     --> false
    /// - >= 1 member casts to true --> true
    /// - all members cast to false --> false
    /// object: empty               --> false
    /// object: non-empty           --> true
    /// function                    --> false
    ///
    /// **Signature**: `$boolean(arg)`
    pub fn boolean(arg: &JSONataValue) -> EvaluationResult {
        Ok(Some(boolean_inner(arg).into()))
    }

    /// Returns Boolean NOT on the argument. arg is first cast to a boolean
    ///
    /// **Signature**: `$not(arg)`
    pub fn not(arg: &JSONataValue) -> EvaluationResult {
        Ok(Some((!boolean_inner(arg)).into()))
    }

    /// Returns Boolean true if the arg expression evaluates to a value,
    /// or false if the expression does not match anything
    /// (e.g. a path to a non-existent field reference).
    ///
    /// **Signature**: `$exists(arg)`
    pub fn exists(arg: &Option<JSONataValue>) -> EvaluationResult {
        Ok(Some(arg.is_some().into()))
    }
}

#[inline]
fn boolean_inner(arg: &JSONataValue) -> bool {
    arg.into()
}
