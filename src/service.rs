use std::fs::{create_dir_all, OpenOptions};
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};
use std::path::Path;

use serde_derive::{Deserialize, Serialize};

use bincode::{deserialize, serialize};
use prettytable::{cell, format, row, Cell, Row, Table};

const DIR_NAME: &str = "todos";

const PENDING_FILE: &str = "pending.todo";
const COMPLETED_FILE: &str = "completed.todo";
const COUNTER_FILE: &str = "counter.todo";

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: u64,
    name: String,
}

impl PartialEq for Todo {
    fn eq(&self, other: &Todo) -> bool {
        self.id == other.id
    }
}

pub fn add(name: String) -> Result<()> {
    let mut todos = read_file(PENDING_FILE)?;
    todos.push(Todo {
        id: read_counter()?,
        name,
    });

    write_file(PENDING_FILE, todos)
}

pub fn done(id: u64) -> Result<()> {
    let mut pending_todos = read_file(PENDING_FILE)?;

    match pending_todos.iter().position(|todo| todo.id == id) {
        None => Err(Error::new(
            ErrorKind::InvalidInput,
            "No todo found with given ID",
        )),
        Some(index) => {
            let todo = pending_todos.remove(index);

            write_file(PENDING_FILE, pending_todos)?;

            let mut completed_todos = read_file(COMPLETED_FILE)?;
            completed_todos.push(todo);

            write_file(COMPLETED_FILE, completed_todos)
        }
    }
}

pub fn list_pending() -> Result<()> {
    print(read_file(PENDING_FILE)?)
}

pub fn list_completed() -> Result<()> {
    print(read_file(COMPLETED_FILE)?)
}

pub fn list_all() -> Result<()> {
    let mut todos = read_file(PENDING_FILE)?;
    todos.append(&mut read_file(COMPLETED_FILE)?);

    print(todos)
}

fn read_file(filename: &str) -> Result<Vec<Todo>> {
    create_dir_all(DIR_NAME)?;

    let path = Path::new(DIR_NAME).join(filename);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let filesize = file.metadata()?.len() as usize;

    if filesize > 0 {
        let mut serialized_todo: Vec<u8> = Vec::with_capacity(filesize);

        file.take(filesize as u64)
            .read_to_end(&mut serialized_todo)?;

        let todos: Vec<Todo> = match deserialize(&serialized_todo) {
            Ok(todos) => Ok(todos),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        }?;

        Ok(todos)
    } else {
        Ok(Vec::new())
    }
}

fn write_file(filename: &str, todos: Vec<Todo>) -> Result<()> {
    let serialized_todos = match serialize(&todos) {
        Ok(serialized_todos) => Ok(serialized_todos),
        Err(e) => Err(Error::new(ErrorKind::InvalidInput, e)),
    }?;

    let path = Path::new(DIR_NAME).join(filename);
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;

    file.write_all(&serialized_todos)
}

fn read_counter() -> Result<u64> {
    create_dir_all(DIR_NAME)?;

    let path = Path::new(DIR_NAME).join(COUNTER_FILE);
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let filesize = file.metadata()?.len();

    if filesize > 0 {
        let mut serialized_counter: [u8; 8] = [0; 8];
        file.read_exact(&mut serialized_counter)?;

        let counter: u64 = match deserialize(&serialized_counter) {
            Ok(counter) => Ok(counter),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        }?;

        let new_counter = counter + 1;
        let serialized_new_counter = match serialize(&new_counter) {
            Ok(serialized_new_counter) => Ok(serialized_new_counter),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        }?;

        file.seek(SeekFrom::Start(0))?;
        file.write_all(&serialized_new_counter)?;

        Ok(new_counter)
    } else {
        let counter: u64 = 1;

        file.write_all(&serialize(&counter).unwrap())?;
        Ok(counter)
    }
}

fn print(todos: Vec<Todo>) -> Result<()> {
    if todos.len() > 0 {
        let mut table = Table::new();
        table.set_titles(row!["ID", "Todo"]);

        for todo in todos {
            let mut cells = Vec::new();

            cells.push(Cell::new(&todo.id.to_string()));
            cells.push(Cell::new(&todo.name));

            let row = Row::new(cells);
            table.add_row(row);
        }

        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.printstd();
    }

    Ok(())
}
