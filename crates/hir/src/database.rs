use la_arena::Arena;
use syntax::SyntaxKind;

use crate::{BinaryOp, Expr, UnaryOp};

#[derive(Debug, Default, PartialEq)]
pub struct Database {
    exprs: Arena<Expr>,
}

impl Database {
    pub(crate) fn lower_expr(&mut self, ast: Option<ast::Expr>) -> Expr {
        if let Some(ast) = ast {
            match ast {
                ast::Expr::Binary(ast) => self.lower_binary(ast),
                ast::Expr::Literal(ast) => Expr::Literal { n: ast.parse() },
                ast::Expr::Paren(ast) => self.lower_expr(ast.expr()),
                ast::Expr::PathIdent(ast) => Expr::PathIdent { name: ast.name() },
                ast::Expr::Unary(ast) => self.lower_unary(ast),
                ast::Expr::VariableRef(ast) => Expr::VariableRef { var: ast.name() },
                ast::Expr::VariableDef(ast) => self.lower_variable_def(ast),
            }
        } else {
            Expr::Missing
        }
    }

    fn lower_binary(&mut self, ast: ast::BinaryExpr) -> Expr {
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::Plus => BinaryOp::Add,
            SyntaxKind::Minus => BinaryOp::Sub,
            SyntaxKind::Star => BinaryOp::Mul,
            SyntaxKind::Slash => BinaryOp::Div,
            SyntaxKind::Dot => BinaryOp::Map,
            _ => unreachable!(),
        };

        let lhs = self.lower_expr(ast.lhs());
        let rhs = self.lower_expr(ast.rhs());

        Expr::Binary {
            op,
            lhs: self.exprs.alloc(lhs),
            rhs: self.exprs.alloc(rhs),
        }
    }

    fn lower_unary(&mut self, ast: ast::UnaryExpr) -> Expr {
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::Minus => UnaryOp::Neg,
            _ => unreachable!(),
        };

        let expr = self.lower_expr(ast.expr());

        Expr::Unary {
            op,
            expr: self.exprs.alloc(expr),
        }
    }

    fn lower_variable_def(&mut self, ast: ast::VariableDef) -> Expr {
        let expr = self.lower_expr(ast.value());
        Expr::VariableDef {
            name: ast.name().unwrap().text().into(),
            value: self.exprs.alloc(expr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> ast::Root {
        ast::Root::cast(parser::parse(input).syntax()).unwrap()
    }

    fn check(input: &str, expected_hir: Expr, expected_database: Database) {
        let root = parse(input);
        dbg!(&root);
        let first_expr = root.expr();
        dbg!(&first_expr);
        let mut database = Database::default();
        let hir = database.lower_expr(first_expr);

        assert_eq!(hir, expected_hir);
        assert_eq!(database, expected_database);
    }

    #[test]
    fn lower_literal() {
        check("100", Expr::Literal { n: 100 }, Database::default());
    }

    #[test]
    fn lower_binary_expr() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(Expr::Literal { n: 2 });
        let rhs = exprs.alloc(Expr::Literal { n: 3 });

        check(
            "2 + 3",
            Expr::Binary {
                op: BinaryOp::Add,
                lhs,
                rhs,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_unary_expr() {
        let mut exprs = Arena::new();
        let twelve = exprs.alloc(Expr::Literal { n: 12 });

        check(
            "-12",
            Expr::Unary {
                expr: twelve,
                op: UnaryOp::Neg,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_variable_ref() {
        check(
            "$foo",
            Expr::VariableRef { var: "$foo".into() },
            Database::default(),
        );
    }

    #[test]
    fn lower_variable_def() {
        let mut exprs = Arena::new();
        let value = exprs.alloc(Expr::Literal { n: 5 });

        check(
            "$foo := 5",
            Expr::VariableDef {
                name: "$foo".into(),
                value,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_path_ident() {
        check(
            "Account",
            Expr::PathIdent {
                name: "Account".into(),
            },
            Database::default(),
        );
    }
}
