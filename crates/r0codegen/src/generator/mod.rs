use std::cell::RefCell;

use crate::ty::Ty;
use crate::{
    code::BasicBlock,
    err::{CompileError, CompileErrorKind},
};
use crate::{
    code::{Cond, JumpInst},
    scope::{Scope, Symbol, SymbolIdGenerator},
};
use r0syntax::{ast, span::Span, util::Mut, util::MutWeak};
use r0vm::{opcodes::Op, s0};

static RET_VAL_KEY: &str = "$ret";

type CompileResult<T> = std::result::Result<T, CompileError>;
type BB = usize;

pub fn compile(tree: &ast::Program) -> CompileResult<s0::S0> {
    let global_sym_gen = RefCell::new(SymbolIdGenerator::new());
    let mut global_scope = Scope::new(&global_sym_gen);
    for decl in &tree.decls {
        add_decl_scope(decl, &mut global_scope)?;
    }
    for func in &tree.funcs {
        compile_func(func, &global_scope)?;
    }
    todo!()
}

// fn compile_start_func()->CompileResult<s0::FnDef>{
//     let start_func = ast::FuncStmt {
//         name: ast::Ident {
//             val: "_start".into(),
//             span: Span::default(),
//         },
//         params: vec![],
//         ret_ty: ast::TyDef {
//             name: "void".into(),
//             params: None,
//         },
//         body: ast::BlockStmt {
//             stmts: tree
//                 .decls
//                 .iter()
//                 .cloned()
//                 .map(|x| ast::Stmt::Decl(x))
//                 .collect(),
//         },
//     };

// }

fn compile_func(func: &ast::FuncStmt, global_scope: &Scope) -> CompileResult<s0::FnDef> {
    let mut fc = FuncCodegen::new(func, global_scope);
    fc.compile()
}

struct FuncCodegen<'f> {
    func: &'f ast::FuncStmt,
    global_scope: &'f Scope<'f>,
    basic_blocks: Vec<BasicBlock>,
}

