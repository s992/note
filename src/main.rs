#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate termion;

use std::process;
use docopt::Docopt;

mod cmd;
mod lib;

const USAGE: &'static str = "\
Note

Usage:
    note add <book>
    note add <book> -c <note>
    note edit <book> <note-index>
    note edit <book> <note-index> -c <note>
    note ls
    note ls <book>
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_add: bool,
    cmd_ls: bool,
    cmd_edit: bool,
    arg_book: Option<String>,
    arg_note: Option<String>,
    arg_note_index: usize,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    match args {
        Args { cmd_add: true, .. } => cmd::add::run(args.arg_book.unwrap(), args.arg_note),
        Args { cmd_ls: true, .. } => cmd::ls::run(args.arg_book),
        Args { cmd_edit: true, .. } => cmd::edit::run(args.arg_book.unwrap(), args.arg_note_index, args.arg_note),
        _ => process::exit(1)
    }
}

