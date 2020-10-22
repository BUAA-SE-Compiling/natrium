//! This crate lists the common AST items inside R0.
//!
//! For the pointer type, see `crate::util::{P, Mut}`

use indexmap::IndexMap;
use smol_str::SmolStr;

use crate::{
    span::Span,
    ty::TyKind,
    util::MutWeak,
    util::{Mut, P},
};

pub trait AstNode {
    fn span(&self) -> Span;
}

#[derive(Debug, Clone)]
pub struct FuncStmt {
    pub name: SmolStr,
    pub body: Stmt,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(BlockStmt),
    Loop(LoopStmt),
    If(IfStmt),
    Expr(Expr),
    Decl,
}

#[derive(Debug, Clone)]
pub struct DeclStmt {
    pub name: Ident,
    pub val: Option<Expr>,
    pub ty: TyDef,
}

#[derive(Debug, Clone)]
pub struct TyDef {
    pub name: SmolStr,
    pub params: Option<Vec<TyDef>>,
}

#[derive(Debug, Clone)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct LoopStmt {
    pub cond: P<Expr>,
    pub body: P<Stmt>,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub cond: P<Expr>,
    pub if_true: P<Stmt>,
    pub if_false: P<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(Ident),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Call(CallExpr),
}

#[derive(Debug, Clone)]
pub struct LiteralExpr {
    pub span: Span,
    pub kind: LiteralKind,
}

#[derive(Debug, Clone)]
pub enum LiteralKind {
    Integer(u64),
    Float(f64),
    String(String),
    Char(char),
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub span: Span,
    pub op: UnaryOp,
    pub expr: P<Expr>,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub span: Span,
    pub op: BinaryOp,
    pub lhs: P<Expr>,
    pub rhs: P<Expr>,
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub span: Span,
    pub func: Ident,
    pub params: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Pos,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Gt,
    Lt,
    Ge,
    Le,
    Eq,
    Neq,
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub span: Span,
    pub idx: SmolStr,
}
