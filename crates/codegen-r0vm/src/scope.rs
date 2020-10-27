use indexmap::IndexMap;
use smol_str::SmolStr;

use crate::ty::Ty;

#[derive(Debug)]
pub struct Scope<'p> {
    pub parent: Option<&'p Scope<'p>>,
    pub vars: IndexMap<SmolStr, Symbol>,
}

#[allow(clippy::new_without_default)]
impl<'p> Scope<'p> {
    pub fn new_with_parent(parent: &'p Scope<'p>) -> Scope<'p> {
        Scope {
            parent: Some(parent),
            vars: IndexMap::new(),
        }
    }

    pub fn new() -> Scope<'p> {
        Scope {
            parent: None,
            vars: IndexMap::new(),
        }
    }

    pub fn find_in_self(&self, ident: &str) -> Option<&Symbol> {
        self.vars.get(ident)
    }

    pub fn find<'s>(&'s self, ident: &str) -> Option<&'s Symbol> {
        let self_res = self.find_in_self(ident);

        if self_res.is_none() {
            if let Some(p) = self.parent {
                return p.find(ident);
            }
        }
        self_res
    }

    pub fn insert(&mut self, ident: SmolStr, symbol: Symbol) -> bool {
        let entry = self.vars.entry(ident);
        match entry {
            indexmap::map::Entry::Occupied(_) => false,
            indexmap::map::Entry::Vacant(v) => {
                v.insert(symbol);
                true
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub ty: Ty,
    pub is_const: bool,
}
