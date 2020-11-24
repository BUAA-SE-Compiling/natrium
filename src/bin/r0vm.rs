use clap::{AppSettings, Clap};
use clap::{FromArgMatches, IntoApp};
use crossterm::{style::Attribute, ExecutableCommand, QueueableCommand};
use natrium::util::pretty_print_error;
use r0vm::{s0::io::WriteBinary, vm::R0Vm};
use r0vm::{s0::S0, vm};
use std::{io::stdout, path::PathBuf, str::FromStr};

pub fn main() {
    let opt = Opt::parse();

    let mut file = match std::fs::File::open(&opt.file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Cannot open file {}: {}", opt.file.to_string_lossy(), e);
            return;
        }
    };

    let s0 = match S0::read_binary(&mut file) {
        Ok(Some(s)) => s,
        Ok(None) => {
            eprintln!("File is not valid s0");
            return;
        }
        Err(e) => {
            eprintln!("File is not valid s0: {}", e);
            return;
        }
    };

    if opt.debug {
        debug_run(&s0)
    } else {
        run(&s0)
    }
}

fn run(s0: &S0) {
    let mut vm = create_vm_stdio(s0);
    match vm.run_to_end() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Runtime error: {}", e);
            eprintln!("{}", vm.debug_stack());
            std::process::exit(1);
        }
    };
}

fn create_vm_stdio(s0: &S0) -> R0Vm {
    let stdin = std::io::stdin();
    let stdout = stdout();
    vm::R0Vm::new(s0, Box::new(stdin), Box::new(stdout)).expect("Failed to create virtual machine")
}

macro_rules! print_unwrap {
    ($calc:expr,$p:pat => $if_true:block) => {
        match $calc {
            $p => $if_true,
            Err(e) => println!("{:?}", e),
        }
    };
}

fn debug_run(s0: &S0) {
    let mut vm = create_vm_stdio(s0);

    let mut terminal = rustyline::Editor::<()>::with_config(
        rustyline::Config::builder().max_history_size(100).build(),
    );
    loop {
        stdout()
            .queue(crossterm::style::SetForegroundColor(
                crossterm::style::Color::White,
            ))
            .unwrap()
            .execute(crossterm::style::SetAttribute(Attribute::Bold))
            .unwrap();

        let line = terminal.readline("navm |><> ");

        stdout()
            .queue(crossterm::style::SetForegroundColor(
                crossterm::style::Color::Reset,
            ))
            .unwrap()
            .execute(crossterm::style::SetAttribute(Attribute::Reset))
            .unwrap();

        match line {
            Ok(mut line) => {
                // If line is empty, repeat the last instruction
                if line.trim().is_empty() {
                    match terminal.history().last().cloned() {
                        Some(history_line) => line = history_line,
                        None => {
                            continue;
                        }
                    };
                } else {
                    terminal.add_history_entry(&line);
                }

                let words = shell_words::split(&line);
                match words {
                    Ok(s) => {
                        let mut app = DebuggerInst::into_app()
                            .setting(AppSettings::NoBinaryName)
                            .setting(AppSettings::InferSubcommands)
                            .help_template(
                                r"
Commands:
{subcommands}",
                            )
                            .override_usage("<command> [args]");
                        let res = app
                            .try_get_matches_from_mut(&s)
                            .map(|x| DebuggerInst::from_arg_matches(&x));
                        match res {
                            Ok(s) => match exec_opt(s, &mut vm) {
                                InstructionResult::Exit => {
                                    break;
                                }
                                InstructionResult::None => {}
                                InstructionResult::Reset => {
                                    vm = create_vm_stdio(s0);
                                }
                            },
                            Err(e) => match e.kind {
                                clap::ErrorKind::DisplayHelp => println!("{}", e),
                                _ => println!("{}", e.to_string().lines().next().unwrap()),
                            },
                        };
                    }
                    Err(e) => println!("Invalid input: {}!", e),
                }
            }

            Err(rustyline::error::ReadlineError::Eof)
            | Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("Interrupted.");
                break;
            }
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }
}

