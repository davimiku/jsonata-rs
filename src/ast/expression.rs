use crate::evaluate::{Context, EvaluationResult};

use super::{literal::LiteralExpression, path::PathExpression};

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
pub(crate) enum Expression {
    Literal(LiteralExpression),

    Variable(VariableBindingExpression),

    Path(PathExpression),
}

impl Expression {
    pub fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        match self {
            Expression::Literal(expr) => expr.evaluate(context),
            Expression::Variable(expr) => expr.evaluate(context),
            Expression::Path(expr) => expr.evaluate(context),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct VariableBindingExpression {
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
