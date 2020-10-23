//! This crate lists the common AST items inside R0.
//!
//! For the pointer type, see `crate::util::{P, Mut}`

use crate::{span::Span, util::P};
use smol_str::SmolStr;

#[derive(Debug, Clone)]
pub struct Program {
    pub decls: Vec<DeclStmt>,
    pub funcs: Vec<FuncStmt>,
}

pub trait AstNode {
    fn span(&self) -> Span;
}

#[derive(Debug, Clone)]
pub struct FuncStmt {
    pub name: Ident,
    pub params: Vec<FuncParam>,
    pub ret_ty: TyDef,
    pub body: BlockStmt,
}

#[derive(Debug, Clone)]
pub struct FuncParam {
    pub name: Ident,
    pub ty: TyDef,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(BlockStmt),
    While(WhileStmt),
    If(IfStmt),
    Expr(Expr),
    Decl(DeclStmt),
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
pub struct WhileStmt {
    pub cond: P<Expr>,
    pub body: P<BlockStmt>,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub cond: Vec<(P<Expr>, P<BlockStmt>)>,
    pub else_block: Option<P<BlockStmt>>,
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
    pub val: SmolStr,
}
