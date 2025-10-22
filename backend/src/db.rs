use std::{fs, path::Path};

use rusqlite::Connection;



pub fn initialise_db() -> Result<Connection,rusqlite::Error> {
    let db_dir = "db";
    let db_file = "db/main.db";

    if !Path::new(db_dir).exists() {
        fs::create_dir_all(db_dir).expect("Failed to create db directory");
    }


    let conn = Connection::open(db_file)?;


    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            hash TEXT NOT NULL UNIQUE
        )
        ",
        [],
    )?;

    Ok(conn)
}