use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:33333").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_request(stream)
                });
            }
            Err(e) => { println!("connection failed"); }
        }
    }

    drop(listener);
}

fn handle_request(mut stream: TcpStream) {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer);
    println!("{:?}", buffer);
}
