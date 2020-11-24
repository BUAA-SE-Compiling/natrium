use clap::{AppSettings, Clap};
use clap::{FromArgMatches, IntoApp};
use crossterm::{style::Attribute, ExecutableCommand, QueueableCommand};
use natrium::util::pretty_print_error;
use r0vm::{opcodes::Op, s0::io::WriteBinary, vm::R0Vm};
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
            Err(e) => println!("Error: {:?}", e),
        }
    };
}

fn debug_run(s0: &S0) {
    let mut vm = create_vm_stdio(s0);
    let mut breakpoints = bimap::BiBTreeMap::<usize, Breakpoint>::new();

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
                            .help_template("Commands:\r\n{subcommands}")
                            .override_usage("<command> [args]");
                        let res = app
                            .try_get_matches_from_mut(&s)
                            .map(|x| DebuggerInst::from_arg_matches(&x));
                        match res {
                            Ok(s) => match exec_opt(s, &mut vm, &mut breakpoints) {
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

fn exec_opt(
    opt: DebuggerInst,
    vm: &mut r0vm::vm::R0Vm,
    breakpoints: &mut bimap::BiBTreeMap<usize, Breakpoint>,
) -> InstructionResult {
    match opt {
        DebuggerInst::Run => print_unwrap! {
            vm.run_to_end_inspect(|vm| cur_breakpoint(vm, breakpoints).is_none()),
            Ok(_) => {
                if vm.is_at_end() {
                    println!("Program exited without error");
                    return InstructionResult::Reset;
                } else {
                    if let Some(b) = cur_breakpoint(vm, breakpoints){
                        println!("At breakpoint {}: {}", b, vm.cur_stack_info().unwrap());
                    }
                    print_vm_next_instruction(vm, None);
                }
            }
        },
        DebuggerInst::Step => print_unwrap! {
            vm.step(),
            Ok(executed_op) => {
                if vm.is_at_end() {
                    println!("Program exited without error");
                    return InstructionResult::Reset;
                } else {
                    print_vm_next_instruction(vm, Some(executed_op));
                }
            }
        },
        DebuggerInst::Finish => {
            let current_fn_bp = vm.bp();
            print_unwrap! {
                vm.run_to_end_inspect(|vm| vm.bp() >= current_fn_bp && cur_breakpoint(vm, breakpoints).is_none()),
                Ok(_) => {
                    if let Some(b) = cur_breakpoint(vm, breakpoints){
                        println!("At breakpoint {}: {}", b, vm.cur_stack_info().unwrap());
                    }
                    print_vm_next_instruction(vm, None);
                }
            }
        }
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
        DebuggerInst::Breakpoint(b) => {
            let pos = b.position;
            match vm.get_fn_by_name(&pos.function_name) {
                Ok(id) => {
                    let def = vm.get_fn_by_id(id).unwrap();
                    if pos.offset < def.ins.len() {
                        let max_breakpoint = breakpoints
                            .left_values()
                            .last()
                            .cloned()
                            .map(|x| x + 1)
                            .unwrap_or(0);

                        breakpoints.insert(
                            max_breakpoint,
                            Breakpoint {
                                fn_id: id as u32,
                                offset: pos.offset,
                            },
                        );

                        println!("Added breakpoint {}", max_breakpoint);
                    } else {
                        println!("Error: offset is larger than function length");
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        DebuggerInst::RemoveBreakpoint { id } => {
            if let Some(point) = breakpoints.get_by_left(&id) {
                let fn_name = vm.get_fn_name_by_id(point.fn_id).unwrap();
                println!("Remove breakpoint #{} at {}:{}", id, fn_name, point.offset);
            } else {
                println!("No such breakpoint was found.");
            }
        }
        DebuggerInst::ListBreakpoint => {
            for (id, point) in breakpoints.iter() {
                let fn_name = vm.get_fn_name_by_id(point.fn_id).unwrap();
                println!("#{}: {}:{}", id, fn_name, point.offset);
            }
        }
        DebuggerInst::Exit => return InstructionResult::Exit,
        DebuggerInst::Reset => {
            return InstructionResult::Reset;
        }
    }
    InstructionResult::None
}

fn print_vm_next_instruction(vm: &R0Vm, executed_op: Option<Op>) {
    let fn_info = vm.fn_info();
    let ip = vm.ip();
    if let Some(executed_op) = executed_op {
        println!("        | {:?}", executed_op);
    }
    if let Some(next_op) = fn_info.ins.get(ip) {
        println!("-> {:4} | {:?}", ip, next_op);
    } else {
        println!("-> {:4}| Function end", ip);
    }
    if let Ok(cur) = vm.cur_stack_info() {
        println!("at: {}", cur)
    }
}

#[inline]
fn cur_breakpoint(vm: &R0Vm, breakpoints: &bimap::BiBTreeMap<usize, Breakpoint>) -> Option<usize> {
    let fn_id = vm.fn_id() as u32;
    let ip = vm.ip();
    breakpoints
        .get_by_right(&Breakpoint { fn_id, offset: ip })
        .copied()
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

    /// Dump the assembly in human-readable format
    #[clap(long)]
    pub dump: bool,
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
    pub position: BreakpointRef,
}

#[derive(Debug)]
struct BreakpointRef {
    pub function_name: String,
    pub offset: usize,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Breakpoint {
    pub fn_id: u32,
    pub offset: usize,
}

impl FromStr for BreakpointRef {
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
            (Some(name), Some(Some(offset))) => Ok(BreakpointRef {
                function_name: name.into(),
                offset,
            }),
            (Some(name), None) => Ok(BreakpointRef {
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

    /// Remove breakpoint. [alias: rb]
    #[clap(alias = "rb")]
    RemoveBreakpoint { id: usize },

    /// List all breakpoints [alias: rb]
    #[clap(alias = "lb")]
    ListBreakpoint,

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
