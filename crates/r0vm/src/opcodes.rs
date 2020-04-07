#[derive(Debug, Copy, Clone)]
#[repr(u8, C)]
pub enum Op {
    Nop,
    Push(u64),
    Pop,
    PopN(u32),
    Dup,
    LocA(u32),
    GlobA(u32),
    Load8,
    Load16,
    Load32,
    Load64,
    Store8,
    Store16,
    Store32,
    Store64,
    Alloc,
    Free,
    StackAlloc(u32),
    AddI,
    SubI,
    MulI,
    DivI,
    AddF,
    SubF,
    MulF,
    DivF,
    DivU,
    Shl,
    Shr,
    And,
    Or,
    Xor,
    Not,
    CmpI,
    CmpU,
    CmpF,
    NegI,
    NegF,
    IToF,
    FToI,
    ShrL,
    BrA(u64),
    Br(i32),
    Bz(i32),
    Bnz(i32),
    Bl(i32),
    Bg(i32),
    Blz(i32),
    Bgz(i32),
    Call(u32),
    Ret,
    ScanI,
    ScanC,
    ScanF,
    PrintI,
    PrintC,
    PrintF,
    PrintS,
    PrintLn,
    Panic,
}

impl Op {
    pub fn code(&self) -> u8 {
        use Op::*;
        match self {
            Nop => 0x00,
            Push(..) => 0x01,
            Pop => 0x02,
            PopN(..) => 0x03,
            Dup => 0x04,
            LocA(..) => 0x08,
            GlobA(..) => 0x09,
            Load8 => 0x10,
            Load16 => 0x11,
            Load32 => 0x12,
            Load64 => 0x13,
            Store8 => 0x14,
            Store16 => 0x15,
            Store32 => 0x16,
            Store64 => 0x17,
            Alloc => 0x18,
            Free => 0x19,
            StackAlloc(..) => 0x1a,
            AddI => 0x20,
            SubI => 0x21,
            MulI => 0x22,
            DivI => 0x23,
            AddF => 0x24,
            SubF => 0x25,
            MulF => 0x26,
            DivF => 0x27,
            DivU => 0x28,
            Shl => 0x29,
            Shr => 0x2a,
            And => 0x2b,
            Or => 0x2c,
            Xor => 0x2d,
            Not => 0x2e,
            CmpI => 0x30,
            CmpU => 0x31,
            CmpF => 0x32,
            NegI => 0x34,
            NegF => 0x35,
            IToF => 0x36,
            FToI => 0x37,
            ShrL => 0x38,
            BrA(..) => 0x40,
            Br(..) => 0x41,
            Bz(..) => 0x42,
            Bnz(..) => 0x43,
            Bl(..) => 0x44,
            Bg(..) => 0x45,
            Blz(..) => 0x46,
            Bgz(..) => 0x47,
            Call(..) => 0x48,
            Ret => 0x49,
            ScanI => 0x50,
            ScanC => 0x51,
            ScanF => 0x52,
            PrintI => 0x54,
            PrintC => 0x55,
            PrintF => 0x56,
            PrintS => 0x57,
            PrintLn => 0x58,
            Panic => 0xfe,
        }
    }

    pub fn param_size(code: u8) -> usize {
        match code {
            0x01 | 0x40 => 8,
            0x03 | 0x08 | 0x09 | 0x1a | 0x41..=0x49 => 4,
            _ => 0,
        }
    }

    pub fn from_code(code: u8, param: u64) -> Option<Op> {
        use Op::*;
        match code {
            0x00 => Nop.into(),
            0x01 => Push(param).into(),
            0x02 => Pop.into(),
            0x03 => PopN(param as u32).into(),
            0x04 => Dup.into(),
            0x08 => LocA(param as u32).into(),
            0x09 => GlobA(param as u32).into(),
            0x10 => Load8.into(),
            0x11 => Load16.into(),
            0x12 => Load32.into(),
            0x13 => Load64.into(),
            0x14 => Store8.into(),
            0x15 => Store16.into(),
            0x16 => Store32.into(),
            0x17 => Store64.into(),
            0x18 => Alloc.into(),
            0x19 => Free.into(),
            0x1a => StackAlloc(param as u32).into(),
            0x20 => AddI.into(),
            0x21 => SubI.into(),
            0x22 => MulI.into(),
            0x23 => DivI.into(),
            0x24 => AddF.into(),
            0x25 => SubF.into(),
            0x26 => MulF.into(),
            0x27 => DivF.into(),
            0x28 => DivU.into(),
            0x29 => Shl.into(),
            0x2a => Shr.into(),
            0x2b => And.into(),
            0x2c => Or.into(),
            0x2d => Xor.into(),
            0x2e => Not.into(),
            0x30 => CmpI.into(),
            0x31 => CmpU.into(),
            0x32 => CmpF.into(),
            0x34 => NegI.into(),
            0x35 => NegF.into(),
            0x36 => IToF.into(),
            0x37 => FToI.into(),
            0x38 => ShrL.into(),
            0x40 => BrA(param as u64).into(),
            0x41 => Br(param as i64 as i32).into(),
            0x42 => Bz(param as i64 as i32).into(),
            0x43 => Bnz(param as i64 as i32).into(),
            0x44 => Bl(param as i64 as i32).into(),
            0x45 => Bg(param as i64 as i32).into(),
            0x46 => Blz(param as i64 as i32).into(),
            0x47 => Bgz(param as i64 as i32).into(),
            0x48 => Call(param as u32).into(),
            0x49 => Ret.into(),
            0x50 => ScanI.into(),
            0x51 => ScanC.into(),
            0x52 => ScanF.into(),
            0x54 => PrintI.into(),
            0x55 => PrintC.into(),
            0x56 => PrintF.into(),
            0x57 => PrintS.into(),
            0x58 => PrintLn.into(),
            0xfe => Panic.into(),
            _ => None,
        }
    }

    pub fn code_param(&self) -> u64 {
        use Op::*;
        match *self {
            Push(x) => x,
            PopN(x) => x as u64,
            LocA(x) => x as u64,
            GlobA(x) => x as u64,
            StackAlloc(x) => x as u64,
            BrA(x) => x,
            Br(x) => x as u32 as u64,
            Bz(x) => x as u32 as u64,
            Bnz(x) => x as u32 as u64,
            Bl(x) => x as u32 as u64,
            Bg(x) => x as u32 as u64,
            Blz(x) => x as u32 as u64,
            Bgz(x) => x as u32 as u64,
            Call(x) => x as u32 as u64,
            _ => 0u64,
        }
    }
}
