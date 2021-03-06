mod event;
mod expr;
mod parser;
mod sink;
mod source;

use self::parser::{ParseError, Parser};
use lexer::Lexer;
use rowan::GreenNode;
use source::Source;
use syntax::SyntaxNode;

use self::sink::Sink;

pub fn parse(input: &str) -> Parse {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let source = Source::new(&tokens);
    let parser = Parser::new(source);
    let events = parser.parse();
    let sink = Sink::new(&tokens, events);

    sink.finish()
}

pub struct Parse {
    green_node: GreenNode,
    errors: Vec<ParseError>,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let mut s = String::new();

        let tree = format!("{:#?}", self.syntax());

        // Remove the newline from the end
        s.push_str(&tree[0..tree.len() - 1]);

        for error in &self.errors {
            s.push_str(&format!("\n{}", error));
        }

        s
    }

    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }
}

enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,

    Map,
}

impl BinaryOp {
    /// Binding power tuple of (left, right)
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
            Self::Map => (3, 4),
        }
    }
}

enum UnaryOp {
    Neg,
}

impl UnaryOp {
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

    pub(crate) fn check(input: &str, expected_tree: Expect) {
        let parse = parse(input);
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
                  Literal@0..3
                    Number@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_variable_ref() {
        check(
            "$counter",
            expect![[r#"
                Root@0..8
                  VariableRef@0..8
                    VariableIdent@0..8 "$counter""#]],
        );
    }

    #[test]
    fn parse_variable_def() {
        check(
            "$foo := 5",
            expect![[r#"
            Root@0..9
              VariableDef@0..9
                VariableIdent@0..4 "$foo"
                Whitespace@4..5 " "
                ColonEquals@5..7 ":="
                Whitespace@7..8 " "
                Literal@8..9
                  Number@8..9 "5""#]],
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
                    Literal@1..3
                      Number@1..3 "10""#]],
        )
    }

    #[test]
    fn parse_nested_parentheses() {
        check(
            "((((((10))))))",
            expect![[r#"
                Root@0..14
                  ParenExpr@0..14
                    LParen@0..1 "("
                    ParenExpr@1..13
                      LParen@1..2 "("
                      ParenExpr@2..12
                        LParen@2..3 "("
                        ParenExpr@3..11
                          LParen@3..4 "("
                          ParenExpr@4..10
                            LParen@4..5 "("
                            ParenExpr@5..9
                              LParen@5..6 "("
                              Literal@6..8
                                Number@6..8 "10"
                              RParen@8..9 ")"
                            RParen@9..10 ")"
                          RParen@10..11 ")"
                        RParen@11..12 ")"
                      RParen@12..13 ")"
                    RParen@13..14 ")""#]],
        );
    }

    #[test]
    fn parse_whitespace() {
        check(
            "   ",
            expect![[r#"
Root@0..3
  Whitespace@0..3 "   ""#]],
        );
    }

    #[test]
    fn parse_number_preceded_by_whitespace() {
        check(
            "   9876",
            expect![[r#"
                Root@0..7
                  Whitespace@0..3 "   "
                  Literal@3..7
                    Number@3..7 "9876""#]],
        );
    }

    #[test]
    fn parse_number_followed_by_whitespace() {
        check(
            "999   ",
            expect![[r#"
                Root@0..6
                  Literal@0..6
                    Number@0..3 "999"
                    Whitespace@3..6 "   ""#]],
        );
    }

    #[test]
    fn parse_number_surrounded_by_whitespace() {
        check(
            " 123     ",
            expect![[r#"
                Root@0..9
                  Whitespace@0..1 " "
                  Literal@1..9
                    Number@1..4 "123"
                    Whitespace@4..9 "     ""#]],
        );
    }

    #[test]
    fn parse_infix_expression_with_whitespace() {
        check(
            " 1 +   2* 3 ",
            expect![[r#"
                Root@0..12
                  Whitespace@0..1 " "
                  InfixExpr@1..12
                    Literal@1..3
                      Number@1..2 "1"
                      Whitespace@2..3 " "
                    Plus@3..4 "+"
                    Whitespace@4..7 "   "
                    InfixExpr@7..12
                      Literal@7..8
                        Number@7..8 "2"
                      Star@8..9 "*"
                      Whitespace@9..10 " "
                      Literal@10..12
                        Number@10..11 "3"
                        Whitespace@11..12 " ""#]],
        );
    }

    #[test]
    fn parse_infix_expression_interspersed_with_comments() {
        check(
            "
1
  + 1 /* Add one */
  + 10 /* Add ten */",
            expect![[r#"
                Root@0..43
                  Whitespace@0..1 "\n"
                  InfixExpr@1..43
                    InfixExpr@1..25
                      Literal@1..5
                        Number@1..2 "1"
                        Whitespace@2..5 "\n  "
                      Plus@5..6 "+"
                      Whitespace@6..7 " "
                      Literal@7..25
                        Number@7..8 "1"
                        Whitespace@8..9 " "
                        Comment@9..22 "/* Add one */"
                        Whitespace@22..25 "\n  "
                    Plus@25..26 "+"
                    Whitespace@26..27 " "
                    Literal@27..43
                      Number@27..29 "10"
                      Whitespace@29..30 " "
                      Comment@30..43 "/* Add ten */""#]],
        );
    }
}
