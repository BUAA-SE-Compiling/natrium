pub mod err;
use err::*;

use crate::{span::Span, token::*};

pub struct Lexer {}

impl Lexer {
    pub fn lex(&mut self) {}
    fn lex_kw_fn(&mut self, i: &str) -> LexResult<Token> {}
}
