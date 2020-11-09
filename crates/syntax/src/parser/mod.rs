#![allow(clippy::redundant_closure_call)]
pub mod err;

use std::iter::Peekable;

use crate::{
    prelude::{Span, P},
    Token,
};

use self::err::*;
use crate::ast::*;

pub struct Parser<L> {
    pub lexer: L,
}

macro_rules! expect {
    ($self:expr, $($pat:pat)|+) => {
        $self
            .next_if(|token| matches!(token, $($pat)|+))
            .map_err(|span| {
                ParseError::new(
                    ParseErrorKind::ExpectedPattern(stringify!($($pat)|+).to_owned()),
                    span,
                )
            })
    };
}

macro_rules! is_next {
    ($self:expr, $($pat:pat)|+) => {
        $self.peek().map_or(false, |token| matches!(token, $($pat)|+))
    };
}

macro_rules! separated {
    ( $parse:expr, $detect_sep:expr, $parse_sep:expr) => {{
        let first: Result<_, ParseError> = (|| $parse)();
        if let Ok(val) = first {
            let mut v = vec![val];
            while $detect_sep {
                let _ = $parse_sep;
                let next = (|| $parse)()?;
                v.push(next);
            }
            v
        } else {
            Vec::new()
        }
    }};
}

/// Combine `lhs` and `rhs` using `op`.
///
/// Requires `op` to be a binary operator, aka `op.is_binary_op() == true`
fn combine_expr(lhs: Expr, rhs: Expr, op: Token) -> Expr {
    match op {
        Token::Assign => Expr::Assign(AssignExpr {
            lhs: P::new(lhs),
            rhs: P::new(rhs),
            span: Span::default(),
        }),
        _ => {
            let binary_op = op
                .to_binary_op()
                .expect("A token passed in combine_expr should always be a binary operator");
            Expr::Binary(BinaryExpr {
                lhs: P::new(lhs),
                rhs: P::new(rhs),
                span: Span::default(),
                op: binary_op,
            })
        }
    }
}

macro_rules! repeated {
    ( $parse:expr, $detect_sep:expr) => {{
        let mut v = vec![];
        while !$detect_sep {
            let val: Result<_, ParseError> = (|| $parse)();
            v.push(val?);
        }
        v
    }};
}

