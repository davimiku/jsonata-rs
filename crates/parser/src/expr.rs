use crate::parser::Parser;
use crate::{BinaryOp, UnaryOp};
use syntax::SyntaxKind;

use super::marker::CompletedMarker;

pub(crate) fn expr(p: &mut Parser) -> Option<CompletedMarker> {
    expr_binding_power(p, 0)
}

fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) -> Option<CompletedMarker> {
    let mut lhs = if let Some(lhs) = lhs(p) {
        lhs
    } else {
        return None; // we’ll handle errors later.
    };

    loop {
        let op = match p.peek() {
            Some(SyntaxKind::Plus) => BinaryOp::Add,
            Some(SyntaxKind::Minus) => BinaryOp::Sub,
            Some(SyntaxKind::Star) => BinaryOp::Mul,
            Some(SyntaxKind::Slash) => BinaryOp::Div,
            _ => return None, // we’ll handle errors later.
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
    let cm = match p.peek() {
        Some(SyntaxKind::Number) => literal(p),
        Some(SyntaxKind::Ident) => variable_ref(p),
        Some(SyntaxKind::Minus) => prefix_expr(p),
        Some(SyntaxKind::LParen) => paren_expr(p),
        _ => return None,
    };

    Some(cm)
}

fn literal(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(SyntaxKind::Number));

    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::Literal)
}

fn variable_ref(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(SyntaxKind::Ident));

    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::VariableRef)
}

fn prefix_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(SyntaxKind::Minus));

    let m = p.start();

    let op = UnaryOp::Neg;
    let ((), right_binding_power) = op.binding_power();

    // Eat the operator’s token.
    p.bump();

    expr_binding_power(p, right_binding_power);

    m.complete(p, SyntaxKind::PrefixExpr)
}

fn paren_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(SyntaxKind::LParen));
    let m = p.start();

    p.bump();
    expr_binding_power(p, 0);

    assert!(p.at(SyntaxKind::RParen));
    p.bump();

    m.complete(p, SyntaxKind::ParenExpr)
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
}
