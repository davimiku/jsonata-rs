use syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Literal(LiteralExpr),
    Paren(ParenExpr),
    PathIdent(PathIdentExpr),
    Unary(UnaryExpr),
    VariableRef(VariableRef),
    VariableDef(VariableDef),
}

impl Expr {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::InfixExpr => Self::Binary(BinaryExpr(node)),
            SyntaxKind::Literal => Self::Literal(LiteralExpr(node)),
            SyntaxKind::ParenExpr => Self::Paren(ParenExpr(node)),
            SyntaxKind::PrefixExpr => Self::Unary(UnaryExpr(node)),
            SyntaxKind::VariableRef => Self::VariableRef(VariableRef(node)),
            SyntaxKind::VariableDef => Self::VariableDef(VariableDef(node)),
            SyntaxKind::PathIdentExpr => Self::PathIdent(PathIdentExpr(node)),
            _ => return None,
        };

        Some(result)
    }
}

#[derive(Debug)]
pub struct Root(SyntaxNode);

impl Root {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::Root {
            Some(Self(node))
        } else {
            None
        }
    }

    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}

#[derive(Debug)]
pub struct BinaryExpr(SyntaxNode);

impl BinaryExpr {
    pub fn lhs(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn rhs(&self) -> Option<Expr> {
        self.0.children().filter_map(Expr::cast).nth(1)
    }

    pub fn op(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| {
                matches!(
                    token.kind(),
                    SyntaxKind::Plus
                        | SyntaxKind::Minus
                        | SyntaxKind::Star
                        | SyntaxKind::Slash
                        | SyntaxKind::Dot,
                )
            })
    }
}

#[derive(Debug)]
pub struct LiteralExpr(SyntaxNode);

impl LiteralExpr {
    pub fn parse(&self) -> u64 {
        self.0.first_token().unwrap().text().parse().unwrap()
    }
}

#[derive(Debug)]
pub struct ParenExpr(SyntaxNode);

impl ParenExpr {
    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}

#[derive(Debug)]
pub struct PathIdentExpr(SyntaxNode);

impl PathIdentExpr {
    pub fn name(&self) -> String {
        self.0.first_token().unwrap().text().into()
    }
}

#[derive(Debug)]
pub struct UnaryExpr(SyntaxNode);

impl UnaryExpr {
    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn op(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Minus)
    }
}

#[derive(Debug)]
pub struct VariableRef(SyntaxNode);

impl VariableRef {
    pub fn name(&self) -> String {
        self.0.first_token().unwrap().text().into()
    }
}

#[derive(Debug)]
pub struct VariableDef(SyntaxNode);

impl VariableDef {
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::VariableIdent)
    }

    pub fn value(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}
