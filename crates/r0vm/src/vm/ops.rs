//! Implementation for all operation codes for R0VM

use super::Slot;
use crate::{error::*, s0::FnDef};

/// Reinterpret x as T
#[inline]
fn reinterpret_u<T>(x: u64) -> T
where
    T: U64Transmutable,
{
    unsafe { std::mem::transmute_copy::<u64, T>(&x) }
}

/// Reinterpret T as x
#[inline]
fn reinterpret_t<T>(x: T) -> u64
where
    T: U64Transmutable,
{
    unsafe { std::mem::transmute_copy::<_, u64>(&x) }
}

/// A value type that is the same size as u64
trait U64Transmutable {}
impl U64Transmutable for i64 {}
impl U64Transmutable for f64 {}
impl U64Transmutable for u64 {}

impl<'src> super::R0Vm<'src> {
    #[inline]
    fn pop2(&mut self) -> Result<(u64, u64)> {
        let lhs = self.pop()?;
        let rhs = self.pop()?;
        Ok((lhs, rhs))
    }

    #[inline]
    fn pop2f(&mut self) -> Result<(f64, f64)> {
        let lhs = reinterpret_u(self.pop()?);
        let rhs = reinterpret_u(self.pop()?);
        Ok((lhs, rhs))
    }

    #[inline]
    fn pop2i(&mut self) -> Result<(i64, i64)> {
        let lhs = reinterpret_u(self.pop()?);
        let rhs = reinterpret_u(self.pop()?);
        Ok((lhs, rhs))
    }

    // ====

    pub(crate) fn push(&mut self, x: u64) -> Result<()> {
        self.check_stack_overflow(1)?;
        self.stack.push(x);
        Ok(())
    }

    pub(crate) fn pop(&mut self) -> Result<u64> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    pub(crate) fn pop_n(&mut self, n: u64) -> Result<()> {
        let rem = (self.stack.len() as u64)
            .checked_sub(n)
            .ok_or(Error::StackUnderflow)?;
        self.stack.truncate(rem as usize);
        Ok(())
    }

    pub(crate) fn dup(&mut self) -> Result<()> {
        let top = *self.stack.last().ok_or(Error::StackUnderflow)?;
        self.push(top)
    }

