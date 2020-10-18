pub mod err;
use crate::{span::Span, token::*};
use err::*;
use std::iter::Peekable;
use std::str::CharIndices;

type LexResult<T> = Result<T, LexError>;

pub struct Lexer<'src> {
    src: Peekable<CharIndices<'src>>,
    pos: usize,
}

const SUBROUTINE_ASSERT: &'static str = "Any call to lex subroutines must not be EOF";

impl<'src> Lexer<'src> {
    pub fn new(src: CharIndices<'src>) -> Lexer<'src> {
        Lexer {
            src: src.peekable(),
            pos: 0,
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
            x if char::is_whitespace(x) => self.lex_spaces(),
            _ => Some(self.skip_error()),
        }
    }

    fn next_char(&mut self) -> Option<(usize, char)> {
        let (idx, ch) = self.src.next()?;
        self.pos = idx;
        Some((idx, ch))
    }

    fn peek_char(&mut self) -> Option<char> {
        self.src.peek().map(|(_idx, ch)| *ch)
    }

    fn cur_pos(&mut self) -> Option<usize> {
        self.src.peek().map(|(idx, _ch)| *idx)
    }

    fn lex_str(&mut self) -> Option<Token> {
        let (start_idx, quote) = self.next_char().expect(SUBROUTINE_ASSERT);
        assert!(quote == '"', "String literal must start with double quote");

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

    fn lex_spaces(&mut self) -> Option<Token> {
        let start = self.cur_pos().unwrap();

        while self.src.peek().map_or(false, |(_, c)| c.is_whitespace()) {
            self.src.next();
        }

        let end = self.cur_pos().unwrap();
        Some(token(TokenKind::Space, Span::new(0, start, end - start)))
    }

    fn skip_error(&mut self) -> Token {
        todo!("Skip the current error token")
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let mut next = self.lex_one();
        while next.as_ref().map_or(false, |t| t.is_skipable()) {
            next = self.lex_one();
        }
        next
    }
}

struct UnescapeStringIterator<T> {
    src: T,
}

impl<T, A> Iterator for UnescapeStringIterator<T>
where
    T: PeekableIterator<Item = A>,
    A: AsChar,
{
    type Item = A;
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.src.next()?;
        let ch = x.as_char();

        if ch == '\\' {
            // escaped character
            let next = self.src.next()?;
            let next_ch = next.as_char();
            let tgt = match next_ch {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '\'' => '\'',
                '"' => '"',
                _ => return None,
            };
            Some(next.replace_char(tgt))
        } else if ch == '"' {
            None
        } else {
            Some(x)
        }
    }
}

trait AsChar {
    fn as_char(&self) -> char;
    fn replace_char(self, ch: char) -> Self;
}

trait PeekableIterator: Iterator {
    fn peek_next(&mut self) -> Option<&Self::Item>;
}
impl<T> PeekableIterator for Peekable<T>
where
    T: Iterator,
{
    fn peek_next(&mut self) -> Option<&Self::Item> {
        self.peek()
    }
}

impl AsChar for char {
    fn as_char(&self) -> char {
        *self
    }
    fn replace_char(self, ch: char) -> Self {
        ch
    }
}

impl<T> AsChar for (T, char) {
    fn as_char(&self) -> char {
        self.1
    }
    fn replace_char(mut self, ch: char) -> Self {
        self.1 = ch;
        self
    }
}
