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
    ($self:expr, $pat:pat) => {
        $self
            .next_if(|token| matches!(token, $pat))
            .map_err(|span| {
                ParseError::new(
                    ParseErrorKind::ExpectedPattern(stringify!($pat).to_owned()),
                    span,
                )
            })
    };
}

macro_rules! is_next {
    ($self:expr, $pat:pat) => {
        $self.peek().map_or(false, |token| matches!(token, $pat))
    };
}

macro_rules! separated {
    ( $parse:expr, $parse_separator:expr) => {{
        let first: Result<_, ParseError> = (|| $parse)();
        if let Ok(val) = first {
            let mut v = vec![val];
            while $parse_separator.is_ok() {
                let next = (|| $parse)()?;
                v.push(next);
            }
            v
        } else {
            Vec::new()
        }
    }};
}

macro_rules! repeated {
    ( $parse:expr, $parse_delimiter:expr) => {{
        let mut v = vec![];
        while (|| $parse_delimiter)().is_err() {
            let val: Result<_, ParseError> = (|| $parse)();
            v.push(val?)
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
        self.lexer.peek().map(|(t, s)| t)
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
            val: name.get_ident_owned().unwrap(),
        })
    }

    fn parse_ty(&mut self) -> Result<TyDef, ParseError> {
        let (name, _name_span) = expect!(self, Token::Ident(_))?;
        Ok(TyDef {
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
            Some(self.parse_expr()?)
        } else {
            None
        };

        Ok(DeclStmt {
            name: ident,
            val,
            ty,
        })
    }

    fn parse_const_decl(&mut self) -> Result<DeclStmt, ParseError> {
        let (_, _start_span) = expect!(self, Token::ConstKw)?;
        let ident = self.parse_ident()?;

        expect!(self, Token::Colon)?;
        let ty = self.parse_ty()?;

        expect!(self, Token::Assign)?;
        let val = self.parse_expr()?;

        Ok(DeclStmt {
            name: ident,
            val: Some(val),
            ty,
        })
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        todo!()
    }

    fn parse_expr_stmt(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_expr()?;
        expect!(self, Token::Semicolon)?;
        Ok(expr)
    }

    fn parse_block(&mut self) -> Result<BlockStmt, ParseError> {
        expect!(self, Token::LBrace)?;
        let vals = repeated!(self.parse_stmt(), expect!(self, Token::RBrace));
        Ok(BlockStmt { stmts: vals })
    }

    fn parse_if_stmt(&mut self) -> Result<IfStmt, ParseError> {
        expect!(self, Token::IfKw)?;
        let mut conds = vec![];

        {
            // if block
            let cond = self.parse_expr()?;
            let if_blk = self.parse_block()?;
            conds.push((P::new(cond), P::new(if_blk)));
        }

        let mut else_block = None;

        if is_next!(self, Token::ElseKw) {
            expect!(self, Token::ElseKw)?;

            while is_next!(self, Token::IfKw) {
                // else-if block
                let cond = self.parse_expr()?;
                let if_blk = self.parse_block()?;
                conds.push((P::new(cond), P::new(if_blk)));
                expect!(self, Token::ElseKw)?;
            }

            if is_next!(self, Token::LBrace) {
                else_block = Some(P::new(self.parse_block()?));
            }
        }

        Ok(IfStmt {
            cond: conds,
            else_block,
        })
    }

    fn parse_while_stmt(&mut self) -> Result<WhileStmt, ParseError> {
        expect!(self, Token::WhileKw)?;
        let cond = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(WhileStmt {
            cond: P::new(cond),
            body: P::new(body),
        })
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
            { expect!(self, Token::Comma) }
        );
        expect!(self, Token::RParen)?;

        expect!(self, Token::Arrow)?;
        let ret_ty = self.parse_ty()?;

        let body = self.parse_block()?;

        Ok(FuncStmt {
            name: fn_name,
            params,
            ret_ty,
            body,
        })
    }
}
