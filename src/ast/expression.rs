use crate::evaluate::{Context, EvaluationResult};

pub trait Expression {
    fn etype(&self) -> ExpressionType;
    /// Runs this AST node
    fn evaluate(&self, context: &Context) -> EvaluationResult;
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ExpressionType {
    Path,
    Unary,
    Group,

    /// Literal value of a string/number/boolean/null
    Literal,

    /// String Concatenation binary expression
    StringConcat,

    Name,
    String,
    Number,
    Value,
    Wildcard,
    Descendant,
    Parent,
    Condition,
    Block,
    Bind,
    Regex,
    Function,
    Variable,
    Lambda,
    Partial,
    Apply,
    Transform,
}
