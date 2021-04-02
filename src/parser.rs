use std::iter::Peekable;

use crate::ast::{Expression, Node, NodeType, Path, Program};
use crate::lexer::{lex_tokens, Token};

pub fn parse(s: &str) -> Result<Program, ParseError> {
    let tokens = lex_tokens(s);
    let tok_iter = tokens.into_iter();
    let mut parser = Parser::new(tok_iter);
    parser.parse()
}

pub struct Parser<'a, I: Iterator<Item = Token>> {
    // tokens: Vec<Token>,
    tok_iter: Peekable<I>,
    curr: Option<&'a Token>,
}

impl<I: Iterator<Item = Token>> Parser<'_, I> {
    pub fn new(tok_iter: I) -> Self {
        let peekable_iter = tok_iter.peekable();
        Parser {
            tok_iter: peekable_iter,
            curr: None,
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut program = Program { nodes: Vec::new() };

        let mut next = self.tok_iter.next();

        loop {
            if next.is_none() {
                break;
            }
            let token = next.unwrap();

            match token {
                Token::Ident(s) => {
                    let path = self.parse_path(&s)?;
                    program.nodes.push(make_path_node(path));
                }
                _ => {}
            }
            next = self.tok_iter.next();
        }
        Ok(program)
    }

    fn parse_path(&mut self, s: &String) -> Result<Path, ParseError> {
        let mut path = Path {
            ident: s.to_string(),
            member: Box::new(None),
        };
        match self.tok_iter.peek() {
            Some(Token::Dot) => {
                // object property
                self.tok_iter.next(); // dot
                let next = self.tok_iter.next();
                match next {
                    Some(Token::Ident(s)) => {
                        let inner_path = self.parse_path(&s)?;
                        *path.member = Some(inner_path);
                    }
                    Some(_) => return Err(ParseError::NotImplemented),
                    None => return Err(ParseError::Syntax), // unexpected end of input
                }
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
            Some(Token::Semicolon) => {
                // Syntax error
            }

            Some(_) => {
                // End of path expression
            }
            None => {
                // end of input, also implies end of path expression
            }
        }

        Ok(path)
    }
}

fn make_path_node(path: Path) -> Node {
    Node {
        ntype: NodeType::ExpressionStatement(Expression::Path(path)),
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseError {
    Syntax,

    NotImplemented, // FIXME: Implement specific errors
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ast::{Node, Path};

    fn make_program(n: Node) -> Program {
        Program { nodes: vec![n] }
    }

    #[test]
    fn single_path_identifier() {
        let result = parse("Surname");
        let actual = result.unwrap();
        let expected = make_path_node(Path {
            ident: "Surname".to_string(),
            member: Box::new(None),
        });
        assert_eq!(actual, make_program(expected));
    }

    #[test]
    fn one_level_path() {
        let result = parse("Address.City");
        let actual = result.unwrap();
        let expected = make_path_node(Path {
            ident: "Address".to_string(),
            member: Box::new(Some(Path {
                ident: "City".to_string(),
                member: Box::new(None),
            })),
        });

        assert_eq!(actual, make_program(expected));
    }
}
