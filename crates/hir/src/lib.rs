mod database;
pub use database::Database;

use la_arena::Idx;

type ExprIdx = Idx<Expr>;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Missing,
    Binary {
        op: BinaryOp,
        lhs: ExprIdx,
        rhs: ExprIdx,
    },
    Literal {
        n: u64,
    },
    Unary {
        op: UnaryOp,
        expr: ExprIdx,
    },
    VariableRef {
        var: String,
    },
    VariableDef {
        name: String,
        value: ExprIdx,
    },
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Neg,
}

pub fn lower(ast: ast::Root) -> (Database, Option<Expr>) {
    let mut db = Database::default();
    let expr = ast.expr().map(|expr| db.lower_expr(Some(expr)));
    (db, expr)
}
