extern crate bufstream;
extern crate rusqlite;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

mod request;
mod store;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:33333").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    request::handle_request(stream)
                });
            }
            Err(e) => { println!("connection failed"); }
        }
    }

    drop(listener);
}
