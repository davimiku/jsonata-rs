//! Parsers for dyadic expressions
//!
//! - comparison

use nom::{error::ParseError, IResult};

use crate::ast::expr::Expression;

fn comparison<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Expression, E> {
    todo!()
}
