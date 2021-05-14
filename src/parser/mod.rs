mod expr;
mod ident;
mod literal;
mod string;

use std::boxed::Box;
use std::error::Error;

use nom::{
    bytes::complete::is_not,
    character::complete::space0,
    error::{ErrorKind, ParseError, VerboseError},
    sequence::delimited,
    AsChar, Err as NomErr, IResult, InputIter, InputLength, InputTake, InputTakeAtPosition, Parser,
};
use nom_locate::LocatedSpan;

use crate::ast::expression::Expression;

use self::expr::parse_expression;
type Span<'a> = LocatedSpan<&'a str>;

/// Type-erased errors
pub type BoxError = Box<dyn Error + Send + Sync>;

/// Parses the provided parser, ignoring spaces before
/// and after the matching input.
fn trim<'a, F, I, O, E: ParseError<I>>(parser: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: Parser<I, O, E>,
    I: InputLength + InputIter + InputTakeAtPosition + InputTake + Clone,
    <I as InputIter>::Item: AsChar + Clone,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    delimited(space0, parser, space0)
}

fn not_whitespace(i: &str) -> IResult<&str, &str> {
    is_not(" \t")(i)
}

fn escaped_tab(i: &str) -> IResult<&str, &str> {
    nom::combinator::recognize(nom::character::complete::char('\t'))(i)
}

fn escaped_backslash(i: &str) -> IResult<&str, &str> {
    nom::combinator::recognize(nom::character::complete::char('\\'))(i)
}

fn transform_escaped(i: &str) -> IResult<&str, std::string::String> {
    nom::bytes::complete::escaped_transform(
        nom::bytes::complete::is_not("\\"),
        '\\',
        nom::branch::alt((escaped_backslash, escaped_tab)),
    )(i)
}

pub(crate) fn parse(input: &str) -> Result<Expression, NomErr<(&str, ErrorKind)>> {
    parse_expression(input).map(|(_, ex)| ex)
}

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[cfg(test)]
mod tests {

    use super::*;

    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

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