impl<L> Parser<Peekable<L>>
where
    L: Iterator<Item = (Token, Span)>,
{
    pub fn new(lexer: L) -> Parser<Peekable<L>> {
        Parser {
            lexer: lexer.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        self.parse_program()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.lexer.peek().map(|(t, _)| t)
    }

    fn next_if<F>(&mut self, f: F) -> Result<(Token, Span), Option<Span>>
    where
        F: FnOnce(&Token) -> bool,
    {
        let peek = self.lexer.peek();
        match peek {
            Some((t, _)) if f(t) => Ok(self.lexer.next().unwrap()),
            Some((_, s)) => Err(Some(*s)),
            None => Err(None),
        }
    }

    fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut funcs = vec![];
        let mut decls = vec![];
        loop {
            if is_next!(self, Token::FnKw) {
                let res = self.parse_fn_decl()?;
                funcs.push(res);
            } else if is_next!(self, Token::LetKw) {
                let res = self.parse_decl()?;
                decls.push(res);
            } else if is_next!(self, Token::ConstKw) {
                let res = self.parse_const_decl()?;
                decls.push(res);
            } else {
                break;
            }
        }
        Ok(Program { decls, funcs })
    }

    fn parse_ident(&mut self) -> Result<Ident, ParseError> {
        let (name, name_span) = expect!(self, Token::Ident(_))?;
        Ok(Ident {
            span: name_span,
            name: name.get_ident_owned().unwrap(),
        })
    }

    fn parse_ty(&mut self) -> Result<TyDef, ParseError> {
        let (name, name_span) = expect!(self, Token::Ident(_))?;
        Ok(TyDef {
            span: name_span,
            name: name.get_ident_owned().unwrap(),
            params: None,
        })
    }

    fn parse_decl(&mut self) -> Result<DeclStmt, ParseError> {
        let (_, _start_span) = expect!(self, Token::LetKw)?;
        let ident = self.parse_ident()?;

        expect!(self, Token::Colon)?;
        let ty = self.parse_ty()?;

        let val = if is_next!(self, Token::Assign) {
            expect!(self, Token::Assign)?;
            Some(P::new(self.parse_expr()?))
        } else {
            None
        };

        let (_, _end_span) = expect!(self, Token::Semicolon)?;

        Ok(DeclStmt {
            is_const: false,
            name: ident,
            val,
            ty,
            span: _start_span + _end_span,
        })
    }

    fn parse_const_decl(&mut self) -> Result<DeclStmt, ParseError> {
        let (_, _start_span) = expect!(self, Token::ConstKw)?;
        let ident = self.parse_ident()?;

        expect!(self, Token::Colon)?;
        let ty = self.parse_ty()?;

        expect!(self, Token::Assign)?;
        let val = P::new(self.parse_expr()?);

        let (_, _end_span) = expect!(self, Token::Semicolon)?;

        Ok(DeclStmt {
            is_const: true,
            name: ident,
            val: Some(val),
            ty,
            span: _start_span + _end_span,
        })
    }

    fn parse_func_call(&mut self, func: Ident) -> Result<CallExpr, ParseError> {
        // FunctionCall -> Ident '(' (Expr (,Expr)* )? ')'
        expect!(self, Token::LParen)?;
        let params = separated!(
            self.parse_expr(),
            is_next!(self, Token::Comma),
            self.lexer.next()
        );
        expect!(self, Token::RParen)?;

        Ok(CallExpr {
            span: Span::default(),
            func,
            params,
        })
    }

    fn parse_item(&mut self) -> Result<Expr, ParseError> {
        // Item -> Ident | FunctionCall | Literal | '(' Expr ')'
        if is_next!(self, Token::Ident(_)) {
            let (ident, span) = self.lexer.next().unwrap();
            let ident = Ident {
                span,
                name: ident.get_ident_owned().unwrap(),
            };

            if is_next!(self, Token::LParen) {
                let call = self.parse_func_call(ident)?;
                Ok(Expr::Call(call))
            } else {
                Ok(Expr::Ident(ident))
            }
        } else if is_next!(self, Token::UIntLiteral(_)|Token::CharLiteral(_)) {
            let (num, span) = self.lexer.next().unwrap();
            Ok(Expr::Literal(LiteralExpr {
                span,
                kind: LiteralKind::Integer(num.get_uint().unwrap()),
            }))
        } else if is_next!(self, Token::FloatLiteral(_)) {
            let (num, span) = self.lexer.next().unwrap();
            Ok(Expr::Literal(LiteralExpr {
                span,
                kind: LiteralKind::Float(num.get_float().unwrap()),
            }))
        } else if is_next!(self, Token::StringLiteral(_)) {
            let (num, span) = self.lexer.next().unwrap();
            Ok(Expr::Literal(LiteralExpr {
                span,
                kind: LiteralKind::String(num.get_string_owned().unwrap()),
            }))
        } else if is_next!(self, Token::LParen) {
            expect!(self, Token::LParen)?;
            let expr = self.parse_expr()?;
            expect!(self, Token::RParen)?;
            Ok(expr)
        } else {
            Err(ParseError {
                kind: ParseErrorKind::ExpectedPattern(
                    "Literal or Identifier or parenthesis".into(),
                ),
                span: self
                    .lexer
                    .peek()
                    .map(|(_, s)| *s)
                    .unwrap_or_else(Span::eof)
                    .into(),
            })
        }
    }

    fn parse_unary_expr(&mut self) -> Result<Expr, ParseError> {
        // UExpr -> PreUOp* Item ProUOp*
        // PreUOp -> '+' | '-'
        // ProUOp -> 'as' TypeDef
        let mut prec_ops = vec![];
        while is_next!(self, Token::Minus) {
            prec_ops.push(self.lexer.next().unwrap().0)
        }

        let mut item = self.parse_item()?;
        for prec_op in prec_ops.drain(..).rev() {
            let unary_op = match prec_op {
                Token::Plus => UnaryOp::Pos,
                Token::Minus => UnaryOp::Neg,
                _ => unreachable!(),
            };
            item = Expr::Unary(UnaryExpr {
                span: Span::default(),
                op: unary_op,
                expr: P::new(item),
            });
        }

        while is_next!(self, Token::AsKw) {
            self.lexer.next();
            let ty = self.parse_ty()?;
            item = Expr::As(AsExpr {
                span: Span::default(),
                val: P::new(item),
                ty,
            })
        }

        Ok(item)
    }

    fn parse_expr_opg(&mut self, lhs: Expr, precedence: u32) -> Result<Expr, ParseError> {
        let mut lhs = lhs;
        while self.lexer.peek().map_or(false, |(x, _)| {
            x.is_binary_op() && x.precedence() >= precedence
        }) {
            // OPG
            let (op, _) = self.lexer.next().unwrap();
            let mut rhs = self.parse_unary_expr()?;

            while self.lexer.peek().map_or(false, |(x, _)| {
                x.is_binary_op()
                    && ((x.precedence() > op.precedence() && x.is_left_assoc())
                        || (x.precedence() == op.precedence() && !x.is_left_assoc()))
            }) {
                let (op, _) = self.lexer.peek().unwrap();
                let op_precedence = op.precedence();
                rhs = self.parse_expr_opg(rhs, op_precedence)?;
            }

            lhs = combine_expr(lhs, rhs, op);
        }
        Ok(lhs)
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let lhs = self.parse_unary_expr()?;
        self.parse_expr_opg(lhs, 0)
    }

    fn parse_expr_stmt(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_expr()?;
        expect!(self, Token::Semicolon)?;
        Ok(expr)
    }

    fn parse_block(&mut self) -> Result<BlockStmt, ParseError> {
        let (_, _start_span) = expect!(self, Token::LBrace)?;
        let vals = repeated!(self.parse_stmt(), is_next!(self, Token::RBrace));
        let (_, _end_span) = expect!(self, Token::RBrace)?;
        Ok(BlockStmt {
            stmts: vals,
            span: _start_span + _end_span,
        })
    }

    fn parse_if_stmt(&mut self) -> Result<IfStmt, ParseError> {
        let (_, mut span) = expect!(self, Token::IfKw)?;
        let mut conds = vec![];

        {
            // if block
            let cond = self.parse_expr()?;
            let if_blk = self.parse_block()?;

            span += if_blk.span;

            conds.push((P::new(cond), P::new(if_blk)));
        }

        let mut else_block = None;

        if is_next!(self, Token::ElseKw) {
            expect!(self, Token::ElseKw)?;

            while is_next!(self, Token::IfKw) {
                // else-if block
                expect!(self, Token::IfKw)?;
                let cond = self.parse_expr()?;
                let if_blk = self.parse_block()?;

                span += if_blk.span;

                conds.push((P::new(cond), P::new(if_blk)));
                expect!(self, Token::ElseKw)?;
            }

            if is_next!(self, Token::LBrace) {
                let else_blk = self.parse_block()?;

                span += else_blk.span;

                else_block = Some(P::new(else_blk));
            }
        }

        Ok(IfStmt {
            cond: conds,
            else_block,
            span,
        })
    }

    fn parse_while_stmt(&mut self) -> Result<WhileStmt, ParseError> {
        expect!(self, Token::WhileKw)?;
        let cond = self.parse_expr()?;
        let body = self.parse_block()?;
        let span = cond.span() + body.span;
        Ok(WhileStmt {
            cond: P::new(cond),
            body: P::new(body),
            span,
        })
    }

    fn parse_return_stmt(&mut self) -> Result<ReturnStmt, ParseError> {
        let (_, _start_span) = expect!(self, Token::ReturnKw)?;

        let val = if !is_next!(self, Token::Semicolon) {
            Some(P::new(self.parse_expr()?))
        } else {
            None
        };
        let (_, _end_span) = expect!(self, Token::Semicolon)?;

        Ok(ReturnStmt {
            val,
            span: _start_span + _end_span,
        })
    }

    fn parse_break_stmt(&mut self) -> Result<Span, ParseError> {
        let (_, span) = expect!(self, Token::BreakKw)?;
        expect!(self, Token::Semicolon)?;
        Ok(span)
    }

    fn parse_continue_stmt(&mut self) -> Result<Span, ParseError> {
        let (_, span) = expect!(self, Token::ContinueKw)?;
        expect!(self, Token::Semicolon)?;
        Ok(span)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        let val = if is_next!(self, Token::ConstKw) {
            Stmt::Decl(self.parse_const_decl()?)
        } else if is_next!(self, Token::LetKw) {
            Stmt::Decl(self.parse_decl()?)
        } else if is_next!(self, Token::LBrace) {
            Stmt::Block(self.parse_block()?)
        } else if is_next!(self, Token::IfKw) {
            Stmt::If(self.parse_if_stmt()?)
        } else if is_next!(self, Token::WhileKw) {
            Stmt::While(self.parse_while_stmt()?)
        } else if is_next!(self, Token::BreakKw) {
            Stmt::Break(self.parse_break_stmt()?)
        } else if is_next!(self, Token::ContinueKw) {
            Stmt::Continue(self.parse_continue_stmt()?)
        } else if is_next!(self, Token::ReturnKw) {
            Stmt::Return(self.parse_return_stmt()?)
        } else {
            Stmt::Expr(self.parse_expr_stmt()?)
        };
        Ok(val)
    }

    fn parse_fn_decl(&mut self) -> Result<FuncStmt, ParseError> {
        let (_, _start_span) = expect!(self, Token::FnKw)?;
        let fn_name = self.parse_ident()?;

        expect!(self, Token::LParen)?;
        let params = separated!(
            {
                let param_name = self.parse_ident()?;
                expect!(self, Token::Colon)?;
                let param_ty = self.parse_ty()?;
                Ok(FuncParam {
                    name: param_name,
                    ty: param_ty,
                })
            },
            is_next!(self, Token::Comma),
            expect!(self, Token::Comma)
        );
        expect!(self, Token::RParen)?;

        expect!(self, Token::Arrow)?;
        let ret_ty = self.parse_ty()?;

        let body = self.parse_block()?;

        let span = _start_span + body.span;

        Ok(FuncStmt {
            name: fn_name,
            params,
            ret_ty,
            body,
            span,
        })
    }
}

