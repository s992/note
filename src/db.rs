use std::io::Result;
use std::path::PathBuf;
use rusqlite::Connection;
use time::{Timespec, get_time};
use lib;

#[derive(Debug)]
pub struct Notebook {
    pub id: i32,
    pub name: String,
    pub time_created: Timespec,
    pub note_count: i32,
}

#[derive(Debug)]
pub struct Note {
    pub id: i32,
    pub note_id: i32,
    pub notebook_id: i32,
    pub contents: String,
    pub time_created: Timespec,
    pub time_modified: Timespec,
}

impl Note {
    pub fn first_line(&mut self) -> String {
        let split_lines = self.contents.split("\n").collect::<Vec<_>>();
        split_lines[0].to_string()
    }
}

pub fn init_db(conn: &Connection) -> Result<()> {
    create_tables(&conn)?;
    create_indices(&conn)?;

    Ok(())
}

pub fn get_db_path() -> Result<PathBuf> {
    let mut path = lib::get_base_path()?;

    path.push("note");
    path.set_extension("db");

    Ok(path)
}

pub fn get_notebook(conn: &Connection, name: String) -> Option<Notebook> {
    let notebook = conn.query_row("
        select nb.id, nb.name, nb.time_created, count(n.id) as note_count
        from notebook nb
        inner join note n on n.notebook_id = nb.id
        where name = ?1
        ", &[&name], |row| {
        Notebook { id: row.get(0), name: row.get(1), time_created: row.get(2), note_count: row.get(3) }
    });

    match notebook {
        Err(e) => None,
        Ok(notebook) => Some(notebook)
    }
}

pub fn get_notebooks(conn: &Connection) -> Vec<Notebook> {
    let count = count_notebooks(&conn);

    if count == 0 {
        return Vec::new();
    }

    let mut stmt = conn.prepare("
        select nb.id, nb.name, nb.time_created, count(n.id) as note_count
        from notebook nb
        inner join note n on n.notebook_id = nb.id
        where nb.id is not null
        ").unwrap();

    let rows = stmt.query_map(&[], |row| {
        Notebook {
            id: row.get(0),
            name: row.get(1),
            time_created: row.get(2),
            note_count: row.get(3),
        }
    }).unwrap();

    let mut notebooks = Vec::new();

    for row in rows {
        let notebook = match row {
            Err(e) => panic!("{}", e),
            Ok(notebook) => notebook
        };

        notebooks.push(notebook);
    }

    notebooks
}

pub fn create_notebook(conn: &Connection, notebook: Notebook) -> i64 {
    let insert = conn.execute("
        insert into notebook (name, time_created)
        values (?1, ?2)
    ", &[&notebook.name, &notebook.time_created]);

    let id = match insert {
        Err(e) => panic!("Failed to create notebook. Possibly trying to reuse a name? {}", e),
        Ok(_) => conn.last_insert_rowid(),
    };

    id
}

pub fn get_note(conn: &Connection, book_name: String, id: i32) -> Option<Note> {
    let note = conn.query_row("
        select n.id, n.note_id, n.notebook_id, n.contents, n.time_created, n.time_modified
        from note n
        inner join notebook nb on nb.id = note.notebook_id
        where n.id = ?1
        and nb.name = ?2
        ", &[&id, &book_name], |row| {
        Note {
            id: row.get(0),
            note_id: row.get(1),
            notebook_id: row.get(2),
            contents: row.get(3),
            time_created: row.get(4),
            time_modified: row.get(5),
        }
    });

    match note {
        Err(e) => None,
        Ok(note) => Some(note)
    }
}

pub fn get_notes(conn: &Connection, book_name: &String) -> Vec<Note> {
    let mut stmt = conn.prepare("
        select n.id, n.note_id, n.notebook_id, n.contents, n.time_created, n.time_modified
        from note n
        inner join notebook nb on nb.id = n.notebook_id
        where nb.name = ?1
        ").unwrap();

    let rows = stmt.query_map(&[book_name], |row| {
        Note {
            id: row.get(0),
            note_id: row.get(1),
            notebook_id: row.get(2),
            contents: row.get(3),
            time_created: row.get(4),
            time_modified: row.get(5),
        }
    }).unwrap();

    let mut notes = Vec::new();

    for row in rows {
        let note = match row {
            Err(e) => panic!("{}", e),
            Ok(note) => note
        };

        notes.push(note);
    }

    notes
}

pub fn create_note(conn: &Connection, note: Note) -> i64 {
    let insert = conn.execute("
        insert into note (note_id, notebook_id, contents, time_created, time_modified))
        values (?1, ?2, ?3, ?4, ?5)
    ", &[
        &note.note_id,
        &note.notebook_id,
        &note.contents,
        &note.time_created,
        &note.time_modified,
    ]);

    let id = match insert {
        Err(e) => panic!("Failed to create note. {}", e),
        Ok(_) => conn.last_insert_rowid(),
    };

    id
}

fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute("
        create table if not exists notebook (
            id integer primary key,
            name text not null,
            time_created text not null
        )
    ", &[]).unwrap();

    conn.execute("\
        create table if not exists note (
            id integer primary key,
            note_id integer not null,
            notebook_id integer not null,
            contents text not null,
            time_created text not null,
            time_modified text not null
        )
    ", &[]).unwrap();

    Ok(())
}

fn create_indices(conn: &Connection) -> Result<()> {
    conn.execute("
        create unique index if not exists notebook_name_idx ON notebook (name)
    ", &[]).unwrap();

    Ok(())
}

fn count_notebooks(conn: &Connection) -> i32 {
    let result = conn.query_row("
        select count(id)
        from notebook
        ", &[], |row| {
        row.get(0)
    });

    match result {
        Err(e) => 0,
        Ok(count) => count
    }
}

