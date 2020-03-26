use crate::error::Error;
use crate::s0::S0;
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

    /// Instruction Pointer
    ip: u64,
    /// Base Pointer
    bp: u64,

    /// Standard Input Stream
    stdin: Box<dyn Read>,
    /// Standard Output Stream
    stdout: Box<dyn Write>,
}

impl<'src> R0Vm<'src> {
    pub fn new(src: &'src S0, stdin: Box<dyn Read>, stdout: Box<dyn Write>) -> R0Vm<'src> {
        // TODO: Move ip onto start of `_start` function
        R0Vm {
            src,
            heap: BTreeMap::new(),
            stack: Vec::new(),
            ip: 0,
            bp: 0,
            stdin,
            stdout,
        }
    }

    pub fn next_instruction(&mut self) {}
}
