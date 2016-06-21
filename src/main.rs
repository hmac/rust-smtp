#[macro_use]
extern crate nom;
extern crate bufstream;
extern crate rusqlite;
use std::net::{TcpListener};
use std::thread;
use std::time::{Duration};

mod request;
mod store;
mod parser;
mod types;

fn main() {
    // Setup the database
    store::setup();

    let server = thread::spawn(move|| { start_server() });

    let sorter = thread::spawn(move|| { start_sorter() });

    let client = thread::spawn(move|| { start_client() });

    // Let the server live on
    server.join().unwrap();
}

fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:33333").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    request::handle_request(stream)
                });
            }
            Err(_) => { println!("connection failed"); }
        }
    }

    drop(listener);
}

fn start_sorter() {
    let conn = store::open();
    loop {
        let new_messages = store::new_inbound_messages(&conn);
        for (id, email) in new_messages {
            println!("NEW INBOUND MESSAGE: {} {:?}", id, email);
            // Determine if the recipient is local
            // For now we'll assume anything addressed to harry@local is local
            if email.to.local == "harry" && email.to.domain == "local" {
                store::set_local_message(&conn, id).unwrap();
            }
            else {
                store::set_outbound_message(&conn, id).unwrap();
            }
        }

        thread::sleep(Duration::new(5, 0));
    }
}

fn start_client() {
    let conn = store::open();
    loop {
        let messages_to_send = store::new_outbound_messages(&conn);
        for (id, email) in messages_to_send {
            println!("Found remote email: {:?}", email);
            // Send email to server
            // ...
        }

        thread::sleep(Duration::new(5, 0));
    }
}
