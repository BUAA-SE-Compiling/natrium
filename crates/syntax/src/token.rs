use crate::span::Span;

pub fn Token(ty: TokenType, span: Span) -> Token {
    Token { ty, span }
}

pub struct Token {
    ty: TokenType,
    span: Span,
}

pub enum TokenType {
    Fn,
}
