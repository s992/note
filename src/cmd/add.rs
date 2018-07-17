use std::fs::{File, create_dir_all};
use std::io::{Result, prelude::*};
use std::path::PathBuf;
use std::error::Error;
use rusqlite::Connection;
use time;
use lib;
use db;

pub fn run(conn: &Connection, book: String, contents: Option<String>) -> () {
    let notebook = match create_book_if_needed(&conn, &book) {
        Err(e) => panic!("Couldn't create book. Error: {}", e.description()),
        Ok(notebook) => notebook
    };

    let idx = db::get_next_note_id(&conn, &book);
    let note = get_note(idx, notebook, contents);

    db::create_note(&conn, note);
}

fn create_book_if_needed(conn: &Connection, book: &String) -> Result<db::Notebook> {
    let notebook = match db::get_notebook(&conn, &book) {
        Some(notebook) => notebook,
        None => {
            let notebook = db::Notebook { id: 0, name: book.clone(), time_created: time::get_time(), note_count: 0 };
            db::create_notebook(&conn, notebook);
            db::get_notebook(&conn, &book).unwrap()
        }
    };

    Ok(notebook)
}

fn get_note(index: i32, notebook: db::Notebook, contents: Option<String>) -> db::Note {
    let txt = match contents {
        Some(t) => t,
        None => lib::editor()
    };

    db::Note {
        id: 0,
        notebook_id: notebook.id,
        note_id: index,
        contents: txt,
        time_created: time::get_time(),
        time_modified: time::get_time(),
    }
}

fn save_note(mut path: PathBuf, index: usize, note: Option<String>) -> Result<()> {
    path.push(index.to_string());
    path.set_extension("txt");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldnt create note: {}", why.description()),
        Ok(f) => f,
    };

    match note {
        Some(n) => file.write_all(n.as_bytes())?,
        None => {
            let content = lib::editor();
            file.write_all(content.as_bytes())?
        }
    };

    Ok(())
}

