use std::{
    convert::TryInto,
    fmt::{Display, Write},
};

use crate::{
    ast::expr::Expression,
    evaluate::{Context, EvaluationResult},
    value::number::JSONataNumber,
};

use super::DyadicOpType;

#[derive(PartialEq, Debug)]
pub enum ArithmeticOpType {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

impl From<&str> for ArithmeticOpType {
    fn from(s: &str) -> Self {
        match s {
            "+" => ArithmeticOpType::Add,
            "-" => ArithmeticOpType::Sub,
            "*" => ArithmeticOpType::Mul,
            "/" => ArithmeticOpType::Div,
            "%" => ArithmeticOpType::Rem,
            _ => unreachable!(),
        }
    }
}

impl Display for ArithmeticOpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArithmeticOpType::Add => f.write_char('+'),
            ArithmeticOpType::Sub => f.write_char('-'),
            ArithmeticOpType::Mul => f.write_char('*'),
            ArithmeticOpType::Div => f.write_char('/'),
            ArithmeticOpType::Rem => f.write_char('%'),
        }
    }
}

impl From<ArithmeticOpType> for DyadicOpType {
    fn from(n: ArithmeticOpType) -> Self {
        DyadicOpType::Arithmetic(n)
    }
}

#[derive(PartialEq, Debug)]
pub(crate) struct ArithmeticExpression {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
    pub arithmetic_type: ArithmeticOpType,
}

impl ArithmeticExpression {
    /// Evaluate a arithmetic expression
    pub(crate) fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        let lhs = self.lhs.evaluate(context)?;
        let rhs = self.rhs.evaluate(context)?;
        match (lhs, rhs) {
            (Some(lhs), Some(rhs)) => ArithmeticExpression::jsonata_value_arithmetic(
                lhs.try_into()?,
                rhs.try_into()?,
                &self.arithmetic_type,
            ),
            (_, _) => Ok(None),
        }
    }

    fn jsonata_value_arithmetic(
        lhs: JSONataNumber,
        rhs: JSONataNumber,
        op: &ArithmeticOpType,
    ) -> EvaluationResult {
        Ok(Some(
            match op {
                ArithmeticOpType::Add => lhs + rhs,
                ArithmeticOpType::Sub => lhs - rhs,
                ArithmeticOpType::Mul => lhs * rhs,
                ArithmeticOpType::Div => lhs / rhs,
                ArithmeticOpType::Rem => lhs % rhs,
            }
            .into(),
        ))
    }
}
