use indexmap::IndexMap;
use smol_str::SmolStr;

use crate::{ty::Ty, util::MutWeak};

#[derive(Debug)]
pub struct Scope {
    pub parent: Option<MutWeak<Scope>>,
    pub vars: IndexMap<SmolStr, Symbol>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub ty: Ty,
}
