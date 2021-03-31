use std::iter::Peekable;

use crate::ast::{Expression, Node, NodeType, Path, Program};
use crate::lexer::{lex_tokens, Token};

pub fn parse(s: &str) -> Result<Program, ParseError> {
    let tokens = lex_tokens(s);
    let tok_iter = tokens.into_iter();
    let mut parser = Parser::new(tok_iter);
    parser.parse()
}

pub struct Parser<I: Iterator<Item = Token>> {
    // tokens: Vec<Token>,
    tok_iter: Peekable<I>,

    curr: Option<Token>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(tok_iter: I) -> Self {
        let peekable_iter = tok_iter.peekable();
        Parser {
            tok_iter: peekable_iter,
            curr: None,
        }
    }

    // pub fn from_tokens(tokens: Vec<Token>) -> Self {
    //     Parser { tokens }
    // }

    fn advance(&mut self) {
        self.curr = self.tok_iter.next();
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut program = Program { nodes: Vec::new() };

        self.advance();

        // let mut iter = self.tokens.iter().peekable();

        loop {
            // let token = self.tok_iter.next();
            if self.curr.is_none() {
                break;
            }
            let token = self.curr.as_ref().unwrap();
            println!("curr: {:?}", token);
            println!("peek: {:?}", self.tok_iter.peek());

            match token {
                Token::Ident(s) => {
                    self.parse_path(s);
                    let node = Node {
                        ntype: NodeType::ExpressionStatement(Expression::Path(Path {
                            ident: s.to_string(),
                            member: Box::new(None),
                        })),
                    };
                    program.nodes.push(node);
                }
                _ => {}
            }
            self.advance();
        }
        Ok(program)
    }

    pub fn parse_path(&mut self, s: &String) -> Result<Node, ParseError> {
        match self.tok_iter.peek() {
            Some(Token::Dot) => {
                // object property
                self.advance();
            }
            Some(Token::LeftBracket) => {
                // Array index, predicate expression, or array constructor
            }
            Some(Token::Caret) => {
                // Order-by
            }
            Some(Token::LeftCurly) => {
                // Object constructor or "reduce"
            }

            Some(_) => {}
            None => {}
        }
        let path = Path {
            ident: s.to_string(),
            member: Box::new(None),
        };
        let expr = Expression::Path(path);
        Ok(Node {
            ntype: NodeType::ExpressionStatement(expr),
        })
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseError {
    Syntax,
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ast::{Expression, Node, Path};

    fn make_program(n: Node) -> Program {
        Program { nodes: vec![n] }
    }

    #[test]
    fn single_path_identifier() {
        let result = parse("Surname");
        let actual = result.unwrap();
        let expected = Node {
            ntype: NodeType::ExpressionStatement(Expression::Path(Path {
                ident: "Surname".to_string(),
                member: Box::new(None),
            })),
        };
        assert_eq!(actual, make_program(expected));
    }

    #[test]
    fn one_level_path() {
        let result = parse("Address.City");
        let actual = result.unwrap();
        let expected = Node {
            ntype: NodeType::ExpressionStatement(Expression::Path(Path {
                ident: "Address".to_string(),
                member: Box::new(Some(Path {
                    ident: "City".to_string(),
                    member: Box::new(None),
                })),
            })),
        };
        assert_eq!(actual, make_program(expected));
    }
}
