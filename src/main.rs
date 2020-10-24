use logos::{Lexer, Logos};
use r0syntax::token::Token;

static input: &str = r#"
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
    let l = r0syntax::lexer::spanned_lexer(input);
    // l.for_each(|t| println!("{:?}", t))
    let mut p = r0syntax::parser::Parser::new(l);
    let r = p.parse();
    match r {
        Ok(p) => println!("{:#?}", p),
        Err(e) => {
            println!("{:?}", e.kind);
            if let Some(span) = e.span {
                let start = line_span::find_line_range(input, span.idx);
                let end = line_span::find_line_range(input, span.idx + span.len);
                if start == end {
                    println!("{}", &input[start]);
                } else {
                    let print_range = start.start..end.end;
                    println!("{}", &input[print_range]);
                }
            }
        }
    }
}
