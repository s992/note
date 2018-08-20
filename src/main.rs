#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate text_io;
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
    note ls [--all]
    note ls <book>
    note lsa
    note cat <book> <note-index>
    note rm <book>
    note rm <book> <note-index>
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_add: bool,
    cmd_ls: bool,
    cmd_edit: bool,
    cmd_cat: bool,
    cmd_rm: bool,
    cmd_lsa: bool,
    arg_book: Option<String>,
    arg_note: Option<String>,
    arg_note_index: Option<usize>,
    flag_all: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    match args {
        Args { cmd_add: true, .. } => cmd::add::run(args.arg_book.unwrap(), args.arg_note),
        Args { cmd_ls: true, .. } => cmd::ls::run(args.arg_book, args.flag_all),
        Args { cmd_edit: true, .. } => cmd::edit::run(args.arg_book.unwrap(), args.arg_note_index.unwrap(), args.arg_note),
        Args { cmd_cat: true, .. } => cmd::cat::run(args.arg_book.unwrap(), args.arg_note_index.unwrap()),
        Args { cmd_rm: true, .. } => cmd::rm::run(args.arg_book.unwrap(), args.arg_note_index),
        Args { cmd_lsa: true, .. } => cmd::ls::run(None, true),
        _ => process::exit(1)
    }
}

