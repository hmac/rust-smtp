extern crate rusqlite;

use rusqlite::{SqliteConnection, SqliteError};
use std::path::Path;
use request;

pub fn open() -> SqliteConnection {
    let path = Path::new("mailbox.sqlite");
    let conn = SqliteConnection::open(path).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS email_addresses (id integer primary key, local text, domain text)",
         &[]
     ).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS inbound_messages ( \
        id integer primary key, \
        recipient text,
        sender text,
        body text)",
         &[]
     ).unwrap();
    conn
}

pub fn save(conn: &SqliteConnection, message: &request::Email) -> Result<i32, SqliteError> {
    conn.execute("INSERT INTO inbound_messages (recipient, sender, body) VALUES ($1, $2, $3)",
                 &[&message.to.unwrap().to_string(), &message.from.unwrap().to_string(), &message.body])
}
