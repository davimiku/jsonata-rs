use serde_json::Value;

use crate::evaluate::EvaluationError;

pub(super) trait TryNumericOps<Rhs = Self> {
    /// Attempt addition between two values
    ///
    /// Ok if the operands are numeric, Err otherwise
    fn try_add(self, rhs: Rhs) -> Result<Self, EvaluationError>
    where
        Self: Sized;

    /// Attempt subtraction between two values
    ///
    /// Ok if the operands are numeric, Err otherwise
    fn try_sub(self, rhs: Rhs) -> Result<Self, EvaluationError>
    where
        Self: Sized;

    /// Attempt multiplication between two values
    ///
    /// Ok if the operands are numeric, Err otherwise
    fn try_mul(self, rhs: Rhs) -> Result<Self, EvaluationError>
    where
        Self: Sized;

    /// Attempt division between two values
    ///
    /// Ok if the operands are numeric, Err otherwise
    fn try_div(self, rhs: Rhs) -> Result<Self, EvaluationError>
    where
        Self: Sized;

    /// Attempt remainder operation between two values
    ///
    /// Ok if the operands are numeric, Err otherwise
    fn try_rem(self, rhs: Rhs) -> Result<Self, EvaluationError>
    where
        Self: Sized;
}
