use crate::span::Span;

pub fn token(kind: TokenKind, span: Span) -> Token {
    Token { kind, span }
}

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub enum TokenKind {
    FnKw,
    LetKw,

    IntLiteral,
    FloatLiteral,
    Ident,

    // Empty stuff
    Space,
    Comment,

    // Error token
    Error,
}

impl Token {
    pub fn is_skipable(&self) -> bool {
        matches!(self.kind, TokenKind::Space | TokenKind::Comment)
    }
}
