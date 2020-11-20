//! This crate lists the common AST items inside R0.
//!
//! For the pointer type, see `crate::util::{P, Mut}`

use crate::{span::Span, util::P};
#[cfg(feature = "serde_impl")]
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct Program {
    pub decls: Vec<DeclStmt>,
    pub funcs: Vec<FuncStmt>,
}

pub trait AstNode {
    fn span(&self) -> Span;
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct FuncStmt {
    pub span: Span,
    pub name: Ident,
    pub params: Vec<FuncParam>,
    pub ret_ty: TyDef,
    pub body: BlockStmt,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct FuncParam {
    pub is_const: bool,
    pub name: Ident,
    pub ty: TyDef,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub enum Stmt {
    Block(BlockStmt),
    While(WhileStmt),
    If(IfStmt),
    Expr(Expr),
    Decl(DeclStmt),
    Return(ReturnStmt),
    Break(Span),
    Continue(Span),
    Empty(Span),
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Stmt::Block(i) => i.span,
            Stmt::While(i) => i.span,
            Stmt::If(i) => i.span,
            Stmt::Expr(i) => i.span(),
            Stmt::Decl(i) => i.span,
            Stmt::Return(i) => i.span,
            Stmt::Break(s) => *s,
            Stmt::Continue(s) => *s,
            Stmt::Empty(s) => *s,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct DeclStmt {
    pub is_const: bool,
    pub name: Ident,
    pub ty: TyDef,
    pub val: Option<P<Expr>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct ReturnStmt {
    pub val: Option<P<Expr>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct TyDef {
    pub span: Span,
    pub name: SmolStr,
    pub params: Option<Vec<TyDef>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct BlockStmt {
    pub span: Span,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct WhileStmt {
    pub span: Span,
    pub cond: P<Expr>,
    pub body: P<BlockStmt>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct IfStmt {
    pub span: Span,
    pub cond: P<Expr>,
    pub if_block: P<BlockStmt>,
    pub else_block: IfElseBlock,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub enum IfElseBlock {
    None,
    If(P<IfStmt>),
    Block(P<BlockStmt>),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub enum Expr {
    Ident(Ident),
    Assign(AssignExpr),
    As(AsExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Call(CallExpr),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Ident(x) => x.span,
            Expr::Assign(x) => x.span,
            Expr::As(x) => x.span,
            Expr::Literal(x) => x.span,
            Expr::Unary(x) => x.span,
            Expr::Binary(x) => x.span,
            Expr::Call(x) => x.span,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct LiteralExpr {
    pub span: Span,
    pub kind: LiteralKind,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub enum LiteralKind {
    Integer(u64),
    Float(f64),
    String(String),
    Char(char),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct UnaryExpr {
    pub span: Span,
    pub op: UnaryOp,
    pub expr: P<Expr>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct AssignExpr {
    pub span: Span,
    pub allow_assign_const: bool,
    pub lhs: P<Expr>,
    pub rhs: P<Expr>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct AsExpr {
    pub span: Span,
    pub val: P<Expr>,
    pub ty: TyDef,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct BinaryExpr {
    pub span: Span,
    pub op: BinaryOp,
    pub lhs: P<Expr>,
    pub rhs: P<Expr>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct CallExpr {
    pub span: Span,
    pub func: Ident,
    pub params: Vec<Expr>,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub enum UnaryOp {
    Neg,
    Pos,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct Ident {
    pub span: Span,
    pub name: SmolStr,
}
