pub mod err;
use crate::{span::Span, token::*};
use err::*;
use std::iter::Peekable;
use std::str::CharIndices;

type LexResult<T> = Result<T, LexError>;

pub struct Lexer<'src> {
    src: Peekable<CharIndices<'src>>,
}

impl<'src> Lexer<'src> {
    pub fn new(src: CharIndices<'src>) -> Lexer<'src> {
        Lexer {
            src: src.peekable(),
        }
    }

    /// Lex exactly one token. Any error token is preserved
    /// as `TokenKind::Error`, only EOF is returned as None.
    pub fn lex_one(&mut self) -> Option<Token> {
        match self.peek_char()? {
            '0'..='9' => self.lex_number(),
            '_' | 'a'..='z' | 'A'..='Z' => self.lex_ident_or_kw(),
            '"' => self.lex_str(),
            '+' | '-' | '*' | '/' | '=' | '<' | '>' | '!' | '^' => self.lex_ops(),
            _ => Some(self.skip_error()),
        }
    }

    fn next_char(&mut self) -> Option<(usize, char)> {
        self.src.next()
    }

    fn peek_char(&mut self) -> Option<char> {
        self.src.peek().map(|(_idx, ch)| *ch)
    }

    fn lex_str(&mut self) -> Option<Token> {
        todo!("Lex string literal")
    }

    fn lex_ident_or_kw(&mut self) -> Option<Token> {
        todo!("Lex identifier or keyword")
    }

    fn lex_number(&mut self) -> Option<Token> {
        todo!("Lex number")
    }

    fn lex_ops(&mut self) -> Option<Token> {
        todo!("Lex operator")
    }

    fn skip_error(&mut self) -> Token {
        todo!("Skip the current error token")
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.lex_one()
    }
}
