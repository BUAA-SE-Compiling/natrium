// #[cfg(parse)]
pub mod io;

use crate::opcodes::Op;
use std::collections::HashMap;

/// S0 Assembly for use in R0VM
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct S0 {
    pub globals: Vec<GlobalValue>,
    pub functions: Vec<FnDef>,
}

/// Global variable or constant, described by bytes, addressed by ID
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GlobalValue {
    pub is_const: bool,
    pub bytes: Vec<u8>,
}

/// Function definition
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FnDef {
    pub name: u32,
    pub ret_slots: u32,
    pub param_slots: u32,
    pub loc_slots: u32,
    pub ins: Vec<Op>,
}
