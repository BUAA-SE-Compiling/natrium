use r0vm::opcodes::Op;

#[derive(Debug, Copy, Clone)]
pub enum JumpInst {
    Undefined,
    Unreachable,
    Return,
    Jump(usize),
    JumpIf(usize, usize),
}

#[derive(Debug, Clone)]
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
