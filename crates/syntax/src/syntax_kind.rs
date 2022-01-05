use std::fmt;

use lexer::TokenKind;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive, ToPrimitive, Hash, PartialOrd, Ord)]
pub enum SyntaxKind {
    Whitespace,
    FunctionKeyword,
    VarIdent,
    PathIdent,
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    Hash,
    Percent,
    At,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Dollar,
    LAngle,
    RAngle,
    ColonEquals,
    DotDot,
    Dot,
    TildeRAngle,
    LBracket,
    RBracket,
    Question,
    Colon,
    SemiColon,
    Literal,
    VariableRef,
    VariableDef,
    InfixExpr,
    PrefixExpr,
    ParenExpr,
    Root,
    Comment,
    Error,
}

impl SyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}

impl From<TokenKind> for SyntaxKind {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::FunctionKeyword => Self::FunctionKeyword,
            TokenKind::VarIdent => Self::VarIdent,
            TokenKind::PathIdent => Self::PathIdent,
            TokenKind::Number => Self::Number,
            TokenKind::Plus => Self::Plus,
            TokenKind::Minus => Self::Minus,
            TokenKind::Star => Self::Star,
            TokenKind::Slash => Self::Slash,
            TokenKind::Equals => Self::Equals,
            TokenKind::Hash => Self::Hash,
            TokenKind::Percent => Self::Percent,
            TokenKind::At => Self::At,
            TokenKind::LParen => Self::LParen,
            TokenKind::RParen => Self::RParen,
            TokenKind::LBrace => Self::LBrace,
            TokenKind::RBrace => Self::RBrace,
            TokenKind::Comment => Self::Comment,
            TokenKind::Error => Self::Error,
            TokenKind::Dollar => Self::Dollar,
            TokenKind::LAngle => Self::LAngle,
            TokenKind::RAngle => Self::RAngle,
            TokenKind::ColonEquals => Self::ColonEquals,
            TokenKind::DotDot => Self::DotDot,
            TokenKind::Dot => Self::Dot,
            TokenKind::TildeRAngle => Self::TildeRAngle,
            TokenKind::LBracket => Self::LBracket,
            TokenKind::RBracket => Self::RBracket,
            TokenKind::Question => Self::Question,
            TokenKind::Colon => Self::Colon,
            TokenKind::SemiColon => Self::SemiColon,
            TokenKind::Literal => Self::Literal,
            TokenKind::VariableRef => Self::VariableRef,
            TokenKind::InfixExpr => Self::InfixExpr,
            TokenKind::PrefixExpr => Self::PrefixExpr,
            TokenKind::ParenExpr => Self::ParenExpr,
            TokenKind::Root => Self::Root,
        }
    }
}

impl fmt::Display for SyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            SyntaxKind::Whitespace => "whitespace",
            SyntaxKind::VarIdent => "identifier",
            SyntaxKind::Number => "number",
            SyntaxKind::FunctionKeyword => "‘function’",
            SyntaxKind::Plus => "‘+’",
            SyntaxKind::Minus => "‘-’",
            SyntaxKind::Star => "‘*’",
            SyntaxKind::Slash => "‘/’",
            SyntaxKind::Equals => "‘=’",
            SyntaxKind::Hash => "‘#’",
            SyntaxKind::Percent => "‘%’",
            SyntaxKind::At => "‘@’",
            SyntaxKind::LParen => "‘(’",
            SyntaxKind::RParen => "‘)’",
            SyntaxKind::LBrace => "‘{’",
            SyntaxKind::RBrace => "‘}’",
            SyntaxKind::LBracket => "‘[’",
            SyntaxKind::RBracket => "‘]’",
            SyntaxKind::Comment => "comment",
            SyntaxKind::Dollar => "‘$’",
            SyntaxKind::LAngle => "‘<’",
            SyntaxKind::RAngle => "‘>’",
            SyntaxKind::ColonEquals => "‘:=’",
            SyntaxKind::DotDot => "‘..’",
            SyntaxKind::Dot => "‘.’",
            SyntaxKind::TildeRAngle => "‘~>’",
            SyntaxKind::Question => "‘?’",
            SyntaxKind::Colon => "‘:’",
            SyntaxKind::SemiColon => "‘;’",
            _ => unreachable!(format!("unreachable: found {:?}", &self)),
        })
    }
}
