use logos::{Lexer, Logos};
use r0syntax::{span::Span, token::Token};

static INPUT: &str = r#"
fn main() -> int {
    // this is great!
    let i: int = 123;
    let b: int = 1234.5678e9 as int;
    putint(i);
    putstr("this is great!\r\nI can even escape \"quotes\"!");
    return 123;
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

    let s0 = r0codegen::generator::compile(&program).unwrap();

    println!("{:#?}", s0);
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
