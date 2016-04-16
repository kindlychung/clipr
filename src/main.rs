extern crate rustc_serialize;
extern crate docopt;
use std::process::{Stdio, Command};
use std::io::Write;


use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn get_clip_cmds() -> Result<(Command, Command), &'static str> {
    if cfg!(target_os = "linux") {
        match Command::new("xsel").spawn() {
            Ok(_) => {}
            _ => {
                writeln!(&mut std::io::stderr(),
                         "☹ xsel not found. You can install by (on Debian/Ubuntu):\n  sudo \
                          apt-get install xsel");
                std::process::exit(1);
            }
        };
        let mut cmd1 = Command::new("sh");
        let mut cmd2 = Command::new("sh");
        cmd1.arg("-c");
        cmd2.arg("-c");
        cmd1.arg("xsel -bi");
        cmd2.arg("xsel -bo");
        return Ok((cmd1, cmd2));
    }
    if cfg!(target_os = "macos") {
        return Ok((Command::new("pbcopy"), Command::new("pbpaste")));
    }
    return Err("Unsupported OS.");
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
