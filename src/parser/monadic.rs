use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    number::complete::double,
    IResult,
};

use crate::ast::{
    expr::Expression,
    literal::{LiteralExpression, LiteralValue},
    path::PathExpression,
};

use super::{ident::path_ident, string::literal_string, Span};

/// Path expressions represent a location in the parsed JSON
/// to query from.
///
/// ```
/// Account
/// ```
///
/// ```
/// `Hello World`
/// ```
///
/// The path expression is either a non-delimited sequence of alphanumeric
/// (such as Account), where the first character must be alphabetic. Alternatively,
/// backticks can delimit the expression (such as `Hello World`) which is often used
/// when there is a space or other character in the identifier.
pub(super) fn path_expr(span: Span) -> IResult<Span, Expression> {
    map(path_ident, |ident| {
        PathExpression {
            ident: ident.to_string(),
        }
        .into()
    })(span)
}

/// Parses a boolean value, either true or false
fn literal_bool(span: Span) -> IResult<Span, LiteralValue> {
    alt((
        value(LiteralValue::Bool(true), tag("true")),
        value(LiteralValue::Bool(false), tag("false")),
    ))(span)
}

/// Parses the literal value `null`
fn literal_null(span: Span) -> IResult<Span, LiteralValue> {
    value(LiteralValue::Null, tag("null"))(span)
}

/// Parses a floating point value (f64)
fn num_parse(span: Span) -> IResult<Span, f64> {
    double(span)
}

/// Parses a literal number value
fn literal_number(span: Span) -> IResult<Span, LiteralValue> {
    map(num_parse, |f| LiteralValue::from(f))(span)
}

pub(super) fn literal_expr(span: Span) -> IResult<Span, Expression> {
    map(
        alt((literal_bool, literal_null, literal_number, literal_string)),
        |val| LiteralExpression::from(val).into(),
    )(span)
}

#[cfg(test)]
mod tests {

    use crate::parser::make_span;

    use super::*;

    #[test]
    fn path_expr_non_delimited() {
        let input = "name";
        let actual = path_expr(make_span(input)).unwrap().1;

        let expected = PathExpression {
            ident: "name".to_string(),
        }
        .into();
        assert_eq!(actual, expected);
    }

    #[test]
    fn path_expr_delimited() {
        let input = "`hello world`";
        let actual = path_expr(make_span(input)).unwrap().1;

        let expected = PathExpression {
            ident: "hello world".to_string(),
        }
        .into();
        assert_eq!(actual, expected);
    }

    #[test]
    fn bool_parser_true() {
        let input = "true";
        let (_, actual) = literal_bool(make_span(input)).unwrap();
        assert_eq!(actual, LiteralValue::Bool(true));
    }

    #[test]
    fn bool_parser_false() {
        let input = "false";
        let (_, actual) = literal_bool(make_span(input)).unwrap();
        assert_eq!(actual, LiteralValue::Bool(false));
    }

    #[test]
    fn null_parser() {
        let input = "null";
        let (_, actual) = literal_null(make_span(input)).unwrap();
        assert_eq!(actual, LiteralValue::Null);
    }

    #[test]
    fn integer_parser() {
        let input = "5";
        let (_, actual) = literal_number(make_span(input)).unwrap();
        assert_eq!(actual, LiteralValue::Integer(5));
    }

    #[test]
    fn float_parser() {
        let input = "5.1";
        let (_, actual) = literal_number(make_span(input)).unwrap();
        assert_eq!(actual, LiteralValue::Float(5.1));
    }

    #[test]
    fn literal_expression_parser() {
        let actual = vec![
            literal_expr(make_span("true")),
            literal_expr(make_span("false")),
            literal_expr(make_span("null")),
            literal_expr(make_span(r#""test""#)),
        ];
        let expected = vec![
            LiteralExpression::from(true),
            LiteralExpression::from(false),
            LiteralExpression::from(LiteralValue::Null),
            LiteralExpression::from(LiteralValue::from("test")),
        ];
        for (expected_res, actual) in actual.iter().zip(expected.iter()) {
            let (_, expected) = expected_res.as_ref().unwrap();
            let actual_expr: Expression = actual.clone().into();
            assert_eq!(*expected, actual_expr);
        }
    }
}
