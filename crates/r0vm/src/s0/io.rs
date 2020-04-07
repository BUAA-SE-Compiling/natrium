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

impl<T> WriteBinary for Vec<T>
where
    T: WriteBinary,
{
    fn read_binary(r: &mut dyn Read) -> std::io::Result<Option<Self>> {
        let mut size_buf = [0u8; 4];
        r.read_exact(&mut size_buf)?;
        let size = u32::from_be_bytes(size_buf);
        let mut vec = vec![];
        for _ in 0..size {
            let t = T::read_binary(r)?;
            let t = unwrap!(t);
            vec.push(t);
        }
        Ok(Some(vec))
    }

    fn write_binary(&self, w: &mut dyn Write) -> std::io::Result<()> {
        w.write_all(&self.len().to_be_bytes())?;
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
        w.write_all(&self.to_be_bytes())
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
        let mut buf = [0u8; 1];
        // read opcode
        r.read_exact(&mut buf)?;
        let opcode = buf[0];
        let param_length = Op::param_size(opcode);
        let op = match param_length {
            0 => Op::from_code(opcode, 0),
            4 => {
                let mut buf = [0u8; 4];
                r.read_exact(&mut buf)?;
                Op::from_code(opcode, u32::from_be_bytes(buf) as u64)
            }
            8 => {
                let mut buf = [0u8; 8];
                r.read_exact(&mut buf)?;
                Op::from_code(opcode, u64::from_be_bytes(buf))
            }
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
                w.write_all(&x.to_be_bytes())?;
            }
            8 => w.write_all(&param.to_be_bytes())?,
            _ => unreachable!(),
        }
        Ok(())
    }
}

impl WriteBinary for FnDef {
    fn read_binary(r: &mut dyn Read) -> std::io::Result<Option<Self>> {
        let name = u32::read_binary(r)?.unwrap();
        let ret_slots = u32::read_binary(r)?.unwrap();
        let param_slots = u32::read_binary(r)?.unwrap();
        let loc_slots = u32::read_binary(r)?.unwrap();
        let ins = unwrap!(Vec::<Op>::read_binary(r)?);
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
        let is_const = u8::read_binary(r)?.unwrap();
        let payload = unwrap!(Vec::<u8>::read_binary(r)?);
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
        let magic_number = u32::read_binary(r)?.unwrap();
        let version = u32::read_binary(r)?.unwrap();
        if magic_number != S0::MAGIC_NUMBER || version != S0::VERSION {
            return Ok(None);
        }
        let global_values = unwrap!(Vec::<GlobalValue>::read_binary(r)?);
        let fn_defs = unwrap!(Vec::<FnDef>::read_binary(r)?);
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
