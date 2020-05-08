#![cfg(feature = "llvm9")]
use inkwell::{builder::Builder, context::Context, module::Module};

pub struct Codegen<'src, 'ctx> {
    // source: &'src S0,
    ctx: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'src> Codegen<'src> {
    pub fn gen(&mut self) {}

    fn gen_fn(&mut self, func: &FnDef) {
        self.module.add_function(func.name.to_string(), ty, linkage)
    }
}
