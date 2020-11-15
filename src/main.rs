use clap::Clap;
use logos::{Lexer, Logos};
use natrium::util::pretty_print_error;
use r0syntax::{ast::Program, span::Span, token::Token};
use std::{
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
};

fn main() {
    let opt = Opt::parse();
    let input = std::fs::read_to_string(&opt.input).expect("Unable to read input file");

    let output_file = get_output(&opt);

    let lexer = r0syntax::lexer::spanned_lexer(&input);
    if opt.emit == EmitTarget::Token {
        dump_lex(lexer, output_file);
    }

    let program = parser(lexer, &input);
    if opt.emit == EmitTarget::Ast {
        dump_ast(program, output_file);
    }

    let s0 = compile_s0(&program, &input);

    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut vm = r0vm::vm::R0Vm::new(&s0, &mut stdin, &mut stdout).unwrap();

    match vm.run_to_end() {
        Ok(_) => {}
        Err(e) => {
            println!("{}", &s0);
            println!("{}", e);
            println!("{}", vm.debug_stack());
        }
    }
}

/// Get real output path based on options. None means stdout.
fn get_output(opt: &Opt) -> Option<PathBuf> {
    match opt.output.as_deref() {
        Some("-") => None,
        Some(x) => PathBuf::from_str(x).ok(),
        None => {
            let filename = opt
                .input
                .file_stem()
                .and_then(|x| x.to_str())
                .unwrap_or("a");
            let ext = match opt.emit {
                EmitTarget::O0 => "o0",
                EmitTarget::Text => "s0",
                EmitTarget::Token => "tokenstream",
                EmitTarget::Ast => "ast",
            };
            let out_file = format!("{}.{}", filename, ext);
            Some(out_file.into())
        }
    }
}

fn dump_lex<T>(lexer: T, output: Option<PathBuf>) -> !
where
    T: Iterator<Item = (Token, Span)>,
{
    todo!("Dump lex result");
    std::process::exit(0);
}

fn dump_ast(ast: Program, output: Option<PathBuf>) -> ! {
    todo!("Dump AST");
    std::process::exit(0);
}

fn parser<T>(lexer: T, input: &str) -> Program
where
    T: Iterator<Item = (Token, Span)>,
{
    let mut p = r0syntax::parser::Parser::new(lexer);
    let r = p.parse();

    match r {
        Ok(p) => p,
        Err(e) => {
            if let Some(span) = e.span {
                pretty_print_error(
                    &mut std::io::stdout(),
                    &input,
                    &format!("{:?}", e.kind),
                    span,
                )
                .unwrap();
            } else {
                println!("{:?}", e.kind);
            }
            std::process::exit(1);
        }
    }
}

fn compile_s0(program: &Program, input: &str) -> r0vm::s0::S0 {
    match r0codegen::generator::compile(program) {
        Ok(p) => p,
        Err(e) => {
            if let Some(span) = e.span {
                pretty_print_error(
                    &mut std::io::stdout(),
                    &input,
                    &format!("{:?}", e.kind),
                    span,
                )
                .unwrap();
            } else {
                println!("{:?}", e.kind);
            }
            std::process::exit(1);
        }
    }
}

#[derive(Clap, Debug)]
struct Opt {
    /// Input file
    pub input: PathBuf,

    /// Emit target
    ///
    /// O0: binary object code;
    /// Text: text format code;
    /// Token: token stream;
    /// Ast: abstract syntax tree
    #[clap(long, default_value = "o0")]
    pub emit: EmitTarget,

    /// Output file. Defaults to `<input_file_name>.o0|s0|tt|ast`
    #[clap(long, short)]
    pub output: Option<String>,

    /// Interpret the input file with virtual machine; alias: `--run`
    #[cfg(feature = "vm")]
    #[clap(short = 'i', long, alias = "run")]
    pub interpret: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum EmitTarget {
    O0,
    Text,
    Token,
    Ast,
}

impl FromStr for EmitTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "o0" => EmitTarget::O0,
            "text" | "s0" => EmitTarget::Text,
            "token" | "lex" => EmitTarget::Token,
            "ast" | "parse" => EmitTarget::Ast,
            _ => return Err(format!("Expected one of: o0, text, token, ast; got: {}", s)),
        })
    }
}
