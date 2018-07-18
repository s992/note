use termion::{color, style};
use lib;

pub fn run(book: Option<String>, all: bool) -> () {
    match book {
        Some(book) => {
            let notes = lib::get_notes(&book).unwrap();
            print_notes(book, notes)
        }
        None => {
            let books = lib::get_books().unwrap();

            if all {
                for book in books {
                    let notes = lib::get_notes(&book.name).unwrap();
                    print_notes(book.name, notes);
                }
            } else {
                print_books(books);
            }
        }
    };
}

fn print_notes(book: String, mut notes: Vec<lib::Note>) -> () {
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

fn print_books(books: Vec<lib::Book>) -> () {
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
