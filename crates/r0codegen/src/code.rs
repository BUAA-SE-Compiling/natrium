use r0vm::opcodes::Op;

pub enum Cond {
    Eq,
    Neq,
    Gt,
    Lt,
    Ge,
    Le,
}

pub enum JumpInst {
    Undefined,
    Unreachable,
    Return,
    Jump(usize),
    JumpIf(Cond, usize, usize),
}

pub struct BasicBlock {
    pub code: Vec<Op>,
    pub jump: JumpInst,
}

impl BasicBlock {
    pub fn new() -> BasicBlock {
        BasicBlock {
            code: vec![],
            jump: JumpInst::Undefined,
        }
    }
}
