#[macro_use]
extern crate nom;
extern crate bufstream;
extern crate rusqlite;
use std::net::{TcpListener};
use std::thread;

mod request;
mod store;
mod parse_request;
mod command;

fn main() {
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