fn exec_opt(opt: DebuggerInst, vm: &mut r0vm::vm::R0Vm) -> InstructionResult {
    match opt {
        DebuggerInst::Run => print_unwrap! {
            vm.run_to_end(),
            Ok(_) => {
                println!("Program exited without error");
                return InstructionResult::Reset;
            }
        },
        DebuggerInst::Step => print_unwrap! {
            vm.step(),
            Ok(executed_op) => {
                if vm.is_at_end() {
                    println!("Program exited without error");
                    return InstructionResult::Reset;
                } else {
                    let next_op = vm.fn_info().ins[vm.ip()];
                    println!("   | {:?}", executed_op);
                    println!("-> | {:?}", next_op);
                }
            }
        },
        DebuggerInst::Finish => {}
        DebuggerInst::Backtrace => {
            let (stacktrace, corrupted) = vm.stack_trace();
            for (idx, frame) in stacktrace.into_iter().enumerate() {
                println!("{:4}: {}", idx, frame);
            }
            if corrupted {
                println!("The stack corrupted here");
            }
        }
        DebuggerInst::Frame(inst) => {
            match vm.debug_frame(inst.position) {
                Ok(debugger) => println!("{}", debugger),
                Err(_) => println!("The stack is corrupted"),
            };
        }
        DebuggerInst::Breakpoint(b) => todo!("Enable breakpoints"),
        DebuggerInst::Exit => return InstructionResult::Exit,
        DebuggerInst::Reset => {
            return InstructionResult::Reset;
        }
    }
    InstructionResult::None
}

#[derive(Clap, Debug)]
#[clap(name = "r0vm")]
/// A virtual machine for r0 stuff
struct Opt {
    /// The file to run
    pub file: PathBuf,

    /// Run in debugger mode
    #[clap(short, long)]
    pub debug: bool,
}

#[derive(Clap, Debug)]
struct FrameInst {
    /// The frame to show
    #[clap(default_value = "0")]
    pub position: usize,
}

#[derive(Clap, Debug)]
struct BreakpointInst {
    /// Breakpoint position, in format `<function>[:<instruction>]`
    pub position: Breakpoint,
}

#[derive(Debug)]
struct Breakpoint {
    pub function_name: String,
    pub offset: usize,
}

impl FromStr for Breakpoint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.splitn(2, ':');
        let name = it.next().to_owned();
        let inst = it.next().map(|x| usize::from_str(x).ok());
        match (name, inst) {
            (None, _) => {
                Err("No function name supplied. Expected: <function_name>[:<offset>]".into())
            }
            (_, Some(None)) => Err("Offset is not a number".into()),
            (Some(name), Some(Some(offset))) => Ok(Breakpoint {
                function_name: name.into(),
                offset,
            }),
            (Some(name), None) => Ok(Breakpoint {
                function_name: name.into(),
                offset: 0,
            }),
        }
    }
}

#[derive(Clap, Debug)]
enum DebuggerInst {
    /// Run or continue the current execution to end. [alias: r, continue, c]
    #[clap(alias = "r", alias = "continue")]
    Run,
    /// Move one instruction forward. [alias: s, si, n]
    #[clap(alias = "s", alias = "n", alias = "si")]
    Step,
    /// Continue until function returns. [alias: f]
    #[clap(alias = "f")]
    Finish,
    /// Show call stack. [alias: where, stacktrace]
    #[clap(alias = "where", alias = "stacktrace")]
    Backtrace,
    /// Add breakpoint. [alias: b]
    #[clap(alias = "b")]
    Breakpoint(BreakpointInst),
    /// Show function frame
    Frame(FrameInst),
    /// Reset execution to start
    Reset,
    /// Exit the debugger. [alias: q, quit]
    #[clap(alias = "quit")]
    Exit,
}

enum InstructionResult {
    Exit,
    Reset,
    None,
}
