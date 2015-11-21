use std::net::{TcpStream};
use std::io::{Read, Write};

pub fn handle_request(mut stream: TcpStream) {
    let mut request_string = String::new();
    stream.read_to_string(&mut request_string);
    let maybe_command = parse_request(&request_string);
    if maybe_command.is_some() {
        let c = maybe_command.unwrap();
        match c {
            Command::Helo(addr) => {
                println!("Got HELO from {:?}, sending 250", addr);
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
    // The command should be a 4 character string like HELO or MAIL
    // It is followed by a space, then any data
    let command_str = &req[0..5];
    match command_str {
        "HELO " => Some(Command::Helo(req[5..].to_string())),
        "MAIL " => {
            let addr = parse_from_address(&req[5..]);
            match addr {
                Some(address) => Some(Command::Mail(address)),
                None          => None
            }
        },
        "RCPT " => {
            let addr = parse_to_address(&req[5..]);
            match addr {
                Some(address) => Some(Command::Rcpt(address)),
                None          => None
            }
        },
        "DATA " => Some(Command::Data),
        _      => None
    }
}

fn parse_from_address(req: &str) -> Option<EmailAddress> {
    // Expects "FROM:<address@domain.com>"
    let prefix = &req[0..5];
    match prefix {
        "FROM:" => Some(req[5..].to_string()),
        _       => None
    }
}

fn parse_to_address(req: &str) -> Option<EmailAddress> {
    // Expects "TO:<address@domain.com>"
    let prefix = &req[0..3];
    match prefix {
        "TO:" => Some(req[3..].to_string()),
        _     => None
    }
}

fn parse_smtp_address(req: &str) -> Option<ServerAddress> {
    // Expects "address@domain.com"
    Some(req.to_string())
}

type EmailAddress = String;
type ServerAddress  = String;

enum Command {
    Helo(ServerAddress), // contains address of connecting server
    Mail(EmailAddress), // contains from address
    Rcpt(EmailAddress), // contains to address
    Data          // mail message to follow
}
