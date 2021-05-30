mod dyadic;
mod expr;
mod ident;
mod monadic;
mod string;

use std::boxed::Box;
use std::error::Error;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::space0,
    combinator::value,
    error::{ErrorKind, FromExternalError, ParseError, VerboseError},
    sequence::{delimited, tuple},
    AsChar, Err as NomErr, IResult, InputIter, InputLength, InputTake, InputTakeAtPosition, Parser,
};
use nom_locate::LocatedSpan;

use crate::ast::expr::Expression;

use self::dyadic::comparison_expr;
use self::{
    dyadic::map_expr,
    expr::variable_binding_expr,
    monadic::{literal_expr, path_expr},
};

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

/// Parses a C-Style comment
///
/// Comments begin with the `/*` characters and close with the `*/` characters.
fn comment<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    value((), tuple((tag("/*"), take_until("*/"), tag("*/"))))(input)
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

/// Top-level function for expression parsing
///
/// Calls each of the other parsers in order until a parser
/// yiels success, or returns a ParseError
pub fn expr_parser<'a, E>(input: &'a str) -> IResult<&'a str, Expression, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    alt((
        map_expr,
        path_expr,
        comparison_expr,
        literal_expr,
        variable_binding_expr,
    ))(input)
}

pub(crate) fn parse(input: &str) -> Result<Expression, NomErr<(&str, ErrorKind)>> {
    expr_parser(input).map(|(_, ex)| ex)
}

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[cfg(test)]
mod tests {

    use crate::ast::{expr::VariableBindingExpression, literal::LiteralExpression};

    use super::*;

    #[test]
    fn expression_parser_test<'a>() {
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
    fn expr_parser_test<'a>() {
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
}
