pub mod err;
pub mod scope;
pub mod ty;

use std::cell::RefCell;

use err::{CompileError, CompileErrorKind};
use r0syntax::{ast, util::Mut, util::MutWeak};
use r0vm::s0;
use scope::{Scope, Symbol, SymbolIdGenerator};
use ty::Ty;

static RET_VAL_KEY: &str = "$ret";

type Result<T> = std::result::Result<T, CompileError>;

pub fn compile(tree: &ast::Program) -> Result<s0::S0> {
    let global_sym_gen = RefCell::new(SymbolIdGenerator::new());
    let mut global_scope = Scope::new(&global_sym_gen);
    for decl in &tree.decls {
        compile_decl(decl, &mut global_scope)?;
    }
    for func in &tree.funcs {
        compile_func(func, &global_scope)?;
    }
    todo!()
}

fn compile_func(func: &ast::FuncStmt, global_scope: &Scope) -> Result<s0::FnDef> {
    let mut scope = Scope::new_with_parent(global_scope);

    scope.insert(
        RET_VAL_KEY.into(),
        Symbol::new(get_ty(&func.ret_ty)?, false),
    );

    for param in &func.params {
        scope.insert(
            param.name.val.clone(),
            Symbol::new(get_ty(&param.ty)?, false),
        );
    }

    compile_block(&func.body, &scope)?;

    todo!()
}

fn compile_block(blk: &ast::BlockStmt, scope: &Scope) -> Result<()> {
    Ok(())
}

fn compile_decl(decl: &ast::DeclStmt, scope: &mut Scope) -> Result<()> {
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

fn get_ty(ty: &ast::TyDef) -> Result<Ty> {
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
