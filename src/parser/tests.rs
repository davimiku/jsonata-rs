use crate::ast::{expr::VariableBindingExpression, literal::LiteralExpression};

use super::*;

#[test]
fn parse_test() {
    let input = "$myvar := false";
    let res = parse(input).unwrap();
    assert_eq!(
        res,
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
    let (actual_remainder, actual) = expr_parser(make_span(input)).unwrap();
    let expected: Expression = VariableBindingExpression {
        var_name: "myvar".to_string(),
        bound_expression: Box::new(LiteralExpression::from(()).into()),
    }
    .into();
    assert_eq!(expected, actual);
    assert_eq!("", *actual_remainder);
}
