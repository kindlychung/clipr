#![feature(plugin)]
#![plugin(docopt_macros)]
extern crate rustc_serialize;
extern crate clipboard;
extern crate docopt;

use std::io::{self, Read};
use clipboard::ClipboardContext;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

docopt!(Args derive Debug, "
clipr

Usage:
  clipr input
  clipr output
  clipr version
  clipr --help

Options:
  --help    Print this message.

Subcommands:
  input:        Put content from stdin into clipboard.
  output:       Put content from clipboard into stdin.
  version:      Print version.
");


fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());


    if args.cmd_input {
        let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
        let mut buffer = String::new();
        let _ = io::stdin().read_to_string(&mut buffer);
        let _ = ctx.set_contents(buffer);
        return;
    }

    if args.cmd_output {
        let ctx: ClipboardContext = ClipboardContext::new().unwrap();
        print!("{}", ctx.get_contents().unwrap());
    }

    if args.cmd_version {
        print!("{}", VERSION);
    }
}
