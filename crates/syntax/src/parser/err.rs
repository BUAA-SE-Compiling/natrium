use crate::{prelude::Span, Token};

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub span: Option<Span>,
}

impl ParseError {
    pub fn new_span(kind: ParseErrorKind, span: Span) -> Self {
        Self {
            kind,
            span: Some(span),
        }
    }

    pub fn new_none(kind: ParseErrorKind) -> Self {
        Self { kind, span: None }
    }

    pub fn new(kind: ParseErrorKind, span: Option<Span>) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug)]
pub enum ParseErrorKind {
    ExpectToken(Token),
    ExpectedPattern(String),
    UnexpectedEof,
    Dummy,
}
