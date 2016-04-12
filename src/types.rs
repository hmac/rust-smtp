use std::fmt;
use std::str::FromStr;
use nom::IResult;
use parser;

pub type ServerAddress  = String;

#[derive(Debug,PartialEq)]
pub enum Command {
    Helo(ServerAddress), // contains address of connecting server
    Mail(EmailAddress),  // contains from address
    Rcpt(EmailAddress),  // contains to address
    Data,                // mail message to follow
    Terminate            // client has disconnected
}

#[derive(Debug,PartialEq,Clone)]
pub struct EmailAddress {
    pub local: String,
    pub domain: String
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.local, self.domain)
    }
}

pub type ParseEmailAddressError = usize;
impl FromStr for EmailAddress {
    type Err = ParseEmailAddressError;
    fn from_str(s: &str) -> Result<EmailAddress, ParseEmailAddressError> {
        match parser::email_address(s.as_bytes()) {
            IResult::Done(rest, addr) => Ok(addr),
            _ => Err(1)
        }
    }
}

#[test]
fn test_from_str() {
    assert_eq!(
        EmailAddress::from_str(&"bob@gnu.org"),
        Ok(EmailAddress { local: "bob".to_string(), domain: "gnu.org".to_string() })
    );
}

#[derive(Debug)]
pub struct Email {
    pub to: EmailAddress,
    pub from: EmailAddress,
    pub body: String
}

#[derive(Debug)]
pub struct InboundEmail {
    pub id: i64,
    pub email: Email
}
