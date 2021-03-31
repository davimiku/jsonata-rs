#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Program {
    // span: (Location, Location),

    // charRange: (u32, u32)  // from 0 to number of characters (do we need this?)
    pub nodes: Vec<Node>,
}

impl Program {
    // should it take &str or serde_json::Value ?
    pub fn evaluate() {}
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Location {
    line: u32,
    col: u32,
    char: u64,
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Node {
    pub ntype: NodeType,
    // pub loc: Location,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum NodeType {
    ExpressionStatement(Expression),
    VariableDeclaration(Declaration),
    FunctionDeclaration(Declaration),
    Empty, // Test only, TODO: remove
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expression {
    Path(Path),

    // Unary expressions
    // Unary,
    Group,

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

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Path {
    pub ident: String,
    pub member: Box<Option<Path>>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum BinaryExpression {
    Numeric,
    Equality,
    Comparison,
    StringConcat,
    Range,
    Includes,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Declaration {
    Test,
}
