mod expr;

use crate::lexer::{Lexer, SyntaxKind};
use crate::syntax::{JsonataLanguage, SyntaxNode};
use expr::expr;
use rowan::{Checkpoint, GreenNode, GreenNodeBuilder, Language};
use std::iter::Peekable;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    builder: GreenNodeBuilder<'static>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn parse(mut self) -> Parse {
        self.start_node(SyntaxKind::Root);

        expr(&mut self);

        self.finish_node();

        Parse {
            green_node: self.builder.finish(),
        }
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek().map(|(kind, _)| *kind)
    }

    fn bump(&mut self) {
        // unwrap: bump() must be only used if peek() is Some
        let (kind, text) = self.lexer.next().unwrap();

        self.builder
            .token(JsonataLanguage::kind_to_raw(kind), text.into())
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(JsonataLanguage::kind_to_raw(kind))
    }

    fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder
            .start_node_at(checkpoint, JsonataLanguage::kind_to_raw(kind));
    }

    fn finish_node(&mut self) {
        self.builder.finish_node()
    }

    fn checkpoint(&self) -> Checkpoint {
        self.builder.checkpoint()
    }
}

pub struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        // Remove the newline from the end
        formatted[0..formatted.len() - 1].to_string()
    }
}

enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl InfixOp {
    /// Binding power tuple of (left, right)
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}

enum PrefixOp {
    Neg,
}

impl PrefixOp {
    fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Neg => ((), 5),
        }
    }
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use super::*;

    fn check(input: &str, expected_tree: Expect) {
        let parse = Parser::new(input).parse();
        expected_tree.assert_eq(&parse.debug_tree());
    }

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }

    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
Root@0..3
  Number@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_binding_usage() {
        check(
            "$counter",
            expect![[r#"
Root@0..8
  Ident@0..8 "$counter""#]],
        );
    }

    #[test]
    fn parse_negation() {
        check(
            "-10",
            expect![[r#"
Root@0..3
  PrefixExpr@0..3
    Minus@0..1 "-"
    Number@1..3 "10""#]],
        )
    }

    #[test]
    fn parse_nested_parentheses() {
        check(
            "((((((10))))))",
            expect![[r#"
Root@0..14
  LParen@0..1 "("
  LParen@1..2 "("
  LParen@2..3 "("
  LParen@3..4 "("
  LParen@4..5 "("
  LParen@5..6 "("
  Number@6..8 "10"
  RParen@8..9 ")"
  RParen@9..10 ")"
  RParen@10..11 ")"
  RParen@11..12 ")"
  RParen@12..13 ")"
  RParen@13..14 ")""#]],
        );
    }
}
