#![feature(map_first_last)]

pub mod error;
pub mod opcodes;
pub mod s0;
mod tests;
pub mod vm;

#[macro_export]
/// Create an in-memory representation for s0 binary
macro_rules! s0_bin {
    (
        $(
            // TODO: global variable declaration
            let $(const)? ($val:expr);
        )*
        $(
            fn $name:ident $max_stack:literal $param:literal -> $ret:literal {
                $($inst:expr $(,)?)*
            }
        )+
    ) => {{
        use crate::opcodes::Op::*;
        let mut fns = vec![];
        $({
            let max_stack = $max_stack;
            let inst = vec![$($inst),*];
            let func = FnDef{
                max_stack,
                param_slots: $param,
                ret_slots: $ret,
                ins: inst,
            };
            fns.push(func);
        })+
        let s0 = S0{
            globals: vec![],
            functions: fns,
        };
        s0
    }};
}
