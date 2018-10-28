extern crate util;

use std::fs::{remove_file, remove_dir_all};
use rm::util::util::{get_book_path, get_note};

pub fn run(book: String, index: Option<usize>) -> () {
    match index {
        Some(_) => delete_note(book, index.unwrap()),
        None => delete_book(book)
    }
}

fn delete_book(book: String) -> () {
    let path = match get_book_path(&book) {
        Err(e) => panic!("Unable to find book: {}", e),
        Ok(path) => path
    };

    let confirmation = format!("Are you sure you want to delete the book {}? This will delete all of the notes it contains. [yN]", book);
    let deleting = confirm(confirmation, false);

    if !deleting {
        return;
    }

    remove_dir_all(path).unwrap();
}

fn delete_note(book: String, index: usize) -> () {
    let note = match get_note(&book, index) {
        Err(e) => panic!("Unable to find note: {}", e),
        Ok(note) => note
    };

    let confirmation = format!("Are you sure you want to delete note {} from {}? [yN]", index, book);
    let deleting = confirm(confirmation, false);

    if !deleting {
        return;
    }

    remove_file(note.path).unwrap();
}

fn confirm(text: String, default: bool) -> bool {
    println!("{}", text);
    let response: String = read!("{}\n");

    match response.as_str() {
        "y" => true,
        "Y" => true,
        "n" => false,
        "N" => false,
        _ => default,
    }
}
