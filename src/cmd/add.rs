extern crate util;

use std::fs::{File, create_dir_all};
use std::io::{Result, prelude::*};
use std::path::PathBuf;
use std::error::Error;
use std::process;
use util::{get_next_index, get_book_path, editor};

pub fn run(book: String, note: Option<String>) -> () {
    let path = match create_book_if_needed(&book) {
        Err(e) => {
            println!("Couldn't create book. Error: {}", e.description());
            process::exit(1)
        },
        Ok(path) => path
    };

    let idx = get_next_index(book).unwrap();

    match save_note(path, idx, note) {
        Err(e) => {
            println!("Couldn't save note. Error: {}", e.description());
            process::exit(1)
        },
        Ok(()) => {}
    }
}

fn create_book_if_needed(book: &String) -> Result<PathBuf> {
    let path = get_book_path(&book.to_string())?;
    create_dir_all(&path)?;

    Ok(path)
}

fn save_note(mut path: PathBuf, index: usize, note: Option<String>) -> Result<()> {
    path.push(index.to_string());
    path.set_extension("txt");

    let mut file = match File::create(&path) {
        Err(e) => {
            println!("Couldn't create note: {}", e.description());
            process::exit(1)
        },
        Ok(f) => f,
    };

    match note {
        Some(n) => file.write_all(n.as_bytes())?,
        None => {
            let content = editor();
            file.write_all(content.as_bytes())?
        }
    };

    Ok(())
}

