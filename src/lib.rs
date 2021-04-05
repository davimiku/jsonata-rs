use ast::Program;
use lexer::lex_tokens;
use parser::{parse, ParseError};

mod ast;
mod e2e;
mod evaluate;
mod lexer;
mod parser;

pub fn jsonata(input: &str) -> Result<Program, ParseError> {
    let tokens = lex_tokens(input);
    parse(tokens.into_iter())
}
