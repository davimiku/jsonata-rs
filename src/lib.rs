use nom::error::Error;
use parser::{parse, Span};

use ast::Program;

mod ast;
mod builtins;
mod evaluate;
mod parser;
#[cfg(test)]
mod tests;
mod value;

// TODO: Make a custom error type for this crate
// that can be converted from nom::Err

pub type ParseError<'a> = nom::Err<Error<Span<'a>>>;

pub type JSONataResult<'a> = Result<Program<'a>, ParseError<'a>>;

pub fn jsonata(input: &str) -> JSONataResult {
    let expression = parse(input)?;
    Ok(Program::new(expression))
}
