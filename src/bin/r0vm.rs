use clap::Clap;
use natrium::util::pretty_print_error;
use r0vm::s0::io::WriteBinary;
use r0vm::{s0::S0, vm};
use std::path::PathBuf;

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
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut vm =
        vm::R0Vm::new(s0, &mut stdin, &mut stdout).expect("Failed to create virtual machine");
    match vm.run_to_end() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Runtime error: {}", e);
            eprintln!("{}", vm.debug_stack());
            std::process::exit(1);
        }
    }
}

fn debug_run(s0: &S0) {
    todo!("Debugging is not yet implemented");
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let vm = vm::R0Vm::new(s0, &mut stdin, &mut stdout).expect("Failed to create virtual machine");
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
