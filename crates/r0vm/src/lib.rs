#![feature(map_first_last)]

pub mod error;
pub mod opcodes;
pub mod s0;
mod tests;
mod util;
pub mod vm;

#[macro_export]
/// Create an in-memory representation for s0 binary
macro_rules! s0_bin {
    (
        $(
            // TODO: global variable declaration
            let $(const)? $val:expr;
        )*
        $(
            fn $name:ident $loc_slots:literal $param:literal -> $ret:literal {
                $($inst:expr $(,)?)*
            }
        )+
    ) => {{
        use $crate::opcodes::Op::*;
        use $crate::util::IntoBytes;
        let mut globals = vec![];

        $({
            let bytes = $val.into_bytes();
            let glob = GlobalValue {
                is_const: false,
                bytes
            };
            globals.push(glob);
        })*

        let mut fns = vec![];
        $({
            let name = stringify!($name);
            let bytes = name.into_bytes();
            let glob = GlobalValue{ is_const:true, bytes };
            let name_idx = globals.len();
            globals.push(glob);

            let loc_slots = $loc_slots;
            let inst = vec![$($inst),*];
            let func = FnDef{
                name: name_idx as u32,
                loc_slots,
                param_slots: $param,
                ret_slots: $ret,
                ins: inst,
            };
            fns.push(func);
        })+
        let s0 = S0{
            globals,
            functions: fns,
        };
        s0
    }};
}
