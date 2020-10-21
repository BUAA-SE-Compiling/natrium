use logos::{Lexer, Logos};
use r0syntax::token::TokenKind;

static input: &str = r#"
fn main() {
    // this is great!
    let i: int = 123;
    let b: float = 1234.5678e9;
    putint(i);
    putstr("this is great!\r\nI can even escape \"quotes\"!")
}
"#;

fn main() {
    let l = TokenKind::lexer(input);
    l.for_each(|t| println!("{:?}", t))
}