impl<'f> FuncCodegen<'f> {
    pub fn new(func: &'f ast::FuncStmt, scope: &'f Scope<'f>) -> FuncCodegen<'f> {
        FuncCodegen {
            func,
            global_scope: scope,
            basic_blocks: vec![],
        }
    }

    pub fn compile(&mut self) -> CompileResult<s0::FnDef> {
        self.compile_func()
    }

    fn new_bb(&mut self) -> BB {
        let bb_id = self.basic_blocks.len();
        self.basic_blocks.push(BasicBlock::new());
        bb_id
    }

    fn append_code(&mut self, bb_id: BB, code: Op) {
        if let Some(bb) = self.basic_blocks.get_mut(bb_id) {
            bb.code.push(code);
        } else {
            panic!("Non-existent basic block: {}", bb_id);
        }
    }

    fn set_jump(&mut self, bb_id: BB, jump: JumpInst) {
        if let Some(bb) = self.basic_blocks.get_mut(bb_id) {
            bb.jump = jump;
        } else {
            panic!("Non-existent basic block: {}", bb_id);
        }
    }

    fn compile_func(&mut self) -> CompileResult<s0::FnDef> {
        let mut scope = Scope::new_with_parent(self.global_scope);

        scope.insert(
            RET_VAL_KEY.into(),
            Symbol::new(get_ty(&self.func.ret_ty)?, false),
        );

        for param in &self.func.params {
            scope.insert(
                param.name.val.clone(),
                Symbol::new(get_ty(&param.ty)?, false),
            );
        }

        let start_bb = self.new_bb();

        self.compile_block(&self.func.body, start_bb, &scope)?;

        todo!()
    }

    fn compile_block(
        &mut self,
        blk: &ast::BlockStmt,
        bb_id: BB,
        scope: &Scope,
    ) -> CompileResult<BB> {
        let mut block_scope = Scope::new_with_parent(scope);
        let mut cur_bb_id = bb_id;
        for stmt in &blk.stmts {
            cur_bb_id = self.compile_stmt(stmt, cur_bb_id, &mut block_scope)?;
        }
        Ok(cur_bb_id)
    }

    fn compile_stmt(
        &mut self,
        stmt: &ast::Stmt,
        bb_id: BB,
        scope: &mut Scope,
    ) -> CompileResult<BB> {
        match stmt {
            ast::Stmt::Block(blk) => self.compile_block(blk, bb_id, scope),
            ast::Stmt::While(stmt) => self.compile_while(stmt, bb_id, scope),
            ast::Stmt::If(stmt) => self.compile_if(stmt, bb_id, scope),
            ast::Stmt::Expr(expr) => {
                self.compile_expr(expr, bb_id, scope)?;
                Ok(bb_id)
            }
            ast::Stmt::Decl(stmt) => self.compile_decl(stmt, bb_id, scope),
            ast::Stmt::Return(stmt) => self.compile_return(stmt, bb_id, scope),
        }
    }

    fn compile_while(
        &mut self,
        stmt: &ast::WhileStmt,
        bb_id: BB,
        scope: &Scope,
    ) -> CompileResult<BB> {
        /*
         *               v----------------------------------\
         * begin --> [bb:A cond] -true--> [bb:B body bb:C] -/  /--> [bb:D next]
         *               \-false------------------------------/
         */

        let cond_bb = self.new_bb();
        let body_bb = self.new_bb();
        let next_bb = self.new_bb();

        self.compile_expr(stmt.cond.as_ref(), cond_bb, scope)?;
        self.set_jump(bb_id, JumpInst::Jump(cond_bb));
        self.set_jump(cond_bb, JumpInst::JumpIf(Cond::Eq, body_bb, next_bb));

        let body_end_bb = self.compile_block(stmt.body.as_ref(), body_bb, scope)?;
        self.set_jump(body_end_bb, JumpInst::Jump(cond_bb));

        Ok(next_bb)
    }

    fn compile_if(&mut self, stmt: &ast::IfStmt, bb_id: BB, scope: &Scope) -> CompileResult<BB> {
        /*
         * begin --> [bb:A1 cond] -true--> [bb:B1 body] -\
         *           |false                              |
         *           V                                   |
         *           [bb:A2 cond] -true--> [bb:B2 body] -|
         *           |false                              |
         *           ...                                 |
         *           V                                   |
         *           [bb:C else_body]----------------------> end
         */
        let mut body_bbs = vec![];
        let mut cond_bbs = vec![];
        let end_bb = self.new_bb();

        for (cond, body) in &stmt.cond {
            let cond_bb = self.new_bb();
            self.compile_expr(cond.as_ref(), cond_bb, scope)?;
            cond_bbs.push(cond_bb);

            let body_bb = self.new_bb();
            body_bbs.push(cond_bb);
            let body_end_bb = self.compile_block(body.as_ref(), body_bb, scope)?;

            self.set_jump(body_end_bb, JumpInst::Jump(end_bb));
        }

        let else_bb = if let Some(b) = &stmt.else_block {
            let else_bb = self.new_bb();
            let else_end_bb = self.compile_block(b, else_bb, scope)?;
            self.set_jump(else_end_bb, JumpInst::Jump(end_bb));
            else_end_bb
        } else {
            end_bb
        };

        let cond_iter = cond_bbs.iter().cloned().zip(
            body_bbs.into_iter().zip(
                cond_bbs
                    .iter()
                    .skip(1)
                    .cloned()
                    .chain(std::iter::once(else_bb)),
            ),
        );

        for (cond, (bb_true, bb_false)) in cond_iter {
            self.set_jump(cond, JumpInst::JumpIf(Cond::, bb_true, bb_false));
        }

        Ok(end_bb)
    }

    fn compile_decl(
        &mut self,
        stmt: &ast::DeclStmt,
        bb_id: BB,
        scope: &mut Scope,
    ) -> CompileResult<BB> {
        add_decl_scope(stmt, scope)?;
        Ok(bb_id)
    }

    fn compile_return(
        &mut self,
        stmt: &ast::ReturnStmt,
        bb_id: BB,
        scope: &Scope,
    ) -> CompileResult<BB> {
        todo!()
    }

    fn compile_expr(&mut self, expr: &ast::Expr, bb_id: BB, scope: &Scope) -> CompileResult<Ty> {
        todo!()
    }
}

fn add_decl_scope(decl: &ast::DeclStmt, scope: &mut Scope) -> CompileResult<()> {
    let ty = get_ty(&decl.ty)?;
    let name = decl.name.val.clone();

    let symbol = Symbol::new(ty, decl.is_const);

    let symbol = scope.insert(name, symbol);
    if symbol.is_none() {
        return Err(CompileError {
            kind: CompileErrorKind::DuplicateSymbol(decl.name.val.as_str().into()),
            span: Some(decl.name.span),
        });
    }

    Ok(())
}

fn get_ty(ty: &ast::TyDef) -> CompileResult<Ty> {
    Ok(match ty.name.as_str() {
        "int" => Ty::Int,
        "double" => Ty::Double,
        "void" => Ty::Void,
        _ => {
            return Err(CompileError {
                kind: CompileErrorKind::UnknownType(ty.name.as_str().into()),
                span: None,
            })
        }
    })
}
