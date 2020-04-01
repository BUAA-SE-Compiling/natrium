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
    pub fn alloc(layout: Layout) -> Result<ManagedMemory> {
        if layout.size() == 0 {
            return Err(Error::AllocZero);
        }
        let mem = unsafe { std::alloc::alloc_zeroed(layout) };
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
    pub fn get_ptr(&self) -> *mut u8 {
        self.ptr
    }
}

impl Drop for ManagedMemory {
    fn drop(&mut self) {
        unsafe { std::alloc::dealloc(self.ptr, self.layout) }
    }
}

// pub struct StackMemory {
//     alloc_base: *mut u64,
//     stack_base: *mut u64,
//     bp: *mut u64,
//     sp: *mut u64,
//     stack_size: usize,
// }

// impl StackMemory {
//     /// Allocate a stack with `size` slots.
//     pub fn alloc(size: usize) -> Result<StackMemory> {
//         unsafe {
//             let mem_base = std::alloc::alloc_zeroed(
//                 Layout::from_size_align(size, std::mem::align_of::<u64>()).unwrap(),
//             ) as *mut u64;
//             let stack_top = mem_base.add(size);
//             Ok(StackMemory {
//                 alloc_base: mem_base,
//                 stack_base: stack_top,
//                 bp: stack_top,
//                 sp: stack_top,
//                 stack_size: size,
//             })
//         }
//     }

//     pub fn push(&mut self, val: u64) -> Result<()> {
//         if self.sp <= self.stack_base {
//             Err(Error::StackOverflow)
//         } else {
//             unsafe {
//                 self.sp = self.sp.sub(1);
//                 *self.sp = val;
//             }
//             Ok(())
//         }
//     }

//     pub fn pop(&mut self) -> Result<u64> {
//         if self.sp >= self.alloc_base {
//             Err(Error::StackUnderflow)
//         } else {
//             let val = unsafe {
//                 let val = *self.sp;
//                 self.sp = self.sp.add(1);
//                 val
//             };
//             Ok(val)
//         }
//     }

//     pub fn stack_alloc(&mut self, cnt: usize) -> Result<()> {
//         let new_ptr = unsafe { self.sp.sub(cnt) };
//         if new_ptr < self.alloc_base {
//             Err(Error::StackOverflow)
//         } else {
//             self.sp = new_ptr;
//             Ok(())
//         }
//     }

//     pub fn get_relative_to_bp(&self, offset: isize) -> Result<u64> {
//         let ptr = unsafe { self.bp.offset(offset) };
//         if ptr > self.stack_base || ptr < self.alloc_base {
//             Err(Error::InvalidAddress(0))
//         } else {
//             Ok(unsafe { *ptr })
//         }
//     }

//     pub fn bp(&self) -> *mut u64 {
//         self.bp
//     }

//     pub fn sp(&self) -> *mut u64 {
//         self.sp
//     }

//     pub fn size(&self) -> usize {
//         self.stack_size
//     }

//     pub unsafe fn set_bp(&mut self, bp: *mut u64) {
//         assert!(bp >= self.sp);
//         assert!(bp >= self.stack_base);
//         assert!(bp <= self.alloc_base);
//         self.bp = bp;
//     }

//     pub unsafe fn set_sp(&mut self, sp: *mut u64) {
//         assert!(sp <= self.bp);
//         assert!(sp >= self.stack_base);
//         assert!(sp <= self.alloc_base);
//         self.sp = sp;
//     }
// }

pub fn stack_idx_to_vm_addr(idx: usize) -> u64 {
    R0Vm::STACK_START - (idx as u64) * 8
}

#[inline]
fn round_up_to_multiple(x: u64, mult: u64) -> u64 {
    x + (mult - x % mult)
}

impl<'src> R0Vm<'src> {
    pub const HEAP_START: u64 = 0x00000001_00000000;
    pub const STACK_START: u64 = 0xffffffff_ffffffff;
    pub const STACK_END: u64 = 0xffffffff_fff00000;

    // * Heap stuff -->

