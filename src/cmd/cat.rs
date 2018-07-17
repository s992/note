use rusqlite::Connection;
use lib;
use db;

pub fn run(conn: &Connection, book: String, index: i32) -> () {
    match db::get_note(&conn, book, index) {
        None => panic!("Unable to read note."),
        Some(note) => println!("{}", note.contents)
    }
}

