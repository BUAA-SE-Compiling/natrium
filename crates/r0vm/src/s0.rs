use crate::opcodes::Op;

/// S0 Assembly for use in R0VM
pub struct S0 {
    pub start: Function,
}

pub struct Function {
    pub ins: Vec<Op>,
}
