use std::{cell::RefCell, sync::atomic::AtomicU64};

use indexmap::IndexMap;
use smol_str::SmolStr;

use crate::ty::Ty;

#[derive(Debug)]
pub struct SymbolIdGenerator {
    next_id: u64,
}

impl SymbolIdGenerator {
    pub fn new() -> SymbolIdGenerator {
        SymbolIdGenerator { next_id: 0 }
    }

    pub fn next(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

#[derive(Debug)]
pub struct Scope<'p> {
    symbol_gen: &'p RefCell<SymbolIdGenerator>,
    pub parent: Option<&'p Scope<'p>>,
    pub vars: IndexMap<SmolStr, Symbol>,
}

#[allow(clippy::new_without_default)]
impl<'p> Scope<'p> {
    pub fn new_with_parent(parent: &'p Scope<'p>) -> Scope<'p> {
        Scope {
            symbol_gen: parent.symbol_gen,
            parent: Some(parent),
            vars: IndexMap::new(),
        }
    }

    pub fn new(symbol_gen: &'p RefCell<SymbolIdGenerator>) -> Scope<'p> {
        Scope {
            symbol_gen,
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

    pub fn is_root_scope(&self) -> bool {
        self.parent.is_none()
    }

    pub fn find_is_global<'s>(&'s self, ident: &str) -> Option<(&'s Symbol, bool)> {
        let self_res = self.find_in_self(ident);

        if self_res.is_none() {
            if let Some(p) = self.parent {
                return p.find_is_global(ident);
            }
        }

        self_res.map(|x| (x, self.is_root_scope()))
    }

    pub fn insert(&mut self, ident: SmolStr, mut symbol: Symbol) -> Option<u64> {
        let entry = self.vars.entry(ident);
        match entry {
            indexmap::map::Entry::Occupied(_) => None,
            indexmap::map::Entry::Vacant(v) => {
                let id = self.symbol_gen.borrow_mut().next();
                symbol.id = id;
                v.insert(symbol);
                Some(id)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub id: u64,
    pub ty: Ty,
    pub is_const: bool,
}

impl Symbol {
    pub fn new(ty: Ty, is_const: bool) -> Symbol {
        Symbol {
            ty,
            is_const,
            id: 0,
        }
    }
}
