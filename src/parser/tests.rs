use crate::ast::{expr::VariableBindingExpression, literal::LiteralExpression};

use super::*;

#[test]
fn parse_test() {
    let input = "$myvar := false";
    let res = parse(input);
    assert_eq!(
        res.unwrap(),
        VariableBindingExpression {
            var_name: "myvar".to_string(),
            bound_expression: Box::new(Expression::Literal(LiteralExpression::from(false)))
        }
        .into()
    )
}

#[test]
fn expr_parser_test() {
    let input = "$myvar := null";
    let res = expr_parser::<(&str, ErrorKind)>(input);
    assert_eq!(
        res.unwrap().1,
        VariableBindingExpression {
            var_name: "myvar".to_string(),
            bound_expression: Box::new(LiteralExpression::from(()).into())
        }
        .into()
    )
}
