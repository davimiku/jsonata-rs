use ast::{
    BinaryExpr, LiteralExpr, ParenExpr, PathIdentExpr, UnaryExpr, VariableDefExpr, VariableRefExpr,
};

use crate::{context::Context, value::JSONataValue};

use self::error::EvaluationError;

pub(crate) mod error;

pub(crate) type EvaluationResult = Result<Option<JSONataValue>, EvaluationError>;

pub(crate) fn evaluate(expr: &ast::Expr, context: &Context) -> EvaluationResult {
    match expr {
        ast::Expr::Binary(expr) => todo!(),
        ast::Expr::Literal(expr) => todo!(),
        ast::Expr::Paren(expr) => todo!(),
        ast::Expr::PathIdent(expr) => evaluate_path_ident_expr(expr, context),
        ast::Expr::Unary(expr) => todo!(),
        ast::Expr::VariableRef(expr) => todo!(),
        ast::Expr::VariableDef(expr) => todo!(),
    }
}

fn evaluate_binary_expr(expr: &BinaryExpr, context: &Context) {
    todo!()
}

fn evaluate_literal_expr(expr: &LiteralExpr, context: &Context) {
    todo!()
}

fn evaluate_paren_expr(expr: &ParenExpr, context: &Context) {
    todo!()
}

fn evaluate_path_ident_expr(expr: &PathIdentExpr, context: &Context) -> EvaluationResult {
    let ident = expr.name();
    let val = context.data.get(ident);
    Ok(val.map(|v| v.to_owned().into()))
}

fn evaluate_unary_expr(expr: &UnaryExpr, context: &Context) {
    todo!()
}

fn evaluate_variable_ref_expr(expr: &VariableRefExpr, context: &Context) {
    todo!()
}

fn evaluate_variable_def_expr(expr: &VariableDefExpr, context: &Context) {
    todo!()
}
