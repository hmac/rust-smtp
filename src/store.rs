extern crate rusqlite;

use rusqlite::{SqliteConnection, SqliteError};
use std::path::Path;
use types::{InboundEmail, Email, EmailAddress};
use std::str::FromStr;

// Open a new Sqlite connection
pub fn open() -> SqliteConnection {
    let path = Path::new("mailbox.sqlite");
    SqliteConnection::open(path).unwrap()
}

// TODO: capture and return any errors encountered
// At the moment this will just panic on error
pub fn setup() {
    let path = Path::new("mailbox.sqlite");
    let conn = SqliteConnection::open(path).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS emails ( \
        id integer primary key, \
        recipient text, \
        sender text, \
        body text, \
        type text)",
        &[]
    ).unwrap();
    conn.close();
}

pub fn save_inbound_message(conn: &SqliteConnection, message: &Email) -> Result<i64, SqliteError> {
    // This must succeed or las_insert_rowid will be invalid
    try!(conn.execute(
        "INSERT INTO emails (recipient, sender, body) VALUES ($1, $2, $3)",
        &[&message.to.to_string(), &message.from.to_string(), &message.body]
    ));
    Ok(conn.last_insert_rowid())

}

pub fn set_local_message(conn: &SqliteConnection, id: i64) -> Result<i32, SqliteError> {
    conn.execute(
        "UPDATE emails SET type='local' WHERE ID = ?",
        &[&id]
    )
}

pub fn set_outbound_message(conn: &SqliteConnection, id: i64) -> Result<i32, SqliteError> {
    conn.execute(
        "UPDATE emails SET type='remote' WHERE ID = ?",
        &[&id]
    )
}

// TODO: use Result for this
pub fn new_inbound_messages(conn: &SqliteConnection) -> Vec<(i64, Email)> {
    let mut statement = conn.prepare(
        "SELECT id, recipient, sender, body from emails WHERE type IS NULL"
    ).unwrap();
    let rows = statement.query(&[]).unwrap();

    let mut messages = Vec::new();
    for maybe_row in rows {
        let row = maybe_row.unwrap();
        let id = row.get(0);
        let to = EmailAddress::from_str(&row.get::<String>(1));
        let from = EmailAddress::from_str(&row.get::<String>(2)).unwrap();
        let body = row.get(3);
        let email = Email { to: to.unwrap(), from: from, body: body };
        messages.push((id, email))
    };
    messages
}

// TODO: use Result for this
pub fn new_outbound_messages(conn: &SqliteConnection) -> Vec<(i64, Email)> {
    let mut statement = conn.prepare(
        "SELECT id, recipient, sender, body from emails WHERE type IS 'remote'"
    ).unwrap();
    let rows = statement.query(&[]).unwrap();

    let mut messages = Vec::new();
    for maybe_row in rows {
        let row = maybe_row.unwrap();
        let id = row.get(0);
        let to = EmailAddress::from_str(&row.get::<String>(1));
        let from = EmailAddress::from_str(&row.get::<String>(2)).unwrap();
        let body = row.get(3);
        let email = Email { to: to.unwrap(), from: from, body: body };
        messages.push((id, email))
    };
    messages
}
