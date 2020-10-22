use indexmap::IndexMap;
use smol_str::SmolStr;

use crate::{
    span::Span,
    ty::TyKind,
    util::MutWeak,
    util::{Mut, P},
};

#[derive(Debug)]
pub struct Scope {
    pub parent: Option<MutWeak<Scope>>,
    pub vars: IndexMap<SmolStr, Symbol>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub ty: TyKind,
}
