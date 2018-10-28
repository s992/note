extern crate util;
extern crate termion;

use std::process;
use termion::{color, style};
use util::{get_notes, get_books, Note, Book};

pub fn run(book: Option<String>, all: bool) -> () {
    match book {
        Some(book) => {
            let notes = get_notes(&book).unwrap_or_else(|e| {
                println!("Couldn't find book '{}': {}", book, e);
                process::exit(1)
            });
            print_notes(book, notes)
        }
        None => {
            let books = get_books().unwrap();

            if all {
                for book in books {
                    let notes = get_notes(&book.name).unwrap();
                    print_notes(book.name, notes);
                }
            } else {
                print_books(books);
            }
        }
    };
}

fn print_notes(book: String, mut notes: Vec<Note>) -> () {
    notes.sort_by_key(|n| n.index);

    println!("{blue}{count} note(s) for {b}{book}{reset}:",
             count = notes.len(),
             book = book,
             blue = color::Fg(color::Blue),
             b = style::Bold,
             reset = style::Reset,
    );

    for mut note in notes {
        let index = note.index;
        let mut line = note.first_line();

        if line.len() > 80 {
            line.truncate(77);
            line = format!("{line}{y}{bold}...{reset}",
                           line = line,
                           y = color::Fg(color::Yellow),
                           bold = style::Bold,
                           reset = style::Reset,
            );
        }

        println!("  {bold}{y}({reset_color}{idx}{y}){reset_color}{reset}  {note}",
                 y = color::Fg(color::Yellow),
                 bold = style::Bold,
                 reset_color = color::Fg(color::Reset),
                 reset = style::Reset,
                 idx = index,
                 note = line,
        )
    }

    println!();
}

fn print_books(books: Vec<Book>) -> () {
    println!("{blue}{b}{count} books:{reset}",
             count = books.len(),
             blue = color::Fg(color::Blue),
             b = style::Bold,
             reset = style::Reset,
    );
    println!();

    for mut book in books {
        let count = book.note_count();
        let name = book.name;

        println!("{book} ({y}{count}{reset})",
                 book = name,
                 count = count,
                 y = color::Fg(color::Yellow),
                 reset = style::Reset,
        )
    }
}
