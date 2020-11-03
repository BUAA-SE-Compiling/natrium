use failure::Fail;
use std::fmt::Display;

use crate::s0::S0;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Invalid instruction: {}", _0)]
    InvalidInstruction(InvalidInstructionCtx),

    #[fail(display = "Stack overflow")]
    StackOverflow,

    #[fail(display = "Stack underflow")]
    StackUnderflow,

    #[fail(display = "Invalid local variable index {}", _0)]
    InvalidLocalIndex(u32),

    #[fail(display = "Invalid local variable index {}", _0)]
    InvalidArgIndex(u32),

    #[fail(display = "Invalid global variable index {}", _0)]
    InvalidGlobalIndex(u32),

    #[fail(display = "Invalid address 0x{:016x}", _0)]
    InvalidAddress(u64),

    #[fail(display = "Invalid stack offset {} (bp + {})", _0, _1)]
    InvalidStackOffset(u64, i64),

    #[fail(display = "Invalid function ID {}", _0)]
    InvalidFnId(u32),

    #[fail(display = "Unknown function (name index: {})", _0)]
    UnknownFunction(u32),

    #[fail(display = "Invalid instruction offset {}", _0)]
    InvalidInstructionOffset(usize),

    #[fail(display = "Dividing by zero")]
    DivZero,

    #[fail(display = "Arithmetic error")]
    ArithmeticErr,

    #[fail(display = "Allocated 0 size of memory")]
    AllocZero,

    #[fail(display = "Deallocating memory that is not allocated")]
    InvalidDeallocation,

    #[fail(display = "Out of memory")]
    OutOfMemory,

    #[fail(display = "Unaligned memory access of address 0x{:016x}", _0)]
    UnalignedAccess(u64),

    #[fail(display = "Control reaches end of function #{} without returning", _0)]
    ControlReachesEnd(usize),

    #[fail(display = "Unable to find entry point")]
    NoEntryPoint,

    #[fail(display = "Parse error")]
    ParseError,

    #[fail(display = "IO error: {}", _0)]
    IoError(std::io::Error),

    #[fail(display = "Allocation Layout error: {}", _0)]
    AllocLayoutError(std::alloc::LayoutErr),

    #[fail(display = "Halt")]
    Halt,
}

impl Error {
    pub fn format_with_ctx(&self, f: &mut std::fmt::Formatter, s0: &S0) -> std::fmt::Result {
        self.fmt(f)?;

        todo!()
    }
}

#[derive(Debug)]
pub struct InvalidInstructionCtx {
    /// Instruction opcode
    pub inst: u8,
    /// Function id
    pub fn_id: u32,
    /// Instruction offset
    pub inst_off: u64,
}

impl Display for InvalidInstructionCtx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "0x{:02x} at fn #{}:{}",
            self.inst, self.fn_id, self.inst_off
        )
    }
}

impl From<std::io::Error> for Error {
    fn from(x: std::io::Error) -> Self {
        Error::IoError(x)
    }
}

impl From<std::alloc::LayoutErr> for Error {
    fn from(x: std::alloc::LayoutErr) -> Self {
        Error::AllocLayoutError(x)
    }
}
