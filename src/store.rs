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
        "INSERT INTO inbound_messages (recipient, sender, body) VALUES ($1, $2, $3)",
        &[&message.to.to_string(), &message.from.to_string(), &message.body]
    ));
    Ok(conn.last_insert_rowid())

}

pub fn save_local_message(conn: &SqliteConnection, inbound_email: &InboundEmail) -> Result<i32, SqliteError> {
    let message = &inbound_email.email;
    let transaction = conn.transaction();
    conn.execute(
        "UPDATE local_messages SET type='local' WHERE ID = ?",
        &[&inbound_email.id]
    )
    // get the ID of this new row, somehow
    // conn.execute("UPDATE inbound_messages SET local_message_id = new_id")
}

pub fn save_outbound_message(conn: &SqliteConnection, message: &InboundEmail) -> Result<i32, SqliteError> {
    conn.execute(
        "UPDATE local_messages SET type='remote' WHERE ID = ?",
        &[&message.id]
    )
}

pub fn save_sent_message(conn: &SqliteConnection, message: &Email) -> Result<i32, SqliteError> {
    conn.execute(
        "INSERT INTO sent_messages (recipient, sender, body) VALUES ($1, $2, $3)",
        &[&message.to.to_string(), &message.from.to_string(), &message.body]
    )
}

// TODO: use Result for this
pub fn new_inbound_messages(conn: &SqliteConnection) -> Vec<InboundEmail> {
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
        let email = InboundEmail {
            id: id,
            email: Email { to: to.unwrap(), from: from, body: body }
        };
        messages.push(email)
    };
    messages
}

// TODO: use Result for this
pub fn new_outbound_messages(conn: &SqliteConnection) -> Vec<InboundEmail> {
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
        let email = InboundEmail {
            id: id,
            email: Email { to: to.unwrap(), from: from, body: body }
        };
        messages.push(email)
    };
    messages
}
