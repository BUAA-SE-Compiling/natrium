use std::io::{Read, Write};

use logos::{Lexer, Logos};
use r0syntax::{span::Span, token::Token};

static INPUT: &str = r#"
fn fib(x: int) -> int {
    if x<=1 {
        return 1;
    }
    let result:int = fib(x-1);
    result = result + fib(x-2);
    return result;
}

fn main() -> int {
    let i: int = 0;
    let j: int;
    j = getint();
    while i < j {
        putint(i);
        putchar(32);
        putint(fib(i));
        putln();
        i = i + 1;
    }
    return 0;
}
"#;

fn main() {
    let l = r0syntax::lexer::spanned_lexer(INPUT);
    // l.for_each(|t| println!("{:?}", t))
    let mut p = r0syntax::parser::Parser::new(l);
    let r = p.parse();
    let program = match r {
        Ok(p) => p,
        Err(e) => {
            if let Some(span) = e.span {
                pretty_print_error(INPUT, &format!("{:?}", e.kind), span);
            } else {
                println!("{:?}", e.kind);
            }
            std::process::exit(1);
        }
    };

    let s0 = match r0codegen::generator::compile(&program) {
        Ok(p) => p,
        Err(e) => {
            if let Some(span) = e.span {
                pretty_print_error(INPUT, &format!("{:?}", e.kind), span);
            } else {
                println!("{:?}", e.kind);
            }
            std::process::exit(1);
        }
    };

    // println!("{}", &s0);

    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut vm = r0vm::vm::R0Vm::new(&s0, &mut stdin, &mut stdout).unwrap();
    // loop {
    //     // crossterm::event::read().unwrap();
    //     {
    //         let vm1 = &mut vm;
    //         match vm1.step() {
    //             Ok(op) => {
    //                 // println!("{:?}", op);
    //             }
    //             Err(err) => {
    //                 // println!("{}", err);
    //                 break;
    //             }
    //         }
    //     }
    //     {
    //         let d = vm.debug_stack();
    //         // println!("{}", d);
    //     }
    // }

    match vm.run_to_end() {
        Ok(_) => {}
        Err(e) => {
            println!("{}", &s0);
            println!("{}", e);
            println!("{}", vm.debug_stack());
        }
    }
}

/// Lines to display around error line
const ERR_CONTEXT_LINES: usize = 2;

fn pretty_print_error(input: &str, error: &str, span: Span) {
    println!("{}", error);

    let start = line_span::find_line_range(input, span.idx);
    let end = line_span::find_line_range(input, span.idx + span.len);

    if let Some(line) = line_span::find_prev_line_range(input, span.idx) {
        println!("{}", &input[line]);
    }
    if start == end {
        println!("{}", &input[start.clone()]);
        println!(
            "{:space_width$}{:^^line_width$}",
            "",
            "",
            space_width = span.idx - start.start,
            line_width = span.len
        );
    } else {
        let print_range = start.start..end.end;
        let input_range = input[print_range].lines().collect::<Vec<_>>();

        println!("{}", input_range[0]);
        println!(
            "{:space_width$}{:^^line_width$}",
            "",
            "",
            space_width = span.idx - start.start,
            line_width = start.end - span.idx
        );
        for i in 1..(input_range.len() - 1) {
            println!("{}", input_range[i]);
            println!("{:^^len$}", "", len = input_range[i].chars().count());
        }
        println!("{}", input_range[input_range.len() - 1]);
        println!(
            "{:^^line_width$}",
            "",
            line_width = span.idx + span.len - end.start
        );
    }
    if let Some(line) = line_span::find_next_line_range(input, span.idx + span.len) {
        println!("{}", &input[line]);
    }
}
