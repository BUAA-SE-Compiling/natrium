use crate::opcodes::Op;
use std::collections::HashMap;

/// S0 Assembly for use in R0VM
pub struct S0 {
    pub globals: Vec<GlobalValue>,
    pub functions: Vec<FnDef>,
}

/// Global variable or constant, described by bytes, addressed by ID
pub struct GlobalValue {
    is_const: bool,
    bytes: Vec<u8>,
}

/// Function definition
pub struct FnDef {
    // pub name: u32,
    pub max_stack: u32,
    pub param_slots: u32,
    pub ret_slots: u32,
    pub ins: Vec<Op>,
}
