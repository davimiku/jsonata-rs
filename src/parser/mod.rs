mod ident;
mod string;

use std::boxed::Box;
use std::error::Error;

use nom::{
    bytes::complete::{is_not, tag, take_while},
    error::{context, VerboseError},
    number::complete::{double, float},
    IResult,
};
use nom_locate::LocatedSpan;

use crate::ast::literal::LiteralValue;
type Span<'a> = LocatedSpan<&'a str>;

/// Type-erased errors
pub type BoxError = Box<dyn Error + Send + Sync>;

fn not_whitespace(i: &str) -> nom::IResult<&str, &str> {
    is_not(" \t")(i)
}

fn escaped_tab(i: &str) -> IResult<&str, &str> {
    nom::combinator::recognize(nom::character::complete::char('\t'))(i)
}

fn escaped_backslash(i: &str) -> IResult<&str, &str> {
    nom::combinator::recognize(nom::character::complete::char('\\'))(i)
}

fn transform_escaped(i: &str) -> nom::IResult<&str, std::string::String> {
    nom::bytes::complete::escaped_transform(
        nom::bytes::complete::is_not("\\"),
        '\\',
        nom::branch::alt((escaped_backslash, escaped_tab)),
    )(i)
}

type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn null_literal(i: &str) -> Res<&str, LiteralValue> {
    context("null", tag("null"))(i).map(|(next, _)| (next, LiteralValue::Null))
}

#[cfg(test)]
mod tests {

    use super::*;

    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

    #[test]
    fn test_null_literal() {
        assert_eq!(null_literal("null"), Ok(("", LiteralValue::Null)));
        assert_eq!(
            null_literal("llun"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("llun", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("llun", VerboseErrorKind::Context("null"))
                ],
            }))
        );
    }

    // #[test]
    // fn one_level_path() {
    //     let input = "name";
    //     let mut parser = make_parser(input);
    //     parser.advance(); // simulate first token already processed

    //     let actual = parser.parse_path(&input.to_string()).unwrap();
    //     let expected = PathExpression {
    //         ident: "name".to_string(),
    //         member: None,
    //     };
    //     assert_eq!(actual, expected);
    // }

    // #[test]
    // fn two_level_path() {
    //     let input = "address.city";
    //     let mut parser = make_parser(input);
    //     parser.advance(); // simulate first token already processed

    //     let actual = parser.parse_path(&"address".to_string()).unwrap();

    //     let expected = PathExpression {
    //         ident: "address".to_string(),
    //         member: Some(Box::new(PathExpression {
    //             ident: "city".to_string(),
    //             member: None,
    //         })),
    //     };

    //     assert_eq!(actual, expected);
    // }

    // #[test]
    // fn three_level_path() {
    //     let input = "address.location.latitude";
    //     let mut parser = make_parser(input);
    //     parser.advance(); // simulate first token already processed

    //     let actual = parser.parse_path(&"address".to_string()).unwrap();

    //     let expected = PathExpression {
    //         ident: "address".to_string(),
    //         member: Some(Box::new(PathExpression {
    //             ident: "location".to_string(),
    //             member: Some(Box::new(PathExpression {
    //                 ident: "latitude".to_string(),
    //                 member: None,
    //             })),
    //         })),
    //     };

    //     assert_eq!(actual, expected);
    // }
}
