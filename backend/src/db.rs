use std::{fs, path::Path};

use rusqlite::{Connection, params};

use crate::pw;

pub fn initialise_db() -> Result<Connection, rusqlite::Error> {
    let db_dir = "db";
    let db_file = "db/main.db";

    if !Path::new(db_dir).exists() {
        fs::create_dir_all(db_dir).expect("Failed to create db directory");
    }

    let conn = Connection::open(db_file)?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            hash TEXT NOT NULL
        )
        ",
        [],
    )?;

    Ok(conn)
}

pub enum STATE {
    USERALREADYEXIST,
    USERDOESNOTEXIST,
    PASSWORDMISMATCH,
    OK,
}
impl ToString for STATE {
    fn to_string(&self) -> String {
        match self {
            STATE::OK => String::from("Ok"),
            STATE::PASSWORDMISMATCH => String::from("Password mismatch"),
            STATE::USERDOESNOTEXIST => String::from("User does not exist"),
            STATE::USERALREADYEXIST => String::from("User already exist"),
        }
    }
}

pub fn verify_safe(
    conn: &Connection,
    username: &str,
    passw: &str,
) -> Result<STATE, rusqlite::Error> {
    if !user_exist(conn, username)? {
        return Ok(STATE::USERDOESNOTEXIST);
    }

    Ok(if verify_user(conn, username, passw)? {
        STATE::OK
    } else {
        STATE::PASSWORDMISMATCH
    })
}

pub fn add_safe(conn: &Connection, username: &str, passw: &str) -> Result<STATE, rusqlite::Error> {
    if user_exist(conn, username)? {
        return Ok(STATE::USERALREADYEXIST);
    }

    add_user(conn, username, passw)?;

    Ok(STATE::OK)
}

fn add_user(conn: &Connection, username: &str, passw: &str) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare(
        "
        INSERT INTO users (username, hash) VALUES (?,?)
    ",
    )?;

    let hash = pw::hash_password(passw);

    stmt.execute(params![username, hash])?;
    Ok(())
}

fn user_exist(conn: &Connection, username: &str) -> Result<bool, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "
        SELECT * FROM users WHERE username = ? LIMIT 1
    ",
    )?;

    let exists = stmt.exists(params![username])?;

    Ok(exists)
}

fn verify_user(conn: &Connection, username: &str, passw: &str) -> Result<bool, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "
        SELECT hash FROM users WHERE username = ? LIMIT 1
    ",
    )?;

    let stored_hash: String = stmt.query_row(params![username], |row| row.get(0))?;

    Ok(pw::verify_password(passw, &stored_hash))
}
