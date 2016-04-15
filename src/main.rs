extern crate rustc_serialize;
extern crate docopt;
extern crate os_type;
use std::process::{Stdio, Command};


use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn get_clip_cmds() -> Result<(Command, Command), &'static str> {
    match os_type::current_platform() {
        os_type::OSType::OSX => Ok((Command::new("pbcopy"), Command::new("pbpaste"))),
        os_type::OSType::Ubuntu |
        os_type::OSType::Arch |
        os_type::OSType::Debian |
        os_type::OSType::Redhat => {
            let mut cmd1 = Command::new("xsel");
            let mut cmd2 = Command::new("xsel");
            cmd1.arg("-bi");
            cmd2.arg("-bo");
            return Ok((cmd1, cmd2));
        }
        _ => Err("Unknown OS."),
    }
}

const USAGE: &'static str = "
clipr

Usage:
  clipr input
  clipr output
  clipr version

Options:
  --help    Print this message.

Subcommands:
  input:        Put content from stdin into clipboard.
  output:       Put content from clipboard into stdin. 
  version:      Print version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_input: bool,
    cmd_output: bool,
    cmd_version: bool,
}

fn run(cmd: &mut Command) {
    assert!(cmd.stdout(Stdio::inherit())
               .stderr(Stdio::inherit())
               .status()
               .unwrap()
               .success());
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                         .and_then(|d| d.decode())
                         .unwrap_or_else(|e| e.exit());


    let (mut cmd_in, mut cmd_out) = get_clip_cmds().unwrap();
    if args.cmd_input {
        run(&mut cmd_in);
    }

    if args.cmd_output {
        run(&mut cmd_out);

    }

    if args.cmd_version {
        println!("{}", VERSION);
    }
}
