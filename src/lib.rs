use nom::error::Error;
use nom_locate::LocatedSpan;
use nom_recursive::RecursiveInfo;
use parser::parse;

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

pub fn jsonata(input: &str) -> Result<Program, nom::Err<Error<LocatedSpan<&str, RecursiveInfo>>>> {
    let expression = parse(input)?;
    Ok(Program::new(expression))
}
