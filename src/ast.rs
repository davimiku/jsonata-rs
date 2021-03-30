pub struct Program {
    span: (Location, Location),

    // charRange: (u32, u32)  // from 0 to number of characters (do we need this?)
    expressions: Vec<Expression>,
}

pub struct Location {
    line: u32,
    col: u32,
    char: u64,
}

pub enum Node {
    Expression(Expression),
}

pub enum Expression {
    Path,

    // Unary expressions
    // Unary,
    Group,

    // Binary expressions
    // Binary,
    Numeric,
    Equality,
    Comparison,
    StringConcat,
    Range,
    Includes,

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

pub enum VariableDeclaration {}
