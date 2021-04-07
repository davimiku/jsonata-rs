use crate::{ast::concat::ConcatExpression, lexer::Token};

use super::{ParseError, Parser};

impl<I: Iterator<Item = Token>> Parser<I> {
    pub(crate) fn parse_string_concat(&mut self) -> Result<(), ParseError> {
        if let Some(expr) = self.last_expr_ref() {
            let concat = ConcatExpression {
                left: expr,
                right: expr,
            };
            // TODO: Start here
            Ok(())
        } else {
            Err(ParseError::UnexpectedUnary("&".to_string()))
        }
    }
}
