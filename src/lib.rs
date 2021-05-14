use nom::error::ErrorKind;
use parser::parse;

use ast::Program;

mod ast;
mod builtins;
mod evaluate;
mod parser;
#[cfg(test)]
mod tests;

// TODO: Make a custom error type for this crate
// that can be converted from nom::Err

pub fn jsonata(input: &str) -> Result<Program, nom::Err<(&str, ErrorKind)>> {
    let expression = parse(input)?;
    Ok(Program::new(expression))
}
