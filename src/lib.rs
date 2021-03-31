use ast::Program;
use parser::{parse, ParseError};

mod ast;
mod lexer;
mod parser;

pub fn jsonata(s: &str) -> Result<Program, ParseError> {
    parse(s)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
