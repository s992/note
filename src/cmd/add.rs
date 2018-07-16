use std::fs::{File, create_dir_all};
use std::io::{Result, prelude::*};
use std::path::PathBuf;
use std::error::Error;
use lib;

pub fn run(book: String, note: Option<String>) -> () {
    let path = match create_book_if_needed(&book) {
        Err(e) => panic!("Couldn't create book. Error: {}", e.description()),
        Ok(path) => path
    };

    let idx = lib::get_next_index(book).unwrap();

    match save_note(path, idx, note) {
        Err(e) => panic!("Couldn't save note. Error: {}", e.description()),
        Ok(()) => {}
    }
}

fn create_book_if_needed(book: &String) -> Result<PathBuf> {
    let path = lib::get_book_path(&book.to_string())?;
    create_dir_all(&path)?;

    Ok(path)
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

