pub mod ops;

use crate::error::*;
use crate::{opcodes::Op, s0::*};
use std::{collections::BTreeMap, io::Read, io::Write};

pub type Slot = u64;

/// An interpreter running S0 code.
pub struct R0Vm<'src> {
    /// Source file
    src: &'src S0,

    /// Memory heap
    heap: BTreeMap<u64, Box<[u8]>>,
    /// Memory stack
    stack: Vec<Slot>,

    /// Function Pointer
    fp: &'src Function,
    /// Instruction Pointer
    ip: usize,
    /// Base Pointer
    bp: usize,

    /// Standard Input Stream
    stdin: Box<dyn Read>,
    /// Standard Output Stream
    stdout: Box<dyn Write>,
}

impl<'src> R0Vm<'src> {
    pub fn new(src: &'src S0, stdin: Box<dyn Read>, stdout: Box<dyn Write>) -> R0Vm<'src> {
        // TODO: Move ip onto start of `_start` function
        let start = &src.start;
        R0Vm {
            src,
            heap: BTreeMap::new(),
            stack: Vec::new(),
            fp: start,
            ip: 0,
            bp: 0,
            stdin,
            stdout,
        }
    }

    #[inline]
    fn get_next_instruction(&mut self) -> Op {
        unimplemented!()
    }

    pub(crate) fn check_stack_overflow(&self, pushed: u64) -> Result<()> {
        if (self.stack.len() as u64) < (usize::max_value() as u64 - pushed) {
            Ok(())
        } else {
            Err(Error::StackOverflow)
        }
    }

    pub fn next_instruction(&mut self, op: Op) -> Result<()> {
        use Op::*;
        match op {
            Nop => Ok(()),
            Push(x) => self.push(x),
            Pop => self.pop().map(|_| ()),
            PopN(n) => self.pop_n(n),
            Dup => self.dup(),
            LocA(n) => self.loc_a(n),
            GlobA(n) => self.glob_a(n),
            Load8 => self.load8(),
            Load16 => self.load16(),
            Load32 => self.load32(),
            Load64 => self.load64(),
            Store8 => self.store8(),
            Store16 => self.store16(),
            Store32 => self.store32(),
            Store64 => self.store64(),
            Alloc => self.alloc(),
            Free => self.free(),
            StackAlloc(n) => self.stack_alloc(n),
            AddI => self.add_i(),
            SubI => self.sub_i(),
            MulI => self.mul_i(),
            DivI => self.div_i(),
            AddF => self.add_f(),
            SubF => self.sub_f(),
            MulF => self.mul_f(),
            DivF => self.div_f(),
            AdcI => unimplemented!("ADC is unstable"),
            Shl => self.shl(),
            Shr => self.shr(),
            And => self.and(),
            Or => self.or(),
            Xor => self.xor(),
            Not => self.not(),
            CmpI => self.cmp_i(),
            CmpF => self.cmp_f(),
            NegI => self.neg_i(),
            NegF => self.neg_f(),
            IToF => self.itof(),
            FToI => self.ftoi(),
            ShrL => self.shr_l(),
            BrA(addr) => self.br_a(addr),
            Br(off) => unimplemented!(),
            Bz(off) => unimplemented!(),
            Bnz(off) => unimplemented!(),
            Bl(off) => unimplemented!(),
            Bg(off) => unimplemented!(),
            Blz(off) => unimplemented!(),
            Bgz(off) => unimplemented!(),
            Call(id) => unimplemented!(),
            Ret => unimplemented!(),
            ScanI => unimplemented!(),
            ScanC => unimplemented!(),
            ScanF => unimplemented!(),
            PrintI => unimplemented!(),
            PrintC => unimplemented!(),
            PrintF => unimplemented!(),
            PrintS => unimplemented!(),
            PrintLn => unimplemented!(),
            Halt => unimplemented!(),
        }
    }
}
