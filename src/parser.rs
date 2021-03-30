use std::iter::Peekable;

use crate::ast::Node;
use crate::lexer::{lex_tokens, Token};

pub struct Parser<I: Iterator<Item = Token>> {
    tok_iter: I,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn from_str(s: &str) -> Self {
        let tok_iter = lex_tokens(s).iter();

        Parser { tok_iter }
    }

    // fn advance(&mut self) {
    //     self.curr = self.next;
    //     self.next = self.lexer.next().as_ref();
    // }

    pub fn parse(&mut self) -> ParseResult {
        for token in self.tok_iter {
            println!("curr: {:?}", token);
            println!("peeked: {:?}", token);
        }
        Err(ParseError::Syntax)
    }
}

pub type ParseResult = Result<Node, ParseError>;

pub enum ParseError {
    Syntax,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let mut parser = Parser::from_str("Surname.City");
        parser.parse();
        assert!(true);
    }
}
