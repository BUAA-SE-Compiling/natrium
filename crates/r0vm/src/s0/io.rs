//! Module for reading and writing s0 values

use super::*;
// use nom::*;
use std::io::{Read, Write};

/// Read and write from binary source
pub trait WriteBinary: Sized {
    fn read_binary(r: &mut dyn Read) -> std::io::Result<Option<Self>>;
    fn write_binary(&self, w: &mut dyn Write) -> std::io::Result<()>;
}

impl S0 {
    pub const MAGIC_NUMBER: u32 = 0x72303b3e;
    pub const VERSION: u32 = 1;
}

macro_rules! unwrap {
    ($e:expr) => {
        match $e {
            Some(x) => x,
            None => return Ok(None),
        }
    };
}

macro_rules! read {
    ($ty:ty,$read:expr) => {
        match <$ty>::read_binary($read)? {
            Some(x) => x,
            None => return Ok(None),
        }
    };
}

impl<T> WriteBinary for Vec<T>
where
    T: WriteBinary,
{
    fn read_binary(r: &mut dyn Read) -> std::io::Result<Option<Self>> {
        let size = read!(u32, r) as usize;
        let mut vec = Vec::with_capacity(size);
        for _ in 0..size {
            vec.push(read!(T, r));
        }
        Ok(Some(vec))
    }

    fn write_binary(&self, w: &mut dyn Write) -> std::io::Result<()> {
        (self.len() as u32).write_binary(w)?;
        for item in self {
            item.write_binary(w)?;
        }
        Ok(())
    }
}

impl WriteBinary for u8 {
    #[inline]
    fn read_binary(r: &mut dyn Read) -> std::io::Result<Option<Self>> {
        let mut buf = [0u8; 1];
        r.read_exact(&mut buf)?;
        Ok(Some(buf[0]))
    }

    #[inline]
    fn write_binary(&self, w: &mut dyn Write) -> std::io::Result<()> {
        w.write_all(&[*self])
    }
}

impl WriteBinary for u32 {
    #[inline]
    fn read_binary(r: &mut dyn Read) -> std::io::Result<Option<Self>> {
        let mut buf = [0u8; 4];
        r.read_exact(&mut buf)?;
        Ok(Some(u32::from_be_bytes(buf)))
    }

    #[inline]
    fn write_binary(&self, w: &mut dyn Write) -> std::io::Result<()> {
        w.write_all(&self.to_be_bytes())
    }
}

impl WriteBinary for u64 {
    #[inline]
    fn read_binary(r: &mut dyn Read) -> std::io::Result<Option<Self>> {
        let mut buf = [0u8; 8];
        r.read_exact(&mut buf)?;
        Ok(Some(u64::from_be_bytes(buf)))
    }

    #[inline]
    fn write_binary(&self, w: &mut dyn Write) -> std::io::Result<()> {
        w.write_all(&self.to_be_bytes())
    }
}

impl WriteBinary for Op {
    fn read_binary(r: &mut dyn Read) -> std::io::Result<Option<Self>> {
        let opcode = unwrap!(u8::read_binary(r)?);
        let param_length = Op::param_size(opcode);
        let op = match param_length {
            0 => Op::from_code(opcode, 0),
            4 => Op::from_code(opcode, read!(u32, r) as u64),
            8 => Op::from_code(opcode, read!(u64, r)),
            _ => unreachable!(),
        };
        Ok(op)
    }

    fn write_binary(&self, w: &mut dyn Write) -> std::io::Result<()> {
        let opcode = self.code();
        let param = self.code_param();
        let param_len = Op::param_size(opcode);

        w.write_all(&[opcode])?;
        match param_len {
            0 => (),
            4 => {
                let x = param as u32;
                x.write_binary(w)?
            }
            8 => param.write_binary(w)?,
            _ => unreachable!(),
        }
        Ok(())
    }
}

impl WriteBinary for FnDef {
    fn read_binary(r: &mut dyn Read) -> std::io::Result<Option<Self>> {
        let name = read!(u32, r);
        let ret_slots = read!(u32, r);
        let param_slots = read!(u32, r);
        let loc_slots = read!(u32, r);
        let ins = read!(Vec<Op>, r);
        Ok(Some(FnDef {
            name,
            ret_slots,
            param_slots,
            loc_slots,
            ins,
        }))
    }

    fn write_binary(&self, w: &mut dyn Write) -> std::io::Result<()> {
        self.name.write_binary(w)?;
        self.ret_slots.write_binary(w)?;
        self.param_slots.write_binary(w)?;
        self.loc_slots.write_binary(w)?;
        self.ins.write_binary(w)
    }
}

impl WriteBinary for GlobalValue {
    fn read_binary(r: &mut dyn Read) -> std::io::Result<Option<Self>> {
        let is_const = read!(u8, r);
        let payload = read!(Vec<u8>, r);
        Ok(Some(GlobalValue {
            is_const: is_const != 0,
            bytes: payload,
        }))
    }
    fn write_binary(&self, w: &mut dyn Write) -> std::io::Result<()> {
        (self.is_const as u8).write_binary(w)?;
        self.bytes.write_binary(w)
    }
}

impl WriteBinary for S0 {
    fn read_binary(r: &mut dyn Read) -> std::io::Result<Option<Self>> {
        let magic_number = read!(u32, r);
        let version = read!(u32, r);
        if magic_number != S0::MAGIC_NUMBER || version != S0::VERSION {
            return Ok(None);
        }
        let global_values = read!(Vec<GlobalValue>, r);
        let fn_defs = read!(Vec<FnDef>, r);
        Ok(Some(S0 {
            globals: global_values,
            functions: fn_defs,
        }))
    }

    fn write_binary(&self, w: &mut dyn Write) -> std::io::Result<()> {
        S0::MAGIC_NUMBER.write_binary(w)?;
        S0::VERSION.write_binary(w)?;
        self.globals.write_binary(w)?;
        self.functions.write_binary(w)
    }
}
