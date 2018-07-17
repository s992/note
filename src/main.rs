#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate text_io;
extern crate docopt;
extern crate termion;
extern crate rusqlite;
extern crate time;

use std::process;
use docopt::Docopt;
use rusqlite::Connection;

mod cmd;
mod lib;
mod db;

const USAGE: &'static str = "\
Note

Usage:
    note add <book>
    note add <book> -c <note>
    note edit <book> <note-index>
    note edit <book> <note-index> -c <note>
    note ls
    note ls <book>
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
    arg_book: Option<String>,
    arg_note: Option<String>,
    arg_note_index: Option<usize>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let db_path = db::get_db_path().unwrap();
    let conn = Connection::open(db_path).unwrap();

    db::init_db(&conn).unwrap();

    match args {
        Args { cmd_add: true, .. } => cmd::add::run(args.arg_book.unwrap(), args.arg_note),
        Args { cmd_ls: true, .. } => cmd::ls::run(&conn, args.arg_book),
        Args { cmd_edit: true, .. } => cmd::edit::run(args.arg_book.unwrap(), args.arg_note_index.unwrap(), args.arg_note),
        Args { cmd_cat: true, .. } => cmd::cat::run(args.arg_book.unwrap(), args.arg_note_index.unwrap()),
        Args { cmd_rm: true, .. } => cmd::rm::run(args.arg_book.unwrap(), args.arg_note_index),
        _ => process::exit(1)
    }
}

