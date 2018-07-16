use std::fs::{File, read_dir, read_to_string};
use std::io::{Result, prelude::*};
use std::env::{home_dir, temp_dir, var_os};
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

#[derive(Debug)]
pub struct Note {
    pub path: PathBuf,
    pub index: usize,
    pub contents: String,
}

impl Note {
    pub fn first_line(&mut self) -> String {
        let split_lines = self.contents.split("\n").collect::<Vec<_>>();
        split_lines[0].to_string()
    }
}

#[derive(Debug)]
pub struct Book {
    pub path: PathBuf,
    pub name: String,
}

impl Book {
    pub fn note_count(&mut self) -> usize {
        count_notes(&self.path).unwrap()
    }
}

pub fn get_base_path() -> Result<PathBuf> {
    let home = home_dir().unwrap();
    let mut path = PathBuf::new();

    path.push(home);
    path.push(".notes");

    Ok(path)
}

pub fn get_book_path(book: &String) -> Result<PathBuf> {
    let mut path = get_base_path()?;

    path.push(&book);

    Ok(path)
}

pub fn get_books() -> Result<Vec<Book>> {
    let path = get_base_path()?;
    let paths = read_dir(path)?;
    let mut books = Vec::new();

    for book in paths {
        let entry = book.unwrap();
        let name = entry.file_name().into_string().unwrap();
        let path = entry.path();

        books.push(Book { name, path });
    }

    Ok(books)
}

pub fn get_notes(book: &String) -> Result<Vec<Note>> {
    let path = get_book_path(&book)?;
    let paths = read_dir(path)?;
    let mut notes = Vec::new();

    for note in paths {
        let entry = note.unwrap();
        let path = entry.path();
        let idx = get_note_index(&path)?;
        let txt = read_note(&path)?;

        notes.push(Note { path, index: idx, contents: txt })
    }

    Ok(notes)
}

pub fn get_next_index(book: String) -> Result<usize> {
    let path = get_book_path(&book)?;
    let paths = read_dir(path)?;
    let mut index = 0;

    for note in paths {
        let entry = note.unwrap();
        let path = entry.path();
        let idx = get_note_index(&path)?;

        if idx > index {
            index = idx;
        }
    }

    Ok(index + 1)
}

pub fn editor() -> String {
    let tmppath = temp_dir().join("asdf");

    match File::create(&tmppath) {
        Ok(f) => f,
        Err(e) => panic!("File error: {}", e)
    };

    match open_editor(&tmppath) {
        Ok(_) => read_to_string(tmppath).expect("Unable to read temporary file."),
        Err(e) => panic!("Failed to open $EDITOR: {}", e)
    }
}

pub fn open_editor(path: &PathBuf) -> Result<ExitStatus> {
    let editor = match var_os("EDITOR") {
        Some(val) => val.into_string().unwrap(),
        None => String::from("vi"),
    };

    Command::new(editor)
        .arg(path.display().to_string())
        .spawn()
        .expect("Failed to open $EDITOR")
        .wait()
}

fn count_notes(path: &PathBuf) -> Result<usize> {
    let files = read_dir(path)?;

    Ok(files.count())
}

fn get_note_index(path: &PathBuf) -> Result<usize> {
    let index = path.file_stem().unwrap().to_str().unwrap();
    let idx = index.parse::<usize>().unwrap();

    Ok(idx)
}

fn read_note(path: &PathBuf) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

