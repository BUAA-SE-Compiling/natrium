use crate::span::Span;

pub fn Token(ty: TokenType, span: Span) -> Token {
    Token { ty, span }
}

pub struct Token {
    ty: TokenType,
    span: Span,
}

pub enum TokenType {
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
        match self.ty {
            TokenType::Space | TokenType::Comment => true,
            _ => false,
        }
    }
}
