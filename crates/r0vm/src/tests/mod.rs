use super::*;
use crate::error::*;
use crate::opcodes::Op::*;
use crate::opcodes::*;
use crate::s0::*;
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
            LocA(1)
            Load64
            LocA(2)
            Load64
            AddI
            Ret
        }
    );
    let mut stdin = std::io::empty();
    let mut stdout = std::io::sink();
    let mut vm = R0Vm::new(&s0, &mut stdin, &mut stdout).unwrap();
    vm.run_to_end().unwrap();
    assert_eq!(vm.stack(), &vec![3])
}
