use clap::Clap;
use logos::{Lexer, Logos};
use natrium::util::pretty_print_error;
use r0syntax::{ast::Program, span::Span, token::Token};
use r0vm::s0::io::WriteBinary;
use std::{
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
};

fn main() {
    let opt = Opt::parse();
    let input = std::fs::read_to_string(&opt.input).expect("Unable to read input file");

    let output_file = get_output(&opt);
    let mut output = build_output(output_file, opt.interpret);

    let lexer = r0syntax::lexer::spanned_lexer(&input);
    if !opt.interpret && opt.emit == EmitTarget::Token {
        dump_lex(lexer, output);
    }

    let program = parser(lexer, &input);
    if !opt.interpret && opt.emit == EmitTarget::Ast {
        dump_ast(program, output);
    }

    let s0 = compile_s0(&program, &input);
    if !opt.interpret {
        if opt.emit == EmitTarget::O0 {
            s0.write_binary(&mut output)
                .expect("Failed to write to output");
        } else {
            write!(output, "{}", s0).expect("Failed to write to output");
        }
    } else {
        let mut stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        let mut vm = r0vm::vm::R0Vm::new(&s0, &mut stdin, &mut stdout).unwrap();

        match vm.run_to_end() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", &s0);
                eprintln!("{}", e);
                eprintln!("{}", vm.debug_stack());
            }
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

fn build_output(output: Option<PathBuf>, interpret: bool) -> Box<dyn Write> {
    if interpret {
        return Box::new(std::io::stdout());
    }
    if let Some(path) = output {
        let file = std::fs::File::create(path).expect("Failed to open file");
        Box::new(file)
    } else {
        Box::new(std::io::stdout())
    }
}

fn dump_lex<T>(lexer: T, mut output: Box<dyn Write>) -> !
where
    T: Iterator<Item = (Token, Span)>,
{
    for (token, span) in lexer {
        writeln!(output, "{:?} at {:?}", token, span).expect("Failed to write");
    }
    std::process::exit(0);
}

fn dump_ast(ast: Program, mut output: Box<dyn Write>) -> ! {
    writeln!(output, "{:?}", ast).expect("Failed to write to output");
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
