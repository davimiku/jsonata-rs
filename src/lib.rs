use ast::Program;
use lexer::lex_tokens;
use parser::{parse, ParseError};

mod ast;
mod evaluate;
mod lexer;
mod parser;
mod tests;

pub fn jsonata(s: &str) -> Result<Program, ParseError> {
    let tokens = lex_tokens(s);
    parse(tokens.into_iter())
}
