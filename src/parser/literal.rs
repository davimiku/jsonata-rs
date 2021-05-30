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
    expr::Expression,
    literal::{LiteralExpression, LiteralValue},
};

use super::string::literal_string;

#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;

    use super::*;

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