impl Token {
    pub fn is_binary_op(&self) -> bool {
        matches!(
            self,
            Token::Plus
                | Token::Minus
                | Token::Mul
                | Token::Div
                | Token::Assign
                | Token::Eq
                | Token::Neq
                | Token::Lt
                | Token::Gt
                | Token::Le
                | Token::Ge
        )
    }

    pub fn precedence(&self) -> u32 {
        match self {
            Token::Plus => 10,
            Token::Minus => 10,
            Token::Mul => 20,
            Token::Div => 20,
            Token::Assign => 1,
            Token::Eq => 2,
            Token::Neq => 2,
            Token::Lt => 2,
            Token::Gt => 2,
            Token::Le => 2,
            Token::Ge => 2,
            _ => unreachable!("Precedence should only be called by binary operators"),
        }
    }

    pub fn is_left_assoc(&self) -> bool {
        match self {
            Token::Plus
            | Token::Minus
            | Token::Mul
            | Token::Div
            | Token::Eq
            | Token::Neq
            | Token::Lt
            | Token::Gt
            | Token::Le
            | Token::Ge => true,
            Token::Assign => false,
            _ => unreachable!("Method should only be called by binary operators"),
        }
    }

    pub fn to_binary_op(&self) -> Option<BinaryOp> {
        match self {
            Token::Plus => Some(BinaryOp::Add),
            Token::Minus => Some(BinaryOp::Sub),
            Token::Mul => Some(BinaryOp::Mul),
            Token::Div => Some(BinaryOp::Div),
            Token::Eq => Some(BinaryOp::Eq),
            Token::Neq => Some(BinaryOp::Neq),
            Token::Lt => Some(BinaryOp::Lt),
            Token::Gt => Some(BinaryOp::Gt),
            Token::Le => Some(BinaryOp::Le),
            Token::Ge => Some(BinaryOp::Ge),
            _ => None,
        }
    }
}
