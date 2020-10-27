use r0syntax::span::Span;

pub struct CompileError {
    pub kind: CompileErrorKind,
    pub span: Option<Span>,
}

pub enum CompileErrorKind {
    UnknownType(String),
    DuplicateSymbol(String),
}
