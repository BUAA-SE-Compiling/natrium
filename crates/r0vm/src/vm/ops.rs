//! Implementation for all operation codes for R0VM

use super::Slot;
use crate::error::*;

/// Reinterpret slot x as f64
#[inline]
fn reinterpret_u_f(x: Slot) -> f64 {
    unsafe { std::intrinsics::transmute::<u64, f64>(x) }
}

/// Reinterpret slot x as u64
#[inline]
fn reinterpret_f_u(x: f64) -> Slot {
    unsafe { std::intrinsics::transmute::<f64, u64>(x) }
}

impl<'src> super::R0Vm<'src> {
    #[inline]
    fn pop2(&mut self) -> Result<(u64, u64)> {
        let lhs = self.pop()?;
        let rhs = self.pop()?;
        Ok((lhs, rhs))
    }

    #[inline]
    fn pop2f(&mut self) -> Result<(f64, f64)> {
        let lhs = reinterpret_u_f(self.pop()?);
        let rhs = reinterpret_u_f(self.pop()?);
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
        let top = *self.stack.last().ok_or(Error::AttemptToReadEmptyStack)?;
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
        let (lhs, rhs) = self.pop2()?;
        self.push(lhs.wrapping_div(rhs))?;
        Ok(())
    }

    pub(crate) fn add_f(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2f()?;
        let res = reinterpret_f_u(lhs + rhs);
        self.push(res)?;
        Ok(())
    }

    pub(crate) fn sub_f(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2f()?;
        let res = reinterpret_f_u(lhs - rhs);
        self.push(res)?;
        Ok(())
    }

    pub(crate) fn mul_f(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2f()?;
        let res = reinterpret_f_u(lhs * rhs);
        self.push(res)?;
        Ok(())
    }

    pub(crate) fn div_f(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2f()?;
        let res = reinterpret_f_u(lhs / rhs);
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

    pub(crate) fn cmp_i(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn cmp_ui(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn cmp_f(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn neg_i(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn neg_f(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn itof(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn ftoi(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn shr_l(&mut self) -> Result<()> {
        let (lhs, rhs) = self.pop2()?;
        let rhs = (rhs & u32::max_value() as u64) as u32;
        // logical shift
        self.push(lhs.wrapping_shr(rhs))?;
        Ok(())
    }

    pub(crate) fn br_a(&mut self, addr: u64) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn br(&mut self, off: i32) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn bz(&mut self, off: i32) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn bnz(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn bl(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn bg(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn blz(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn bgz(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn call(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn ret(&mut self) -> Result<()> {
        unimplemented!()
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
        unimplemented!()
    }

    pub(crate) fn print_c(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn print_f(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn print_s(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn print_ln(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn halt(&mut self) -> Result<()> {
        unimplemented!()
    }
}
