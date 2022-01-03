mod database;
pub use database::Database;

use la_arena::Idx;

type ExprIdx = Idx<Expr>;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
}

pub fn lower(ast: ast::Root) -> (Database, Vec<Expr>) {
    let mut db = Database::default();
    let exprs = ast.exprs().map(|expr| db.lower_expr(Some(expr))).collect();
    (db, exprs)
}