    /// Find the piece of heap memory by address.
    /// Returns the managed memory instance and the index offset from start.
    pub fn get_heap_mem_managed_ref(&self, addr: u64) -> Result<(&ManagedMemory, usize)> {
        let range = self
            .heap
            .range((std::ops::Bound::Unbounded, std::ops::Bound::Included(addr)));
        // Get the last memory chunk that is less or equal than address
        let (start_addr, mem) = range.last().ok_or(Error::InvalidAddress(addr))?;
        let addr_offset = addr - start_addr;
        if addr_offset > mem.len() as u64 {
            Err(Error::InvalidAddress(addr))
        } else {
            Ok((mem, addr_offset as usize))
        }
    }

    fn get_heap_mem_ptr<T>(&self, addr: u64) -> Result<*mut u8> {
        assert!(addr >= R0Vm::HEAP_START && addr < R0Vm::STACK_END);

        let alignment_of_t = std::mem::align_of::<T>();
        if addr % alignment_of_t as u64 != 0 {
            return Err(Error::UnalignedAccess(addr));
        }

        let (slice, offset) = self.get_heap_mem_managed_ref(addr)?;
        let sizeof_t = std::mem::size_of::<T>();

        // Check remaining space is enough
        if sizeof_t + offset > slice.len() {
            return Err(Error::InvalidAddress(addr));
        }

        let t_ptr = unsafe { slice.get_ptr().add(offset) };
        Ok(t_ptr)
    }

    /// Assuming `mem` is heap memory, get the reference of this memory as `&T`.
    pub unsafe fn heap_mem_ref<T>(&self, addr: u64) -> Result<&T> {
        let t_ptr = self.get_heap_mem_ptr::<T>(addr)?;
        let t_ptr = t_ptr as *mut T;
        Ok(&*t_ptr)
    }

    /// Assuming `mem` is heap memory, get the reference of this memory as `&mut T`.
    pub unsafe fn heap_mem_mut<T>(&self, addr: u64) -> Result<&mut T> {
        let t_ptr = self.get_heap_mem_ptr::<T>(addr)?;
        let t_ptr = t_ptr as *mut T;
        Ok(&mut *t_ptr)
    }

    /// Assuming `mem` is heap memory, get the reference of this memory as `&T`.
    pub unsafe fn heap_mem_get<T>(&self, addr: u64) -> Result<T>
    where
        T: Copy,
    {
        let t_ptr = self.get_heap_mem_ptr::<T>(addr)?;
        let t_ptr = t_ptr as *mut T;
        Ok(*t_ptr)
    }

    /// Assuming `mem` is heap memory, get the reference of this memory as `&mut T`.
    pub unsafe fn heap_mem_set<T>(&self, addr: u64, val: T) -> Result<()> {
        let t_ptr = self.get_heap_mem_ptr::<T>(addr)?;
        let t_ptr = t_ptr as *mut T;
        *t_ptr = val;
        Ok(())
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

    // * Stack stuff -->

    pub fn get_stack_mem<T>(&self) -> Result<T> {
        unimplemented!("Stack layout needs to change")
    }

    // * Misc stuff -->

    /// Access an immutable reference of a piece of memory at `addr`
    pub fn access_mem_get<T>(&self, addr: u64) -> Result<T>
    where
        T: Copy,
    {
        if addr < R0Vm::HEAP_START {
            // Global vars
            unimplemented!("Access global variables")
        } else if addr < R0Vm::STACK_END {
            // Heap vars
            unsafe { self.heap_mem_get::<T>(addr) }
        } else {
            // Stack vars
            unimplemented!("Access stack variables")
        }
    }

    /// Access a mutable reference of a piece of memory at `addr`
    pub fn access_mem_set<T>(&self, addr: u64, val: T) -> Result<()> {
        if addr < R0Vm::HEAP_START {
            // Global vars
            unimplemented!("Access global variables")
        } else if addr < R0Vm::STACK_END {
            // Heap vars
            unsafe { self.heap_mem_set::<T>(addr, val) }
        } else {
            // Stack vars
            unimplemented!("Access stack variables")
        }
    }
}
