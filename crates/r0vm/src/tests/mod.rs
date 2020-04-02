use super::*;
use crate::error::*;
use crate::opcodes::Op::*;
use crate::opcodes::*;
use crate::s0::*;
use crate::vm::ops::{reinterpret_t, reinterpret_u};
use crate::vm::*;

#[test]
pub fn base_test() {
    let s0 = s0_bin!(
        fn _start 0 0 -> 0 {
            Push(1),
            Push(2),
            AddI,
            IToF,
            Push(unsafe { std::mem::transmute(0.4f64) }),
            MulF,
        }
    );
    let mut stdin = std::io::empty();
    let mut stdout = std::io::sink();
    let mut vm = R0Vm::new(&s0, &mut stdin, &mut stdout).unwrap();
    for _ in 0..3 {
        vm.step().unwrap();
    }
    let stack = vm.stack();
    assert_eq!(stack, &vec![3]);
    for _ in 0..3 {
        vm.step().unwrap();
    }
    let stack = vm.stack();
    assert!((unsafe { std::mem::transmute::<_, f64>(stack[0]) } - 1.2f64).abs() < 1e-10);
}

#[test]
pub fn panic_test() {
    let s0 = s0_bin! (
        fn _start 0 0 -> 0 {
            Panic
        }
    );
    let mut stdin = std::io::empty();
    let mut stdout = std::io::sink();
    let mut vm = R0Vm::new(&s0, &mut stdin, &mut stdout).unwrap();
    let e = vm.run_to_end().unwrap_err();
    assert!(matches!(e, Error::Halt))
}

#[test]
pub fn call_test() {
    let s0 = s0_bin! (
        fn _start 0 0 -> 0 {
            StackAlloc(1),
            Push(1),
            Push(2),
            Call(1),
        }
        fn main 1 2 -> 1 {
            LocA(0)
            LocA(1)
            Load64
            LocA(2)
            Load64
            AddI
            Store64
            Ret
        }
    );
    let mut stdin = std::io::empty();
    let mut stdout = std::io::sink();
    let mut vm = R0Vm::new(&s0, &mut stdin, &mut stdout).unwrap();
    vm.run_to_end().unwrap();
    assert_eq!(vm.stack(), &vec![3])
}

#[test]
pub fn simple_local_var_test() {
    let s0 = s0_bin! (
        fn _start 1 0 -> 0 {
            // store 1
            LocA(0)
            Push(1)
            Store32

            // store 2
            LocA(0)
            Push(4)
            AddI
            Push(2)
            Store16

            // store 3
            LocA(0)
            Push(6)
            AddI
            Push(3)
            Store8

            // load 1
            LocA(0)
            Load32

            LocA(0)
            Push(4)
            AddI
            Load16

            LocA(0)
            Push(6)
            AddI
            Load8
        }
    );
    let mut stdin = std::io::empty();
    let mut stdout = std::io::sink();
    let mut vm = R0Vm::new(&s0, &mut stdin, &mut stdout).unwrap();
    vm.run_to_end().unwrap();
    assert_eq!(vm.stack(), &vec![0x00_03_0002_00000001, 1, 2, 3])
}

#[test]
pub fn simple_alloc_test() {
    let s0 = s0_bin! (
        fn _start 0 0 -> 0 {
            Push(8),
            Alloc,
            Dup,
            Push(0x10008086),
            Store64,
            Load64
        }
    );
    let mut stdin = std::io::empty();
    let mut stdout = std::io::sink();
    let mut vm = R0Vm::new(&s0, &mut stdin, &mut stdout).unwrap();
    vm.run_to_end().unwrap();
    assert_eq!(vm.stack(), &vec![0x10008086])
}

#[test]
pub fn simple_branch_test() {
    let s0 = s0_bin! (
        fn _start 0 0 -> 0 {
            Push(0)
            Push(1)
            CmpI
            Bz(2)
            Br(2)
            Push(3)
            Br(2)
            Push(5)
        }
    );
    let mut stdin = std::io::empty();
    let mut stdout = std::io::sink();
    let mut vm = R0Vm::new(&s0, &mut stdin, &mut stdout).unwrap();
    vm.run_to_end().unwrap();
    assert_eq!(vm.stack(), &vec![5])
}

#[test]
pub fn simple_stdin_test() {
    let s0 = s0_bin! (
        fn _start 0 0 -> 0 {
            ScanC
            ScanF
            ScanI
        }
    );
    let mut stdin = std::io::Cursor::new("A3.1415926e3 1234");
    let mut stdout = std::io::sink();
    let mut vm = R0Vm::new(&s0, &mut stdin, &mut stdout).unwrap();
    vm.run_to_end().unwrap();
    assert_eq!(vm.stack()[0], b'A' as u64);
    assert_eq!(vm.stack()[2], 1234u64);
    assert!((reinterpret_u::<f64>(vm.stack()[1]) - 3.1415926e3f64).abs() < 1e-10);
}
