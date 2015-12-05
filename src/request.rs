use std::net::{TcpStream};
use std::io::{Read, Write, BufReader, BufRead, BufWriter};
use bufstream::BufStream;
use store;

pub fn handle_request(mut stream: TcpStream) {
    let conn = store::open();
    let mut buf = BufStream::new(stream);
    let mut request_string = String::new();
    let mut email = Email {to: None, from: None, body: None};
    loop {
        buf.read_line(&mut request_string);
        match parse_request(&request_string) {
            Ok(Command::Helo(addr)) => {
                println!("Got HELO from {:?}", addr);
                buf.respond(ResponseCode::Hello);
            },
            Ok(Command::Mail(from)) => {
                println!("Got MAIL from {:?}", from);
                buf.write(ResponseCode::Ok.to_string().as_bytes());
                email.from = Some(from);
            },
            Ok(Command::Rcpt(recipient)) => {
                println!("Got RCPT {:?}", recipient);
                buf.write(ResponseCode::Ok.to_string().as_bytes());
                email.to = Some(recipient);
            },
            Ok(Command::Data) => {
                let mut body: Vec<String> = Vec::new();
                let mut body_string = String::new();
                buf.write(ResponseCode::StartMailInput.to_string().as_bytes());
                buf.flush();
                'data: loop {
                    buf.read_line(&mut body_string);
                    println!("{:?}", body_string);
                    if body_string == ".\n" {
                        println!("Detected end of mail body");
                        break 'data;
                    }
                    body.push(body_string.clone());
                    buf.flush();
                    body_string.clear();
                }
                email.body = Some(body.concat());
                println!("{:?}", email);
                match store::save(&conn, &email) {
                    Ok(rows_updated) => {
                        println!("Message saved for delivery");
                        buf.write(ResponseCode::SavedForDelivery.to_string().as_bytes());
                    },
                    Err(err) => {
                        println!("Error saving message: {}", err);
                        buf.write(ResponseCode::TransactionFailed.to_string().as_bytes());
                    }
                };
            },
            Ok(Command::Terminate) => break,
            Err(response_code) => {
                buf.write(&response_code.to_string().as_bytes());
                println!("Command not recognised: {:?}", request_string);
            }
        }
        buf.flush();
        request_string.clear();
    };
}

fn parse_request(req: &str) -> Result<Command, ResponseCode> {
    // The command should be a 4 character string like HELO or MAIL
    // It is followed by a space, then any data
    if req.len() == 0 { return Ok(Command::Terminate) }
    if req.len() < 4 { return Err(ResponseCode::CommandUnrecognised) };
    let command_str = &req[0..4];
    match command_str {
        "HELO" => Ok(Command::Helo(req[5..].to_string())),
        "MAIL" => {
            let addr = parse_from_address(&req[5..]);
            match addr {
                Some(address) => Ok(Command::Mail(address)),
                None          => Err(ResponseCode::ArgumentError)
            }
        },
        "RCPT" => {
            let addr = parse_to_address(&req[5..]);
            match addr {
                Some(address) => Ok(Command::Rcpt(address)),
                None          => Err(ResponseCode::ArgumentError)
            }
        },
        "DATA" => Ok(Command::Data),
        _      => Err(ResponseCode::CommandUnrecognised)
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

pub type EmailAddress = String;
type ServerAddress  = String;

enum Command {
    Helo(ServerAddress), // contains address of connecting server
    Mail(EmailAddress),  // contains from address
    Rcpt(EmailAddress),  // contains to address
    Data,                // mail message to follow
    Terminate            // client has disconnected
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
        self.write(data.as_bytes());
        //self.write("\n".as_bytes());
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
        self.write(response.as_bytes());
        self.write("\n".as_bytes());
    }
}
