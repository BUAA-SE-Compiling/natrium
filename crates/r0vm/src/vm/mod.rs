pub mod mem;
pub mod ops;

use crate::error::*;
use crate::{opcodes::Op, s0::*};
use mem::*;
use ops::*;
use std::{
    collections::{BTreeMap, HashMap},
    io::Write,
    io::{Bytes, Read},
};

pub type Slot = u64;
pub type Addr = u64;

/// An interpreter running S0 code.
pub struct R0Vm<'src> {
    /// Source file
    src: &'src S0,
    max_stack_size: usize,

    /// Global variable index
    global_idx: HashMap<u32, Addr>,

    /// Memory heap
    heap: BTreeMap<Addr, ManagedMemory>,
    /// Memory stack
    stack: Vec<Slot>,

    /// Function Pointer
    fn_info: &'src FnDef,
    /// Function ID
    fn_id: usize,
    /// Instruction Pointer
    ip: usize,
    /// Base Pointer
    bp: usize,

    /// Standard Input Stream
    stdin: Bytes<&'src mut dyn Read>,
    /// Standard Output Stream
    stdout: &'src mut dyn Write,
}

impl<'src> R0Vm<'src> {
    pub fn new(
        src: &'src S0,
        stdin: &'src mut dyn Read,
        stdout: &'src mut dyn Write,
    ) -> Result<R0Vm<'src>> {
        let start = src.functions.get(0).ok_or(Error::NoEntryPoint)?;
        let mut stack = vec![0; start.loc_slots as usize];
        {
            // push an invalid stack frame
            // TODO: Is there a more elegant way here?
            let usize_max = usize::max_value() as u64;
            stack.append(&mut vec![usize_max, usize_max, usize_max]);
        }
        let bp = start.loc_slots as usize;
        let (globals, global_idx) = Self::index_globals(&src.globals[..])?;
        Ok(R0Vm {
            src,
            max_stack_size: 131072,
            global_idx,
            heap: globals,
            stack,
            fn_info: start,
            fn_id: 0,
            ip: 0,
            bp,
            stdin: stdin.bytes(),
            stdout,
        })
    }

    fn index_globals(
        globals: &[GlobalValue],
    ) -> Result<(BTreeMap<Addr, ManagedMemory>, HashMap<u32, Addr>)> {
        let mut curr_max_addr = 0u64;

        let mut globals_map = BTreeMap::new();
        let mut idx = HashMap::new();

        for val in globals.into_iter().enumerate() {
            let (i, x) = val;
            let x: &GlobalValue = x;
            let len = x.bytes.len();
            let managed = ManagedMemory::from_slice(&x.bytes[..])?;

            let mem_addr = round_up_to_multiple(curr_max_addr + len as u64, 8);
            curr_max_addr = mem_addr;
            if mem_addr >= R0Vm::HEAP_START {
                return Err(Error::OutOfMemory);
            }

            globals_map.insert(mem_addr, managed);
            idx.insert(i as u32, mem_addr);
        }
        Ok((globals_map, idx))
    }

    pub fn step(&mut self) -> Result<()> {
        let op = self.get_next_instruction()?;
        self.exec_instruction(op)
    }

    /// Drive virtual machine to end, and abort when any error occurs.
    pub fn run_to_end(&mut self) -> Result<()> {
        loop {
            match self.step() {
                Ok(()) => (),
                Err(Error::ControlReachesEnd(0)) => break Ok(()),
                e @ _ => return e,
            }
        }
    }

    /// Drive virtual machine to end, and abort when any error occurs.
    pub fn run_to_end_inspect<F>(&mut self, mut inspect: F) -> Result<()>
    where
        F: FnMut(&Self) -> (),
    {
        loop {
            let res = self.step();
            inspect(self);
            match res {
                Ok(()) => (),
                Err(Error::ControlReachesEnd(0)) => break Ok(()),
                e @ _ => return e,
            }
        }
    }

    #[inline]
    fn get_next_instruction(&mut self) -> Result<Op> {
        let op = *self
            .fn_info
            .ins
            .get(self.ip)
            .ok_or(Error::ControlReachesEnd(self.fn_id))?;
        self.ip += 1;
        Ok(op)
    }

    pub(crate) fn check_stack_overflow(&self, push_cnt: usize) -> Result<()> {
        if self.stack.len() < (self.max_stack_size - push_cnt) {
            Ok(())
        } else {
            Err(Error::StackOverflow)
        }
    }

    fn get_fn_by_id(&self, id: u32) -> Result<&'src FnDef> {
        self.src
            .functions
            .get(id as usize)
            .ok_or(Error::InvalidFnId(id))
    }

    pub fn exec_instruction(&mut self, op: Op) -> Result<()> {
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
            DivU => self.div_u(),
            Shl => self.shl(),
            Shr => self.shr(),
            And => self.and(),
            Or => self.or(),
            Xor => self.xor(),
            Not => self.not(),
            CmpI => self.cmp_i(),
            CmpU => self.cmp_u(),
            CmpF => self.cmp_f(),
            NegI => self.neg_i(),
            NegF => self.neg_f(),
            IToF => self.itof(),
            FToI => self.ftoi(),
            ShrL => self.shr_l(),
            BrA(addr) => self.br_a(addr),
            Br(off) => self.br(off),
            Bz(off) => self.bz(off),
            Bnz(off) => self.bnz(off),
            Bl(off) => self.bl(off),
            Bg(off) => self.bg(off),
            Blz(off) => self.blz(off),
            Bgz(off) => self.bgz(off),
            Call(id) => self.call(id),
            Ret => self.ret(),
            ScanI => self.scan_i(),
            ScanC => self.scan_c(),
            ScanF => self.scan_f(),
            PrintI => self.print_i(),
            PrintC => self.print_c(),
            PrintF => self.print_f(),
            PrintS => self.print_s(),
            PrintLn => self.print_ln(),
            Panic => self.halt(),
        }
    }

    /// All information from current runtime stack. Usually being called
    /// during panic, halt, stack overflow or debug.
    pub fn stack_trace(&self) -> Result<Vec<StackInfo>> {
        let mut infos = Vec::new();

        infos.push(self.cur_stack_info()?);

        let mut bp = self.bp;
        while bp != usize::max_value() {
            let (info, bp_) = self.stack_info(bp)?;
            if info.fn_id == usize::max_value() as u64 {
                // Stack bottom sentinel item
                break;
            }
            bp = bp_;
            infos.push(info);
        }
        Ok(infos)
    }

    /// Return the information of current running function
    pub fn cur_stack_info(&self) -> Result<StackInfo> {
        Ok(StackInfo {
            fn_id: self.fn_id as u64,
            inst: self.ip as u64,
            fn_name: self
                .src
                .globals
                .get(self.fn_info.name as usize)
                .map(|val| String::from_utf8_lossy(&val.bytes[..]).into()),
        })
    }

    pub fn stack_info(&self, bp: usize) -> Result<(StackInfo, usize)> {
        let prev_bp = *self
            .stack
            .get(bp)
            .ok_or_else(|| Error::InvalidAddress(stack_idx_to_vm_addr(bp)))?;
        let ip = *self
            .stack
            .get(bp + 1)
            .ok_or_else(|| Error::InvalidAddress(stack_idx_to_vm_addr(bp + 1)))?;
        let fn_id = *self
            .stack
            .get(bp + 2)
            .ok_or_else(|| Error::InvalidAddress(stack_idx_to_vm_addr(bp + 2)))?;
        let fn_name = self.src.functions.get(fn_id as usize).and_then(|f| {
            self.src
                .globals
                .get(f.name as usize)
                .map(|val| String::from_utf8_lossy(&val.bytes[..]).into())
        });
        dbg!(prev_bp);
        Ok((
            StackInfo {
                fn_name,
                fn_id,
                inst: ip,
            },
            prev_bp as usize,
        ))
    }

    pub fn stack(&self) -> &Vec<Slot> {
        &self.stack
    }

    #[inline]
    fn total_loc(&self) -> usize {
        let total_loc = self.fn_info.loc_slots + self.fn_info.param_slots + self.fn_info.ret_slots;
        total_loc as usize
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StackInfo {
    pub fn_name: Option<String>,
    pub fn_id: u64,
    pub inst: u64,
}
