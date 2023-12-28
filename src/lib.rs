use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use rusqlite::{Connection, params, Result};

mod constants;
use constants::{TEST_DATABASE, SQL_COMMANDS_PATH};
use crate::constants::RAW_DATA_PATH;

#[derive(Debug)]
pub struct Person {
    id: i32,
    name: String,
    age: i32,
}
impl Person {
    pub fn new(id: i32, name: String, age: i32) -> Self {
        Self {id, name, age}
    }
}
pub fn read_file_and_create_database() -> std::io::Result<()> {
    let file = File::open(RAW_DATA_PATH)?;
    let reader = BufReader::new(file);
    let mut data: Vec<Person> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let items: Vec<String> = line.split(",").map(|s| s.to_string()).collect();
        data.push(Person::new(0, items.get(0).unwrap().clone(), items.get(1).unwrap().clone().parse().unwrap_or_else(|_| {
            1
        })))
    }
    create_database(data).expect("Couldn't create table");
    Ok(())
}
fn read_sql_file_into_str() -> std::io::Result<String> {
    let mut file = File::open(SQL_COMMANDS_PATH)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    Ok(result)
}
fn create_database(data: Vec<Person>) -> Result<()> {
    let conn = Connection::open(TEST_DATABASE)?;
    if let Err(_) = conn.execute("DROP TABLE IF EXISTS people", []) {
        println!("Couldn't drop table");
    }
    if let Err(_) = conn.execute(
        "CREATE TABLE people (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                age INTEGER NOT NULL)", []) {
        println!("Couldn't create table");
    }
    for entry in data {
        if let Err(_) = conn.execute(
            "INSERT INTO people (name, age) VALUES (?1, ?2)",
            params![entry.name, entry.age]
        ) {
            println!("Couldn't insert entry");
        }
    }
    Ok(())
}
pub fn query_database() -> Result<()> {
    let connection = Connection::open(TEST_DATABASE)?;
    let mut search_statement = connection.prepare("SELECT id, name, age FROM people")?;
    let person_iter = search_statement.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;
    for person in person_iter {
        println!("Found: {:?}", person.unwrap());
    }
    Ok(())
}
