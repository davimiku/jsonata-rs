use crate::parser::marker::CompletedMarker;
use crate::parser::Parser;
use crate::{BinaryOp, UnaryOp};
use syntax::SyntaxKind;

pub(crate) fn expr(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at_end() {
        return None;
    }
    expr_binding_power(p, 0)
}

fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) -> Option<CompletedMarker> {
    let mut lhs = lhs(p)?;

    loop {
        let op = if p.at(SyntaxKind::Plus) {
            BinaryOp::Add
        } else if p.at(SyntaxKind::Minus) {
            BinaryOp::Sub
        } else if p.at(SyntaxKind::Star) {
            BinaryOp::Mul
        } else if p.at(SyntaxKind::Slash) {
            BinaryOp::Div
        } else {
            // We're not at an operator and don't know what to do next, so just return
            // and let the caller decide.
            break;
        };

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < minimum_binding_power {
            break;
        }

        // Eat the operator’s token.
        p.bump();

        let m = lhs.precede(p);
        expr_binding_power(p, right_binding_power);
        lhs = m.complete(p, SyntaxKind::InfixExpr);
    }

    Some(lhs)
}

fn lhs(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(SyntaxKind::Number) {
        literal(p)
    } else if p.at(SyntaxKind::Ident) {
        variable(p)
    } else if p.at(SyntaxKind::Minus) {
        prefix_expr(p)
    } else if p.at(SyntaxKind::LParen) {
        paren_expr(p)
    } else {
        p.error();
        None
    }
}

fn literal(p: &mut Parser) -> Option<CompletedMarker> {
    assert!(p.at(SyntaxKind::Number));

    let m = p.start();
    p.bump();
    Some(m.complete(p, SyntaxKind::Literal))
}

fn variable(p: &mut Parser) -> Option<CompletedMarker> {
    assert!(p.at(SyntaxKind::Ident));

    let m = p.start();
    p.bump();

    if p.at(SyntaxKind::ColonEquals) {
        // variable def
        p.bump();

        expr(p);

        Some(m.complete(p, SyntaxKind::VariableDef))
    } else {
        // variable ref
        Some(m.complete(p, SyntaxKind::VariableRef))
    }
}

fn variable_ref(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(SyntaxKind::Ident));

    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::VariableRef)
}

fn prefix_expr(p: &mut Parser) -> Option<CompletedMarker> {
    assert!(p.at(SyntaxKind::Minus));

    let m = p.start();

    let op = UnaryOp::Neg;
    let ((), right_binding_power) = op.binding_power();

    // Eat the operator’s token.
    p.bump();

    expr_binding_power(p, right_binding_power);

    Some(m.complete(p, SyntaxKind::PrefixExpr))
}

fn paren_expr(p: &mut Parser) -> Option<CompletedMarker> {
    assert!(p.at(SyntaxKind::LParen));

    let m = p.start();
    p.bump();
    expr_binding_power(p, 0);
    p.expect(SyntaxKind::RParen);

    Some(m.complete(p, SyntaxKind::ParenExpr))
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::tests::check;

    #[test]
    fn parse_simple_infix_expression() {
        check(
            "1+2",
            expect![[r#"
                Root@0..3
                  InfixExpr@0..3
                    Literal@0..1
                      Number@0..1 "1"
                    Plus@1..2 "+"
                    Literal@2..3
                      Number@2..3 "2""#]],
        );
    }

    #[test]
    fn parse_left_associative_infix_expression() {
        check(
            "1+2+3+4",
            expect![[r#"
                Root@0..7
                  InfixExpr@0..7
                    InfixExpr@0..5
                      InfixExpr@0..3
                        Literal@0..1
                          Number@0..1 "1"
                        Plus@1..2 "+"
                        Literal@2..3
                          Number@2..3 "2"
                      Plus@3..4 "+"
                      Literal@4..5
                        Number@4..5 "3"
                    Plus@5..6 "+"
                    Literal@6..7
                      Number@6..7 "4""#]],
        );
    }

    #[test]
    fn parse_infix_expression_with_mixed_binding_power() {
        check(
            "1+2*3-4",
            expect![[r#"
                Root@0..7
                  InfixExpr@0..7
                    InfixExpr@0..5
                      Literal@0..1
                        Number@0..1 "1"
                      Plus@1..2 "+"
                      InfixExpr@2..5
                        Literal@2..3
                          Number@2..3 "2"
                        Star@3..4 "*"
                        Literal@4..5
                          Number@4..5 "3"
                    Minus@5..6 "-"
                    Literal@6..7
                      Number@6..7 "4""#]],
        );
    }

    #[test]
    fn negation_has_higher_binding_power_than_infix_operators() {
        check(
            "-20+20",
            expect![[r#"
                Root@0..6
                  InfixExpr@0..6
                    PrefixExpr@0..3
                      Minus@0..1 "-"
                      Literal@1..3
                        Number@1..3 "20"
                    Plus@3..4 "+"
                    Literal@4..6
                      Number@4..6 "20""#]],
        );
    }

    #[test]
    fn parentheses_affect_precedence() {
        check(
            "5*(2+1)",
            expect![[r#"
                Root@0..7
                  InfixExpr@0..7
                    Literal@0..1
                      Number@0..1 "5"
                    Star@1..2 "*"
                    ParenExpr@2..7
                      LParen@2..3 "("
                      InfixExpr@3..6
                        Literal@3..4
                          Number@3..4 "2"
                        Plus@4..5 "+"
                        Literal@5..6
                          Number@5..6 "1"
                      RParen@6..7 ")""#]],
        );
    }

    #[test]
    fn parse_variable_definition() {
        check(
            "$foo := 4",
            expect![[r#"
Root@0..9
  VariableDef@0..9
    Ident@0..4 "$foo"
    Whitespace@4..5 " "
    ColonEquals@5..7 ":="
    Whitespace@7..8 " "
    Literal@8..9
      Number@8..9 "4""#]],
        );
    }

    #[test]
    fn parse_unclosed_parentheses() {
        check(
            "(2",
            expect![[r#"
Root@0..2
  ParenExpr@0..2
    LParen@0..1 "("
    Literal@1..2
      Number@1..2 "2"
error at 1..2: expected ‘+’, ‘-’, ‘*’, ‘/’ or ‘)’"#]],
        )
    }
}
