use r0vm::s0::{io::WriteBinary, S0};

#[test]
fn test_ser() {
    let input = r#"
fn is_odd(x: int) -> int {
    return (x / 2 * 2) - x;
}

fn fastpow(base: int, exp: int) -> int {
    let res: int = 1;
    if exp < 0 {
        return 0;
    }
    while exp > 0 {
        if is_odd(exp) {
            res = res * base;
        }
        base = base * base;
        exp = exp / 2;
    }
    return res;
}

fn main() -> void {
    let base: int;
    let exp: int;
    let count: int;
    count = getint();
    while count > 0 {
        base = getint();
        exp = getint();
        putint(fastpow(base,exp));
        putln();
        count = count - 1;
    }
}
    "#;
    let lexer = r0syntax::lexer::spanned_lexer(&input);
    let program = r0syntax::parser::Parser::new(lexer).parse().unwrap();
    let s0 = r0codegen::generator::compile(&program).unwrap();

    let mut bin = vec![];
    s0.write_binary(&mut bin).unwrap();
    let s0_re = S0::read_binary(&mut &bin[..]).unwrap().unwrap();
    assert_eq!(s0, s0_re);
}
