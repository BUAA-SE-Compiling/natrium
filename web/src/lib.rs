use bytes::Bytes;
use natrium::util::pretty_print_error;
use r0vm::s0::S0;
use std::{fmt::Write as FmtWrite, io, io::Write};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
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
            let mut err = Vec::new();
            if let Some(span) = e.span {
                pretty_print_error(&mut err, input, &format!("{:?}", e.kind), span)
                    .map_err(|x| x.to_string())?;
            } else {
                writeln!(err, "{:?}", e.kind).map_err(|x| x.to_string())?;
            }
            return Err(unsafe { String::from_utf8_unchecked(err) });
        }
    };

    let s0 = match r0codegen::generator::compile(&program) {
        Ok(p) => p,
        Err(e) => {
            let mut err = Vec::new();
            if let Some(span) = e.span {
                pretty_print_error(&mut err, input, &format!("{:?}", e.kind), span)
                    .map_err(|x| x.to_string())?;
            } else {
                writeln!(err, "{:?}", e.kind).map_err(|x| x.to_string())?;
            }
            return Err(unsafe { String::from_utf8_unchecked(err) });
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
pub fn run(
    input: &str,
    read_chunk: js_sys::Function,
    write_chunk: js_sys::Function,
) -> Result<(), JsValue> {
    let code = compile_internal(input).map_err(|x| JsValue::from_str(&x))?;
    let mut stdout = JsStdioAdaptor::new(read_chunk, write_chunk);
    let mut var_name = io::empty();
    let mut vm = r0vm::vm::R0Vm::new(&code, Box::new(var_name), Box::new(stdout)).map_err(|x| {
        let mut s = String::new();
        write!(s, "{:?}", x).unwrap();
        s
    })?;
    vm.run_to_end().unwrap();
    Ok(())
}

struct JsStdioAdaptor {
    read_chunk: js_sys::Function,
    write_chunk: js_sys::Function,
    pending_read: Option<Bytes>,
}

impl JsStdioAdaptor {
    pub fn new(read_chunk: js_sys::Function, write_chunk: js_sys::Function) -> JsStdioAdaptor {
        JsStdioAdaptor {
            read_chunk,
            write_chunk,
            pending_read: None,
        }
    }
}

impl io::Read for JsStdioAdaptor {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(mut data) = self.pending_read.take() {
            if data.len() > buf.len() {
                let remianing = data.split_off(buf.len());
                self.pending_read = Some(remianing);
                buf.copy_from_slice(&data[..]);
                Ok(buf.len())
            } else {
                let buf_sub = &mut buf[0..data.len()];
                buf_sub.copy_from_slice(&data[..]);
                Ok(data.len())
            }
        } else {
            let val: JsValue = self.read_chunk.call0(&JsValue::null()).unwrap();
            if !val.is_string() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Value is not a string",
                ));
            }
            let val_str = val.as_string().unwrap();
            let mut data = Bytes::from(val_str);
            if data.len() > buf.len() {
                let remianing = data.split_off(buf.len());
                self.pending_read = Some(remianing);
                buf.copy_from_slice(&data[..]);
                Ok(buf.len())
            } else {
                let buf_sub = &mut buf[0..data.len()];
                buf_sub.copy_from_slice(&data[..]);
                Ok(data.len())
            }
        }
    }
}

impl std::io::Write for JsStdioAdaptor {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let js_buf = js_sys::Uint8Array::from(buf);
        self.write_chunk.call1(&JsValue::null(), &js_buf).unwrap();
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
