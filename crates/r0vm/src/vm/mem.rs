//! Memory-related implementations for R0VM
use super::*;
use std::alloc::Layout;

/// A piece of managed memory owned by the virtual machine
pub struct ManagedMemory {
    ptr: *mut u8,
    layout: Layout,
}

impl ManagedMemory {
    /// Allocate a piece of managed memory using global allocator
    pub unsafe fn alloc(layout: Layout) -> Result<ManagedMemory> {
        if layout.size() == 0 {
            return Err(Error::AllocZero);
        }
        let mem = std::alloc::alloc_zeroed(layout);
        if mem.is_null() {
            return Err(Error::OutOfMemory);
        }
        Ok(ManagedMemory { ptr: mem, layout })
    }

    /// Construct a piece of managed memory using raw pointer and length
    pub unsafe fn new(ptr: *mut u8, layout: Layout) -> ManagedMemory {
        ManagedMemory { ptr, layout }
    }

    /// Length of the memory
    pub fn len(&self) -> usize {
        self.layout.size()
    }

    /// Get the memory as slice
    pub unsafe fn get_slice(&self) -> &[u8] {
        std::slice::from_raw_parts(self.ptr, self.layout.size())
    }

    /// Get the memory as mutable slice
    pub unsafe fn get_slice_mut(&mut self) -> &mut [u8] {
        std::slice::from_raw_parts_mut(self.ptr, self.layout.size())
    }

    /// Get the memory as raw pointer
    pub unsafe fn get_ptr(&mut self) -> *mut u8 {
        self.ptr
    }
}

impl Drop for ManagedMemory {
    fn drop(&mut self) {
        unsafe { std::alloc::dealloc(self.ptr, self.layout) }
    }
}

pub fn stack_idx_to_vm_addr(idx: usize) -> u64 {
    R0Vm::STACK_START - (idx as u64) * 8
}

#[inline]
fn round_up_to_multiple(x: u64, mult: u64) -> u64 {
    x + (mult - x % mult)
}

impl<'src> R0Vm<'src> {
    pub fn get_heap_mem(&self, addr: u64) -> Result<(&[u8], usize)> {
        let range = self
            .heap
            .range((std::ops::Bound::Unbounded, std::ops::Bound::Included(addr)));
        // Get the last memory chunk that is less or equal than address
        let (start_addr, mem) = range.last().ok_or(Error::InvalidAddress(addr))?;
        let addr_offset = addr - start_addr;
        if addr_offset > mem.len() as u64 {
            Err(Error::InvalidAddress(addr))
        } else {
            Ok((unsafe { mem.get_slice() }, addr_offset as usize))
        }
    }

    pub fn get_heap_mem_mut(&mut self, addr: u64) -> Result<(&mut [u8], usize)> {
        let range = self
            .heap
            .range_mut((std::ops::Bound::Unbounded, std::ops::Bound::Included(addr)));
        // Get the last memory chunk that is less or equal than address
        let (start_addr, mem) = range.last().ok_or(Error::InvalidAddress(addr))?;
        let addr_offset = addr - start_addr;
        if addr_offset > mem.len() as u64 {
            Err(Error::InvalidAddress(addr))
        } else {
            Ok((unsafe { mem.get_slice_mut() }, addr_offset as usize))
        }
    }

    /// Allocate a piece of memory of length `len` onto heap. Returns address.
    pub fn alloc_heap(&mut self, len: usize, alignment: usize) -> Result<u64> {
        let mem = unsafe { ManagedMemory::alloc(Layout::from_size_align(len, alignment)?)? };
        let mem_addr = self
            .heap
            .last_key_value()
            .map(|(k, v)| round_up_to_multiple(*k + v.len() as u64, alignment as u64))
            .unwrap_or(R0Vm::HEAP_START);
        self.heap.insert(mem_addr, mem);
        Ok(mem_addr)
    }

    /// Free a piece of memory specified by `addr`. Will return an error if
    /// memory is not the very same address as the allocator returns.
    pub fn free_heap(&mut self, addr: u64) -> Result<()> {
        let mem = self.heap.remove(&addr).ok_or(Error::InvalidDeallocation)?;
        drop(mem);
        Ok(())
    }
}
