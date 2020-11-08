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
    pub span: Span,
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
    Return(ReturnStmt),
    Break(Span),
    Continue(Span),
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
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeclStmt {
    pub is_const: bool,
    pub name: Ident,
    pub ty: TyDef,
    pub val: Option<P<Expr>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub val: Option<P<Expr>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct TyDef {
    pub span: Span,
    pub name: SmolStr,
    pub params: Option<Vec<TyDef>>,
}

#[derive(Debug, Clone)]
pub struct BlockStmt {
    pub span: Span,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub span: Span,
    pub cond: P<Expr>,
    pub body: P<BlockStmt>,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub span: Span,
    pub cond: Vec<(P<Expr>, P<BlockStmt>)>,
    pub else_block: Option<P<BlockStmt>>,
}

#[derive(Debug, Clone)]
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
pub struct AssignExpr {
    pub span: Span,
    pub lhs: P<Expr>,
    pub rhs: P<Expr>,
}

#[derive(Debug, Clone)]
pub struct AsExpr {
    pub span: Span,
    pub val: P<Expr>,
    pub ty: TyDef,
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

#[derive(Debug, Copy, Clone)]
pub enum UnaryOp {
    Neg,
    Pos,
}

#[derive(Debug, Copy, Clone)]
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
    pub name: SmolStr,
}
