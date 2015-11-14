use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
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
    let mut request_string = String::new();
    stream.read_to_string(&mut request_string);
    let maybe_command = parse_request(&request_string);
    if maybe_command.is_some() {
        let c = maybe_command.unwrap();
        match c {
            Command::Echo(message) => {
                println!("Sending response: {:?}", message);
                stream.write(message.as_bytes());
            },
            Command::Helo(addr) => {
                println!("Got HELO, sending 250");
                stream.write("250 Hello ".as_bytes());
                stream.write(addr.as_bytes());
            },
            Command::Mail(from) => {
                println!("Got MAIL, sending 250");
                println!("From: {:?}", from);
                stream.write("250 Ok".as_bytes());
            },
            Command::Rcpt(recipient) => {
                println!("Got RCPT, sending 250");
                println!("Recipient: {:?}", recipient);
                stream.write("250 Ok".as_bytes());
            }
            _ => {}
        };
    }
    else {
        println!("Command not recognised: {:?}", request_string);
    };
}

fn parse_request(req: &str) -> Option<Command> {
    // The command should be a 4 character string like ECHO or SEND
    // It is followed by a space, then any data
    let command_str = &req[0..4];
    match command_str {
        "ECHO" => Some(Command::Echo(req[5..].to_string())),
        "HELO" => Some(Command::Helo(req[5..].to_string())),
        "RCPT" => Some(Command::Rcpt(req[5..].to_string())),
        "MAIL" => Some(Command::Mail(req[5..].to_string())),
        _      => None
    }
}

enum Command {
    Echo(String),
    Helo(String), // contains address of connecting server
    Mail(String), // contains from address
    Rcpt(String), // contains to address
    Data          // mail message to follow
}
