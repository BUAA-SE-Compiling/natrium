pub mod err;
pub mod scope;
pub mod ty;

use err::{CompileError, CompileErrorKind};
use r0syntax::{ast, util::Mut, util::MutWeak};
use r0vm::s0;
use scope::{Scope, Symbol};
use ty::Ty;

type Result<T> = std::result::Result<T, CompileError>;

pub fn compile(tree: &ast::Program) -> Result<s0::S0> {
    let mut global_scope = Scope::new();
    for decl in &tree.decls {
        compile_decl(decl, &mut global_scope)?;
    }
    for func in &tree.funcs {
        compile_func(func, &global_scope)?;
    }
    todo!()
}

fn compile_func(func: &ast::FuncStmt, global_scope: &Scope) -> Result<s0::FnDef> {
    let mut func_scope = Scope::new_with_parent(global_scope);
    todo!()
}

fn compile_decl(decl: &ast::DeclStmt, scope: &mut Scope) -> Result<()> {
    let ty = get_ty(&decl.ty)?;
    let name = decl.name.val.clone();

    let symbol = Symbol {
        ty,
        is_const: decl.is_const,
    };

    if !scope.insert(name, symbol) {
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
