use termion::{color, style};
use rusqlite::Connection;
use lib;
use db;

pub fn run(conn: &Connection, book: Option<String>) -> () {
    match book {
        Some(book) => {
            let notes = db::get_notes(&conn, &book);
            print_notes(book, notes)
        }
        None => {
            let books = db::get_notebooks(&conn);
            print_books(books);
        }
    };
}

fn print_notes(book: String, mut notes: Vec<db::Note>) -> () {
    notes.sort_by_key(|n| n.note_id);

    println!("{blue}{b}{count} notes for {book}:{reset}",
             count = notes.len(),
             book = book,
             blue = color::Fg(color::Blue),
             b = style::Bold,
             reset = style::Reset,
    );
    println!();

    for mut note in notes {
        let index = note.note_id;
        let mut line = note.first_line();

        if line.len() > 80 {
            line.truncate(77);
            line = format!("{line}{y}{bold}...",
                           line = line,
                           y = color::Fg(color::Yellow),
                           bold = style::Bold,
            );
        }

        println!("{bold}{y}({reset_color}{idx}{y}){reset_color}{reset}  {note}",
                 y = color::Fg(color::Yellow),
                 bold = style::Bold,
                 reset_color = color::Fg(color::Reset),
                 reset = style::Reset,
                 idx = index,
                 note = line,
        )
    }
}

fn print_books(books: Vec<db::Notebook>) -> () {
    println!("{blue}{b}{count} books:{reset}",
             count = books.len(),
             blue = color::Fg(color::Blue),
             b = style::Bold,
             reset = style::Reset,
    );
    println!();

    for mut book in books {
        let count = book.note_count;
        let name = book.name;

        println!("{book} ({y}{count}{reset})",
                 book = name,
                 count = count,
                 y = color::Fg(color::Yellow),
                 reset = style::Reset,
        )
    }
}
