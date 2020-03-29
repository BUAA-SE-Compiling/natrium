use crate::error::*;
use crate::opcodes::Op::*;
use crate::opcodes::*;
use crate::s0::*;
use crate::vm::*;

#[test]
pub fn base_test() {
    let inst = vec![
        Push(1),
        Push(2),
        AddI,
        IToF,
        Push(unsafe { std::mem::transmute(0.4f64) }),
        MulF,
    ];
    let func = FnDef {
        max_stack: 0,
        param_slots: 0,
        ret_slots: 0,
        ins: inst,
    };
    let s0 = S0 {
        globals: vec![],
        functions: vec![func],
    };
    let stdin = Vec::<u8>::new();
    let mut stdout = Vec::<u8>::new();
    let mut stdin = stdin.as_slice();
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
