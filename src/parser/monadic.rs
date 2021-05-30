use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    error::{FromExternalError, ParseError},
    IResult,
};

use crate::ast::{
    expr::Expression,
    literal::{LiteralExpression, LiteralValue},
    path::PathExpression,
};

use super::{ident::path_ident, string::literal_string};

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
pub(super) fn path_expr<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Expression, E> {
    map(path_ident, |ident| {
        PathExpression {
            ident: ident.to_string(),
        }
        .into()
    })(input)
}

/// Parses a boolean value, either true or false
fn literal_bool<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, LiteralValue, E> {
    alt((
        value(LiteralValue::Bool(true), tag("true")),
        value(LiteralValue::Bool(false), tag("false")),
    ))(input)
}

/// Parses the literal value `null`
fn literal_null<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, LiteralValue, E> {
    value(LiteralValue::Null, tag("null"))(input)
}

pub(super) fn literal_expr<'a, E>(input: &'a str) -> IResult<&'a str, Expression, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    map(alt((literal_bool, literal_null, literal_string)), |val| {
        LiteralExpression::from(val).into()
    })(input)
}

#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;

    use super::*;

    #[test]
    fn path_expr_non_delimited() {
        let input = "name";
        let actual = path_expr::<(&str, ErrorKind)>(input).unwrap().1;

        let expected = PathExpression {
            ident: "name".to_string(),
        }
        .into();
        assert_eq!(actual, expected);
    }

    #[test]
    fn path_expr_delimited() {
        let input = "`hello world`";
        let actual = path_expr::<(&str, ErrorKind)>(input).unwrap().1;

        let expected = PathExpression {
            ident: "hello world".to_string(),
        }
        .into();
        assert_eq!(actual, expected);
    }

    #[test]
    fn bool_parser_true() {
        let input = "true";
        let res = literal_bool::<(&str, ErrorKind)>(input);
        assert_eq!(res, Ok(("", LiteralValue::Bool(true))))
    }

    #[test]
    fn bool_parser_false() {
        let input = "false";
        let res = literal_bool::<(&str, ErrorKind)>(input);
        assert_eq!(res, Ok(("", LiteralValue::Bool(false))));
    }

    #[test]
    fn null_parser() {
        let input = "null";
        let res = literal_null::<(&str, ErrorKind)>(input);
        assert_eq!(res, Ok(("", LiteralValue::Null)))
    }

    #[test]
    fn literal_expression_parser() {
        assert_eq!(
            literal_expr::<(&str, ErrorKind)>("true"),
            Ok(("", LiteralExpression::from(true).into()))
        );
        assert_eq!(
            literal_expr::<(&str, ErrorKind)>("false"),
            Ok(("", LiteralExpression::from(false).into()))
        );
        assert_eq!(
            literal_expr::<(&str, ErrorKind)>("null"),
            Ok(("", LiteralExpression::from(LiteralValue::Null).into()))
        );
        assert_eq!(
            literal_expr::<(&str, ErrorKind)>(r#""test""#),
            Ok((
                "",
                LiteralExpression::from(LiteralValue::from("test")).into()
            ))
        )
    }
}
