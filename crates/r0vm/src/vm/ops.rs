//! Implementation for all instruction operations for R0VM

use super::{R0Vm, Slot};
use crate::{error::*, s0::FnDef};
use text_io::try_read;

/// Reinterpret x as T
#[inline]
pub(crate) fn reinterpret_u<T>(x: u64) -> T
where
    T: U64Transmutable,
{
    unsafe { std::mem::transmute_copy::<u64, T>(&x) }
}

/// Reinterpret T as x
#[inline]
pub(crate) fn reinterpret_t<T>(x: T) -> u64
where
    T: U64Transmutable,
{
    unsafe { std::mem::transmute_copy::<_, u64>(&x) }
}

/// A value type that is the same size as u64
pub(crate) trait U64Transmutable {}
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
        self.stack_push(x)?;
        Ok(())
    }

    #[inline]
    pub(crate) fn pop(&mut self) -> Result<u64> {
        self.stack_pop().map_err(|_| Error::StackUnderflow)
    }

    pub(crate) fn pop_n(&mut self, n: u32) -> Result<()> {
        self.stack_truncate_by(n as usize)?;
        Ok(())
    }

    pub(crate) fn dup(&mut self) -> Result<()> {
        let top = self.stack_top()?;
        self.push(top)
    }

    pub(crate) fn loc_a(&mut self, a: u32) -> Result<()> {
        let total_loc = self.fn_info.loc_slots;

        // check local index
        if a > total_loc {
            return Err(Error::InvalidLocalIndex(a));
        }

        let bp = self.bp;
        let addr = R0Vm::STACK_START
            .wrapping_add((bp) as u64 * 8)
            .wrapping_add(3 * 8)
            .wrapping_add(a as u64 * 8);
        self.push(addr)
    }

    pub(crate) fn arg_a(&mut self, a: u32) -> Result<()> {
        let total_arg = self.fn_info.ret_slots + self.fn_info.param_slots;

        // check local index
        if a > total_arg {
            return Err(Error::InvalidArgIndex(a));
        }

        let bp = self.bp;
        let addr = R0Vm::STACK_START
            .wrapping_add((bp) as u64 * 8)
            .wrapping_add(a as u64 * 8)
            .wrapping_sub(total_arg as u64 * 8);

        self.push(addr)
    }

    pub(crate) fn glob_a(&mut self, a: u32) -> Result<()> {
        let addr = *self
            .global_idx
            .get(&a)
            .ok_or(Error::InvalidGlobalIndex(a))?;
        self.push(addr)
    }

    #[inline]
    fn load_t<T>(&mut self) -> Result<()>
    where
        T: Into<u64> + Copy,
    {
        assert!(std::mem::size_of::<T>() <= std::mem::size_of::<u64>());
        let addr = self.pop()?;
        let res = self.access_mem_get::<T>(addr)?;
        let res = res.into();
        self.push(res)
    }

    pub(crate) fn load8(&mut self) -> Result<()> {
        self.load_t::<u8>()
    }

    pub(crate) fn load16(&mut self) -> Result<()> {
        self.load_t::<u16>()
    }

    pub(crate) fn load32(&mut self) -> Result<()> {
        self.load_t::<u32>()
    }

    pub(crate) fn load64(&mut self) -> Result<()> {
        self.load_t::<u64>()
    }

    #[inline]
    fn store_t<T, F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(u64) -> T,
        T: Copy + Into<u64>,
    {
        assert!(std::mem::size_of::<T>() <= std::mem::size_of::<u64>());
        let t = self.pop()?;
        let t = f(t);
        let addr = self.pop()?;

        self.access_mem_set(addr, t)
    }

    pub(crate) fn store8(&mut self) -> Result<()> {
        self.store_t(|x| (x & 0xff) as u8)
    }

    pub(crate) fn store16(&mut self) -> Result<()> {
        self.store_t(|x| (x & 0xffff) as u16)
    }

    pub(crate) fn store32(&mut self) -> Result<()> {
        self.store_t(|x| (x & 0xffffffff) as u32)
    }

    pub(crate) fn store64(&mut self) -> Result<()> {
        self.store_t(|x| x)
    }

    pub(crate) fn alloc(&mut self) -> Result<()> {
        let len = self.pop()? as usize;
        let addr = self.alloc_heap(len, 64)?;
        self.push(addr)
    }

    pub(crate) fn free(&mut self) -> Result<()> {
        let addr = self.pop()?;
        self.free_heap(addr)
    }

    pub(crate) fn stack_alloc(&mut self, count: u32) -> Result<()> {
        self.sp += count as usize;
        Ok(())
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
        let res = lhs
            .checked_div(rhs)
            // there's only 2 ways this could raise None:
            // - lhs == i64::min_value && rhs == -1  => Ok(i64::min_value)
            // - rhs == 0   => Err(Error::DivZero)
            .or_else(|| if rhs == -1 { Some(rhs) } else { None })
            .ok_or(Error::DivZero)?;
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
        let lhs = self.pop()?;
        if lhs == 0 {
            self.push(1)?;
        } else {
            self.push(0)?;
        }
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
        let x = self.pop()?;
        let res = x.wrapping_neg();
        self.push(res)
    }

    pub(crate) fn neg_f(&mut self) -> Result<()> {
        let x = self.pop()?;
        let f = reinterpret_u::<f64>(x);
        let res = -f;
        self.push(reinterpret_t(res))
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

    pub(crate) fn set_lt(&mut self) -> Result<()> {
        let val = self.pop()?;
        if (val as i64) < 0 {
            self.push(1)
        } else {
            self.push(0)
        }
    }

    pub(crate) fn set_gt(&mut self) -> Result<()> {
        let val = self.pop()?;
        if (val as i64) > 0 {
            self.push(1)
        } else {
            self.push(0)
        }
    }

    pub(crate) fn br_a(&mut self, addr: u64) -> Result<()> {
        unimplemented!("branch to specific address is unstable")
    }

    pub(crate) fn br(&mut self, off: i32) -> Result<()> {
        let off = if off > 0 {
            self.ip.wrapping_add(off as usize)
        } else {
            let off = (-off) as usize;
            self.ip.wrapping_sub(off as usize)
        };

        if off > self.fn_info.ins.len() {
            Err(Error::InvalidInstructionOffset(off))
        } else {
            self.ip = off;
            Ok(())
        }
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
        let fp = self.get_fn_by_id(id)?;

        let bp = self.sp;

        self.push(self.bp as u64)?;
        self.push(self.ip as u64)?;
        self.push(self.fn_id as u64)?;

        self.stack_alloc(fp.loc_slots)?;

        self.fn_id = id as usize;
        self.fn_info = fp;
        self.ip = 0;
        self.bp = bp;

        Ok(())
    }

    pub(crate) fn ret(&mut self) -> Result<()> {
        let old_bp = self
            .stack_slot_get(self.bp)
            .map_err(|_| Error::StackUnderflow)?;
        let old_ip = self
            .stack_slot_get(self.bp + 1)
            .map_err(|_| Error::StackUnderflow)?;
        let old_fn = self
            .stack_slot_get(self.bp + 2)
            .map_err(|_| Error::StackUnderflow)?;
        let truncate_to = self.bp - self.fn_info.param_slots as usize;

        let fp = self.get_fn_by_id(old_fn as u32)?;

        self.sp = truncate_to;

        self.fn_info = fp;
        self.bp = old_bp as usize;
        self.ip = old_ip as usize;
        self.fn_id = old_fn as usize;

        Ok(())
    }

    pub(crate) fn call_by_name(&mut self, name_idx: u32) -> Result<()> {
        let global = self.get_global_by_id(name_idx)?;
        let name = String::from_utf8_lossy(&global.bytes);
        match name.as_ref() {
            "putint" => self.print_i(),
            "putdouble" => self.print_f(),
            "putstr" => self.print_s(),
            "putchar" => self.print_c(),
            "getint" => self.scan_i(),
            "getdouble" => self.scan_f(),
            "getchar" => self.scan_c(),
            _ => {
                let function_id = *self
                    .function_idx
                    .get(name.as_ref())
                    .ok_or_else(|| Error::UnknownFunction(name_idx))?;
                self.call(function_id as u32)
            }
        }
    }

    pub(crate) fn scan_i(&mut self) -> Result<()> {
        let mut err = None;
        let val = try_read!(
            // HACK: Whenever an IOError is encountered, that error is forwarded to the error variable outside. This allows us to use try_read!() to parse the value.
            // Remained here until better options are available.
            "{}",
            (&mut self.stdin)
                .map(|x| match x {
                    Ok(x) => Some(x),
                    Err(e) => {
                        err = Some(e);
                        None
                    }
                })
                .flatten()
        )
        .map_err(|_| err.map(|e| Error::IoError(e)).unwrap_or(Error::ParseError))?;
        self.push(val)
    }

    pub(crate) fn scan_c(&mut self) -> Result<()> {
        let ch = self.stdin.next().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Input does not provide anything",
            )
        })??;
        let val = ch as u64;
        self.push(val)
    }

    pub(crate) fn scan_f(&mut self) -> Result<()> {
        let mut err = None;
        let val: f64 = try_read!(
            "{}",
            (&mut self.stdin)
                .map(|x| match x {
                    Ok(x) => Some(x),
                    Err(e) => {
                        err = Some(e);
                        None
                    }
                })
                .flatten()
        )
        .map_err(|_| err.map(|e| Error::IoError(e)).unwrap_or(Error::ParseError))?;
        self.push(reinterpret_t(val))
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
            .write_fmt(format_args!("{:.6}", f))
            .map_err(|err| err.into())
    }

    pub(crate) fn print_s(&mut self) -> Result<()> {
        // // TODO: Should we use address + length or a simple CString?
        // Print the specified global value
        let id = self.pop()?;
        if id > u32::max_value() as u64 {
            return Err(Error::ArithmeticErr);
        }
        let global = self.get_global_by_id(id as u32)?;
        self.stdout.write_all(&global.bytes)?;
        // let len = self.pop()?;
        // for i in 0..len {
        //     let addr = addr + i;
        //     let val = self.access_mem_get::<u8>(addr)?;
        //     self.stdout.write_all(&[val])?;
        // }
        Ok(())
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
