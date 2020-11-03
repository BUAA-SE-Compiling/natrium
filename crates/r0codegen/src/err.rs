use r0syntax::span::Span;

#[derive(Debug, Clone)]
pub struct CompileError {
    pub kind: CompileErrorKind,
    pub span: Option<Span>,
}

#[allow(non_snake_case)]
pub fn CompileError(kind: CompileErrorKind, span: Option<Span>) -> CompileError {
    CompileError { kind, span }
}

#[derive(Debug, Clone)]
pub enum CompileErrorKind {
    UnknownType(String),
    NoSuchSymbol(String),
    DuplicateSymbol(String),
    VoidTypeVariable,
    TypeMismatch {
        expected: String,
        got: Option<String>,
    },
    NotLValue,
    InvalidCalculation(String),
    FuncParamSizeMismatch(usize, usize),
    NotAllRoutesReturn,
}

pub trait WithSpan {
    fn with_span(self, span: Span) -> Self;
}

impl WithSpan for CompileError {
    fn with_span(mut self, span: Span) -> CompileError {
        self.span = Some(span);
        self
    }
}

impl<T> WithSpan for Result<T, CompileError> {
    fn with_span(self, span: Span) -> Result<T, CompileError> {
        self.map_err(|e| e.with_span(span))
    }
}
