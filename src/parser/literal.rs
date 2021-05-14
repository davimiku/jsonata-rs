//! Module parses literal JSON values including
//!
//! - null
//! - true
//! - false

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    error::{FromExternalError, ParseError},
    IResult,
};

use crate::ast::{
    expression::Expression,
    literal::{LiteralExpression, LiteralValue},
};

use super::string::literal_string;

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

pub(super) fn literal_expression<'a, E>(input: &'a str) -> IResult<&'a str, Expression, E>
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
    fn bool_parser_true() {
        let input = "true";
        let res = literal_bool::<(&str, ErrorKind)>(input);
        assert_eq!(res.unwrap().1, LiteralValue::Bool(true))
    }

    #[test]
    fn bool_parser_false() {
        let input = "false";
        let res = literal_bool::<(&str, ErrorKind)>(input);
        assert_eq!(res.unwrap().1, LiteralValue::Bool(false));
    }

    #[test]
    fn null_parser() {
        let input = "null";
        let res = literal_null::<(&str, ErrorKind)>(input);
        assert_eq!(res.unwrap().1, LiteralValue::Null)
    }

    #[test]
    fn literal_expression_parser() {
        assert_eq!(
            literal_expression::<(&str, ErrorKind)>("true").unwrap().1,
            LiteralExpression::from(true).into()
        );
        assert_eq!(
            literal_expression::<(&str, ErrorKind)>("false").unwrap().1,
            LiteralExpression::from(false).into()
        );
        assert_eq!(
            literal_expression::<(&str, ErrorKind)>("null").unwrap().1,
            LiteralExpression::from(LiteralValue::Null).into()
        );
        assert_eq!(
            literal_expression::<(&str, ErrorKind)>(r#""test""#)
                .unwrap()
                .1,
            LiteralExpression::from(LiteralValue::from("test")).into()
        )
    }
}
