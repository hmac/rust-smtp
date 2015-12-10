use std::net::{TcpStream};
use std::io::{Read, Write, BufRead};
use bufstream::BufStream;
use store;
use parse_request;
use command::{Command, EmailAddress};

pub fn handle_request(stream: TcpStream) {
    let conn = store::open();
    let mut buf = BufStream::new(stream);
    let mut request_string = String::new();
    let mut email = Email {to: None, from: None, body: None};
    loop {
        buf.read_line(&mut request_string).unwrap();
        match parse_request::parse(&request_string) {
            Ok(Command::Helo(addr)) => {
                println!("Got HELO from {:?}", addr);
                buf.respond(ResponseCode::Hello);
            },
            Ok(Command::Mail(from)) => {
                println!("Got MAIL from {:?}", from);
                buf.respond(ResponseCode::Ok);
                email.from = Some(from);
            },
            Ok(Command::Rcpt(recipient)) => {
                println!("Got RCPT {:?}", recipient);
                buf.respond(ResponseCode::Ok);
                email.to = Some(recipient);
            },
            Ok(Command::Data) => {
                let mut body: Vec<String> = Vec::new();
                let mut body_string = String::new();
                buf.respond(ResponseCode::StartMailInput);
                buf.flush().unwrap();
                'data: loop {
                    buf.read_line(&mut body_string).unwrap();
                    println!("{:?}", body_string);
                    if body_string == ".\n" {
                        println!("Detected end of mail body");
                        break 'data;
                    }
                    body.push(body_string.clone());
                    buf.flush().unwrap();
                    body_string.clear();
                }
                email.body = Some(body.concat());
                println!("{:?}", email);
                match store::save(&conn, &email) {
                    Ok(_) => {
                        println!("Message saved for delivery");
                        buf.respond(ResponseCode::SavedForDelivery);
                    },
                    Err(err) => {
                        println!("Error saving message: {}", err);
                        buf.respond(ResponseCode::TransactionFailed);
                    }
                };
            },
            Ok(Command::Terminate) => {},
            Err(_) => {
                buf.respond(ResponseCode::CommandUnrecognised);
                println!("Command not recognised: {:?}", request_string);
            }
        }
        buf.flush().unwrap();
        request_string.clear();
    };
}


#[derive(Debug)]
pub struct Email {
    pub to: Option<EmailAddress>,
    pub from: Option<EmailAddress>,
    pub body: Option<String>
}

#[derive(Debug)]
enum ResponseCode {
    Ok,
    Hello,
    StartMailInput,
    CommandUnrecognised,
    ArgumentError,
    TransactionFailed,
    SavedForDelivery
}

impl ToString for ResponseCode {
    fn to_string(&self) -> String {
        match self {
            &ResponseCode::Ok => "250 Requested mail action completed.\n".to_string(),
            &ResponseCode::Hello => "250 rust-smtp at your service.\n".to_string(),
            &ResponseCode::StartMailInput => "354 End data with <CR><LF>.<CR><LF>\n".to_string(),
            &ResponseCode::CommandUnrecognised => "500 Syntax error, command unrecognised.\n".to_string(),
            &ResponseCode::ArgumentError => "501 Syntax error in command arguments\n".to_string(),
            &ResponseCode::TransactionFailed => "554 Transaction failed\n".to_string(),
            &ResponseCode::SavedForDelivery => "554 Saved for delivery\n".to_string()
        }
    }
}

trait WriteLine {
    fn write_line(&mut self, data: &str);
    fn respond(&mut self, code: ResponseCode);
}

impl<S: Read + Write> WriteLine for BufStream<S> {
    fn write_line(&mut self, data: &str) {
        self.write(data.as_bytes()).unwrap();
    }
    fn respond(&mut self, code: ResponseCode) {
        let response = match code {
            ResponseCode::Ok => "250 Requested mail action completed.",
            ResponseCode::Hello => "250 rust-smtp at your service.",
            ResponseCode::StartMailInput => "354 End data with <CR><LF>.<CR><LF>",
            ResponseCode::CommandUnrecognised => "500 Syntax error, command unrecognised.",
            ResponseCode::ArgumentError => "501 Syntax error in command arguments",
            ResponseCode::TransactionFailed => "554 Transaction failed",
            ResponseCode::SavedForDelivery => "554 Saved for delivery"
        };
        self.write(response.as_bytes())
            .and_then( |_| self.write("\n".as_bytes())).unwrap();
    }
}
