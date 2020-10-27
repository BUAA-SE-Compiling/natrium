/// A span representing a section of source file
pub mod span;

/// Utilities
pub mod util;

/// Lexer for r0 tokens
pub mod lexer;
/// Models of r0 tokens
pub mod token;

/// Models of the abstract syntax tree.
pub mod ast;
/// Parser for r0 programs
pub mod parser;

pub use lexer::Lexer;
pub use token::Token;

mod prelude {
    pub use crate::span::Span;
    pub use crate::util::{Mut, MutWeak, P};
}
