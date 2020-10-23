use logos::Logos;

use crate::{prelude::Span, token::Token};

pub type Lexer<'src> = logos::Lexer<'src, Token>;

pub fn lexer(s: &str) -> Lexer {
    Token::lexer(s)
}

pub fn spanned_lexer<'s>(s: &'s str) -> impl Iterator<Item = (Token, Span)> + 's {
    Token::lexer(s)
        .spanned()
        .map(|(t, s)| (t, crate::prelude::Span::new_idx(s.start, s.end)))
}
