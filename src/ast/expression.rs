use crate::evaluate::{Context, EvaluationResult};

use super::{
    binary::{CompareExpression, InclusionExpression},
    literal::LiteralExpression,
    path::PathExpression,
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

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(LiteralExpression),

    Variable(VariableBindingExpression),

    Path(PathExpression),

    Compare(CompareExpression),
    Includes(InclusionExpression),
}

impl Expression {
    pub fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        match self {
            Expression::Literal(expr) => expr.evaluate(context),
            Expression::Variable(expr) => expr.evaluate(context),
            Expression::Path(expr) => expr.evaluate(context),
            Expression::Compare(expr) => expr.evaluate(context),
            Expression::Includes(expr) => expr.evaluate(context),
        }
    }
}

impl From<LiteralExpression> for Expression {
    fn from(expr: LiteralExpression) -> Self {
        Expression::Literal(expr)
    }
}

impl From<VariableBindingExpression> for Expression {
    fn from(expr: VariableBindingExpression) -> Self {
        Expression::Variable(expr)
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

impl From<InclusionExpression> for Expression {
    fn from(expr: InclusionExpression) -> Self {
        Expression::Includes(expr)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableBindingExpression {
    pub var_name: String,
    pub bound_expression: Box<Expression>,
}

impl VariableBindingExpression {
    fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        let value = self.bound_expression.evaluate(context)?;
        context.set_var(self.var_name.clone(), value.clone());
        Ok(value)
    }
}
