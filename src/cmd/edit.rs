extern crate util;

use std::fs::{OpenOptions};
use std::io::{Result, prelude::*};
use std::path::PathBuf;
use std::process;
use util::{get_book_path, open_editor};

pub fn run(book: String, index: usize, note: Option<String>) -> () {
    let mut path = match get_book_path(&book) {
        Err(e) => {
            println!("Couldn't load book '{}'. Error: {}", book, e);
            process::exit(1)
        },
        Ok(path) => path
    };

    path.push(index.to_string());
    path.set_extension("txt");

    match note {
        Some(note) => {
            update_note(path, note).unwrap();
        },
        None => {
            edit_note(path).unwrap();
        }
    }
}

fn update_note(path: PathBuf, note: String) -> Result<()> {
    let mut file = OpenOptions::new().write(true).open(path)?;
    file.set_len(0)?;
    file.write_all(note.as_bytes())
}

fn edit_note(path: PathBuf) -> Result<()> {
    open_editor(&path)?;
    Ok(())
}