    pub(crate) fn loc_a(&mut self, a: u64) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn glob_a(&mut self, a: u64) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn load8(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn load16(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn load32(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn load64(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn store8(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn store16(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn store32(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn store64(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn alloc(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn free(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn stack_alloc(&mut self, count: u64) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn add_i(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2()?;
        self.push(lhs.wrapping_add(rhs))?;
        Ok(())
    }

    pub(crate) fn sub_i(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2()?;
        self.push(lhs.wrapping_sub(rhs))?;
        Ok(())
    }

    pub(crate) fn mul_i(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2()?;
        self.push(lhs.wrapping_mul(rhs))?;
        Ok(())
    }

    pub(crate) fn div_i(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2i()?;
        let res = lhs.checked_div(rhs).ok_or_else(|| {
            if rhs == -1 {
                Error::ArithmeticErr
            } else {
                Error::DivZero
            }
        })?;
        self.push(reinterpret_t(res))?;
        Ok(())
    }

    pub(crate) fn div_u(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2()?;
        let res = lhs.checked_div(rhs).ok_or(Error::DivZero)?;
        self.push(res)?;
        Ok(())
    }

    pub(crate) fn add_f(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2f()?;
        let res = reinterpret_t(lhs + rhs);
        self.push(res)?;
        Ok(())
    }

    pub(crate) fn sub_f(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2f()?;
        let res = reinterpret_t(lhs - rhs);
        self.push(res)?;
        Ok(())
    }

    pub(crate) fn mul_f(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2f()?;
        let res = reinterpret_t(lhs * rhs);
        self.push(res)?;
        Ok(())
    }

    pub(crate) fn div_f(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2f()?;
        let res = reinterpret_t(lhs / rhs);
        self.push(res)?;
        Ok(())
    }

    pub(crate) fn _adc_i(&mut self) -> Result<()> {
        unimplemented!("adc is unstable")
    }

    pub(crate) fn shl(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2()?;
        let rhs = (rhs & u32::max_value() as u64) as u32;
        self.push(lhs.wrapping_shl(rhs))?;
        Ok(())
    }

    pub(crate) fn shr(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2()?;
        let rhs = (rhs & u32::max_value() as u64) as u32;
        // arithmetic shift
        let lhs = lhs as i64;
        self.push(lhs.wrapping_shr(rhs) as u64)?;
        Ok(())
    }

    pub(crate) fn and(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2()?;
        self.push(lhs & rhs)?;
        Ok(())
    }

    pub(crate) fn or(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2()?;
        self.push(lhs | rhs)?;
        Ok(())
    }

    pub(crate) fn xor(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2()?;
        self.push(lhs ^ rhs)?;
        Ok(())
    }

    pub(crate) fn not(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2()?;
        self.push(lhs ^ rhs)?;
        Ok(())
    }

    fn cmp_t<T>(&mut self) -> Result<()>
    where
        T: PartialOrd + U64Transmutable,
    {
        let (lhs, rhs) = self.pop2()?;
        let lhs = reinterpret_u::<T>(lhs);
        let rhs = reinterpret_u::<T>(rhs);
        if lhs < rhs {
            self.push(reinterpret_t(-1i64))
        } else if lhs > rhs {
            self.push(1)
        } else {
            self.push(0)
        }
    }

    pub(crate) fn cmp_i(&mut self) -> Result<()> {
        self.cmp_t::<i64>()
    }

    pub(crate) fn cmp_u(&mut self) -> Result<()> {
        self.cmp_t::<u64>()
    }

    pub(crate) fn cmp_f(&mut self) -> Result<()> {
        self.cmp_t::<f64>()
    }

    pub(crate) fn neg_i(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn neg_f(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn itof(&mut self) -> Result<()> {
        let val = reinterpret_u::<i64>(self.pop()?);
        self.push(reinterpret_t(val as f64))
    }

    pub(crate) fn ftoi(&mut self) -> Result<()> {
        let val = reinterpret_u::<f64>(self.pop()?);
        // UB: converting f64 that are larger than i64::max_value() is undefined
        self.push(reinterpret_t(val as i64))
    }

    pub(crate) fn shr_l(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2()?;
        let rhs = (rhs & u32::max_value() as u64) as u32;
        // logical shift
        self.push(lhs.wrapping_shr(rhs))?;
        Ok(())
    }

    pub(crate) fn br_a(&mut self, addr: u64) -> Result<()> {
        unimplemented!("branch to specific address is unstable")
    }

    pub(crate) fn br(&mut self, off: i32) -> Result<()> {
        self.ip = if off > 0 {
            self.ip.checked_add(off as usize)
        } else {
            let off = (-off) as usize;
            self.ip.checked_sub(off as usize)
        }
        .and_then(|off| {
            if off > self.fp.ins.len() {
                None
            } else {
                Some(off)
            }
        })
        .ok_or(Error::InvalidInstructionOffset)?;

        Ok(())
    }

    pub(crate) fn bz(&mut self, off: i32) -> Result<()> {
        let x = self.pop()?;
        if x == 0 {
            self.br(off)
        } else {
            Ok(())
        }
    }

    pub(crate) fn bnz(&mut self, off: i32) -> Result<()> {
        let x = self.pop()?;
        if x != 0 {
            self.br(off)
        } else {
            Ok(())
        }
    }

    pub(crate) fn bl(&mut self, off: i32) -> Result<()> {
        let x = self.pop()?;
        if x & (1 << 63) != 0 {
            self.br(off)
        } else {
            Ok(())
        }
    }

    pub(crate) fn bg(&mut self, off: i32) -> Result<()> {
        let x = self.pop()?;
        if x != 0 && x & (1 << 63) == 0 {
            self.br(off)
        } else {
            Ok(())
        }
    }

    pub(crate) fn blz(&mut self, off: i32) -> Result<()> {
        let x = self.pop()?;
        if x & (1 << 63) != 0 || x == 0 {
            self.br(off)
        } else {
            Ok(())
        }
    }

    pub(crate) fn bgz(&mut self, off: i32) -> Result<()> {
        let x = self.pop()?;
        if x & (1 << 63) == 0 {
            self.br(off)
        } else {
            Ok(())
        }
    }

    pub(crate) fn call(&mut self, id: u32) -> Result<()> {
        let sp = self.stack.len();

        let fp = self.get_fn_by_id(id)?;
        self.stack_alloc(fp.max_stack as u64)?;

        let bp = self.stack.len();

        self.push(sp as u64)?;
        self.push(self.ip as u64)?;
        self.push(self.fn_id as u64)?;

        self.fn_id = id as usize;
        self.fp = fp;
        self.ip = 0;
        self.bp = bp;

        Ok(())
    }

    pub(crate) fn ret(&mut self) -> Result<()> {
        let old_sp = *self.stack.get(self.bp).ok_or(Error::StackUnderflow)?;
        let old_ip = *self.stack.get(self.bp + 1).ok_or(Error::StackUnderflow)?;
        let old_fn = *self.stack.get(self.bp + 2).ok_or(Error::StackUnderflow)?;

        let fp = self.get_fn_by_id(old_fn as u32)?;
        // %sp = %bp
        self.stack.truncate(old_sp as usize);
        self.pop_n(self.fp.param_slots as u64)?;

        self.fp = fp;
        self.ip = old_ip as usize;

        Ok(())
    }

    pub(crate) fn scan_i(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn scan_c(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn scan_f(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn print_i(&mut self) -> Result<()> {
        let i = self.pop()?;
        self.stdout
            .write_fmt(format_args!("{}", i))
            .map_err(|err| err.into())
    }

    pub(crate) fn print_c(&mut self) -> Result<()> {
        let i = self.pop()?;
        let c = (i & 0xff) as u8 as char;
        self.stdout
            .write_fmt(format_args!("{}", c))
            .map_err(|err| err.into())
    }

    pub(crate) fn print_f(&mut self) -> Result<()> {
        let i = self.pop()?;
        let f = reinterpret_u::<f64>(i);
        self.stdout
            .write_fmt(format_args!("{}", f))
            .map_err(|err| err.into())
    }

    pub(crate) fn print_s(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn print_ln(&mut self) -> Result<()> {
        self.stdout
            .write_fmt(format_args!("\r\n"))
            .map_err(|err| err.into())
    }

    pub(crate) fn halt(&mut self) -> Result<()> {
        Err(Error::Halt)
    }
}
