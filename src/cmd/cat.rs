use process;
use lib;

pub fn run(book: String, index: usize) -> () {
    match lib::get_note(&book, index) {
        Err(e) => {
            println!("Unable to read note {} from book '{}': {}", index, book, e);
            process::exit(1)
        },
        Ok(note) => println!("{}", note.contents)
    }
}

