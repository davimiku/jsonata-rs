use syntax::SyntaxKind;

pub fn lower(ast: ast::Root) -> impl Iterator<Item = Expr> {
    ast.exprs().filter_map(|expr| Some(Expr::lower(Some(expr))))
}

#[derive(Debug)]
pub enum Expr {
    Missing,
    Binary {
        op: BinaryOp,
        lhs: Box<Self>,
        rhs: Box<Self>,
    },
    Literal {
        n: u64,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Self>,
    },
    VariableRef {
        var: String,
    },
    VariableDef {
        name: String,
        value: Box<Expr>,
    },
}

impl Expr {
    pub(self) fn lower(ast: Option<ast::Expr>) -> Self {
        if let Some(ast) = ast {
            match ast {
                ast::Expr::BinaryExpr(ast) => Self::lower_binary(ast),
                ast::Expr::Literal(ast) => Self::Literal { n: ast.parse() },
                ast::Expr::ParenExpr(ast) => Expr::lower(ast.expr()),
                ast::Expr::UnaryExpr(ast) => Self::lower_unary(ast),
                ast::Expr::VariableRef(ast) => Self::VariableRef { var: ast.name() },
                ast::Expr::VariableDef(ast) => Self::VariableDef {
                    name: ast.name().unwrap().text().to_string(),
                    value: Box::new(Self::lower(ast.value())),
                },
            }
        } else {
            Self::Missing
        }
    }

    fn lower_binary(ast: ast::BinaryExpr) -> Self {
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::Plus => BinaryOp::Add,
            SyntaxKind::Minus => BinaryOp::Sub,
            SyntaxKind::Star => BinaryOp::Mul,
            SyntaxKind::Slash => BinaryOp::Div,
            _ => unreachable!(),
        };

        Self::Binary {
            op,
            lhs: Box::new(Expr::lower(ast.lhs())),
            rhs: Box::new(Expr::lower(ast.rhs())),
        }
    }

    fn lower_unary(ast: ast::UnaryExpr) -> Self {
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::Minus => UnaryOp::Neg,
            _ => unreachable!(),
        };

        Self::Unary {
            op,
            expr: Box::new(Expr::lower(ast.expr())),
        }
    }
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
