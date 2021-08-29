use crate::{
    evaluate::{Context, EvaluationResult},
    value::JSONataValue,
};

use super::{
    dyadic::{
        arithmetic::ArithmeticExpression, compare::CompareExpression, concat::ConcatExpression,
        InclusionExpression,
    },
    literal::LiteralExpression,
    path::{FilterExpression, MapExpression, PathExpression, ReduceExpression},
};

// #[derive(PartialEq, Eq, Debug, Clone)]
// pub enum ExpressionType {
//     Path,
//     Unary,
//     Group,

//     /// Literal value of a string/number/boolean/null
//     Literal,

//     /// String Concatenation binary expression
//     StringConcat,

//     Name,
//     String,
//     Number,
//     Value,
//     Wildcard,
//     Descendant,
//     Parent,
//     Condition,
//     Block,
//     Bind,
//     Regex,
//     Function,
//     VariableBinding,
//     Lambda,
//     Partial,
//     Apply,
//     Transform,
// }

#[derive(Debug, PartialEq)]
pub(crate) enum Expression {
    Multi(MultiExpression),

    Literal(LiteralExpression),

    Variable(VariableBindingExpression),

    // Related to path operators or path expressions
    Map(MapExpression),
    Path(PathExpression),
    Filter(FilterExpression),
    Reduce(ReduceExpression),

    // Dyadic expressions
    Compare(CompareExpression),
    Arithmetic(ArithmeticExpression),
    Concat(ConcatExpression),
    Includes(InclusionExpression),
}

impl Expression {
    pub fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        match self {
            Expression::Multi(expr) => expr.evaluate(context),
            Expression::Literal(expr) => expr.evaluate(context),
            Expression::Variable(expr) => expr.evaluate(context),
            Expression::Map(expr) => expr.evaluate(context),
            Expression::Path(expr) => expr.evaluate(context),
            Expression::Filter(expr) => expr.evaluate(context),
            Expression::Reduce(expr) => expr.evaluate(context),
            Expression::Compare(expr) => expr.evaluate(context),
            Expression::Arithmetic(expr) => expr.evaluate(context),
            Expression::Concat(expr) => expr.evaluate(context),
            Expression::Includes(expr) => expr.evaluate(context),
        }
    }
}

impl From<LiteralExpression> for Expression {
    fn from(expr: LiteralExpression) -> Self {
        Expression::Literal(expr)
    }
}

impl From<MultiExpression> for Expression {
    fn from(expr: MultiExpression) -> Self {
        Expression::Multi(expr)
    }
}

impl From<VariableBindingExpression> for Expression {
    fn from(expr: VariableBindingExpression) -> Self {
        Expression::Variable(expr)
    }
}

impl From<MapExpression> for Expression {
    fn from(expr: MapExpression) -> Self {
        Expression::Map(expr)
    }
}

impl From<PathExpression> for Expression {
    fn from(expr: PathExpression) -> Self {
        Expression::Path(expr)
    }
}

impl From<CompareExpression> for Expression {
    fn from(expr: CompareExpression) -> Self {
        Expression::Compare(expr)
    }
}

impl From<ArithmeticExpression> for Expression {
    fn from(expr: ArithmeticExpression) -> Self {
        Expression::Arithmetic(expr)
    }
}

impl From<InclusionExpression> for Expression {
    fn from(expr: InclusionExpression) -> Self {
        Expression::Includes(expr)
    }
}

impl From<ConcatExpression> for Expression {
    fn from(expr: ConcatExpression) -> Self {
        Expression::Concat(expr)
    }
}

/// A MultiExpression contains one or more expressions
///
/// Evaluation creates a new closure and the return
/// value of the final inner expression is the return
/// value of the MultiExpression.
///
///
#[derive(Debug, PartialEq)]
pub(crate) struct MultiExpression {
    pub expressions: Vec<Expression>,
}

impl MultiExpression {
    /// Evaluate each expression, returning the final value.
    fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        let mut value: Option<JSONataValue> = None;
        for expr in &self.expressions {
            value = expr.evaluate(context)?;
        }
        Ok(value)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct VariableBindingExpression {
    pub var_name: String,
    pub bound_expression: Box<Expression>,
}

impl VariableBindingExpression {
    fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        let value = self.bound_expression.evaluate(context)?;
        context.set_var(self.var_name.clone(), value);

        // FIXME: Implement Clone for JSONataFunction so values can be cloned
        // For now this returns None but it should also return the value of the expression that
        // was just assigned to the variable.
        //
        // context.set_var(self.var_name.clone(), value.clone());
        // Ok(value)

        Ok(None)
    }
}
