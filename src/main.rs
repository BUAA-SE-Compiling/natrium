use logos::{Lexer, Logos};
use r0syntax::token::Token;

static input: &str = r#"
fn main() -> int {
    // this is great!
    let i: int = 123;
    let b: float = 1234.5678e9;
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
    println!("{:#?}", r);
}
