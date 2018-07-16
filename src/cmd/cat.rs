use lib;

pub fn run(book: String, index: usize) -> () {
    match lib::get_note(&book, index) {
        Err(e) => panic!("Unable to read note: {}", e),
        Ok(note) => println!("{}", note.contents)
    }
}

