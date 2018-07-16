use termion::{color, style};
use lib;

pub fn run(book: Option<String>) -> () {
    match book {
        Some(book) => {
            let notes = lib::get_notes(&book).unwrap();
            print_notes(book, notes)
        }
        None => {
            let books = lib::get_books().unwrap();
            print_books(books);
        }
    };
}

fn print_notes(book: String, mut notes: Vec<lib::Note>) -> () {
    notes.sort_by_key(|n| n.index);

    println!("{r}{b}{count} notes for {book}:{reset}",
             count = notes.len(),
             book = book,
             r = color::Fg(color::LightGreen),
             b = style::Bold,
             reset = style::Reset,
    );
    println!();

    for mut note in notes {
        let index = note.index;
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

fn print_books(books: Vec<String>) -> () {
    println!("{r}{b}{count} books:{reset}",
             count = books.len(),
             r = color::Fg(color::LightGreen),
             b = style::Bold,
             reset = style::Reset,
    );
    println!();

    for book in books {
        println!("{y}{book}{reset}",
                 book = book,
                 y = color::Fg(color::Yellow),
                 reset = style::Reset,
        )
    }
}
