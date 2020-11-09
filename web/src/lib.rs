use natrium::util::pretty_print_error;
use r0vm::s0::S0;
use std::fmt::Write;
use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    unsafe { console::log_1(&JsValue::from_str("Hello world!")) };

    Ok(())
}

fn compile_internal(input: &str) -> Result<S0, String> {
    let l = r0syntax::lexer::spanned_lexer(input);
    let mut p = r0syntax::parser::Parser::new(l);
    let r = p.parse();
    let program = match r {
        Ok(p) => p,
        Err(e) => {
            let mut err = String::new();
            if let Some(span) = e.span {
                pretty_print_error(&mut err, input, &format!("{:?}", e.kind), span)
                    .map_err(|x| x.to_string())?;
            } else {
                writeln!(err, "{:?}", e.kind).map_err(|x| x.to_string())?;
            }
            return Err(err);
        }
    };

    let s0 = match r0codegen::generator::compile(&program) {
        Ok(p) => p,
        Err(e) => {
            let mut err = String::new();
            if let Some(span) = e.span {
                pretty_print_error(&mut err, input, &format!("{:?}", e.kind), span)
                    .map_err(|x| x.to_string())?;
            } else {
                writeln!(err, "{:?}", e.kind).map_err(|x| x.to_string())?;
            }
            return Err(err);
        }
    };

    Ok(s0)
}

#[wasm_bindgen]
pub fn compile(input: &str) -> Result<String, JsValue> {
    compile_internal(input)
        .map_err(|x| JsValue::from_str(&x))
        .map(|x| x.to_string())
}

#[wasm_bindgen]
pub fn run(input: &str) -> Result<String, JsValue> {
    todo!()
}
