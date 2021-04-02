use ast::Program;
use parser::{parse, ParseError};

mod ast;
mod evaluate;
mod lexer;
mod parser;
mod tests;

pub fn jsonata(s: &str) -> Result<Program, ParseError> {
    parse(s)
}
