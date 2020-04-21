//! This crate lists the common AST items inside R0.
//!
//! For the pointer type, see `crate::util::{P, Mut}`

use crate::{
    span::Span,
    util::{Mut, P},
};

pub trait AstNode {
    fn span(&self) -> Span;
}

pub enum Stmt {
    Loop,
    Decl,
    Expr(Expr),
}

pub enum Expr {
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Ident(Ident),
}

pub struct LiteralExpr {
    pub span: Span,
    pub kind: LiteralKind,
}

pub enum LiteralKind {
    Integer(),
    Float(),
    String(String),
    Char(char),
}

pub struct UnaryExpr {
    pub span: Span,
    pub op: UnaryOp,
    pub expr: P<Expr>,
}

pub struct BinaryExpr {
    pub span: Span,
    pub op: BinaryOp,
    pub lhs: P<Expr>,
    pub rhs: P<Expr>,
}

pub enum UnaryOp {}

pub enum BinaryOp {}

pub struct Ident {
    pub span: Span,
    pub idx: Name,
}

/// An reference to a name string stored inside parsing context. Index 0 is not
/// used.
pub struct Name(std::num::NonZeroUsize);
