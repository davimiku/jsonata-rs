use crate::value::JSONataValue;

use self::error::EvaluationError;

pub(crate) mod error;

pub(crate) type EvaluationResult = Result<Option<JSONataValue>, EvaluationError>;
